use std::{ffi::OsStr, process::Command};

use crate::{Invocation, PaletteItem};

#[derive(Debug)]
pub struct ExecutionError {
    pub message: String,
}

pub fn execute(item: &PaletteItem, herdr_bin: &OsStr) -> Result<(), ExecutionError> {
    let Invocation::Herdr(argv) = &item.invocation else {
        return Err(ExecutionError {
            message: "This action is available through its displayed shortcut.".into(),
        });
    };

    let output = Command::new(herdr_bin)
        .args(argv)
        .output()
        .map_err(|error| ExecutionError {
            message: format!("Could not run Herdr: {error}"),
        })?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    let status = output
        .status
        .code()
        .map_or_else(|| "signal".to_owned(), |code| code.to_string());
    let message = if stderr.is_empty() {
        format!("Herdr exited with status {status}.")
    } else {
        format!("Herdr exited with status {status}: {stderr}")
    };
    Err(ExecutionError { message })
}
