use std::process::Command;

pub fn list_snapshots() -> String {
    let stdout = Command::new("zfs")
        .arg("list")
        .arg("-t")
        .arg("snapshot")
        .output()
        .expect("Failed to execute process")
        .stdout;
    String::from_utf8(stdout).expect("Invalid string")
}
