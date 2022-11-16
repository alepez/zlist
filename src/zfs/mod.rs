use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZfsError {
    #[error("Failed to execute process: {0}")]
    FailedToExecuteProcess(std::io::Error),
    #[error("Invalid output: {0}")]
    InvalidOutput(std::string::FromUtf8Error),
}

pub fn list_snapshots() -> Result<String, ZfsError> {
    let stdout = Command::new("zfs")
        .arg("list")
        .arg("-t")
        .arg("snapshot")
        .output()
        .map_err(ZfsError::FailedToExecuteProcess)?
        .stdout;
    String::from_utf8(stdout).map_err(ZfsError::InvalidOutput)
}
