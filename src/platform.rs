use crate::error::AppError;
use std::process::{Command, Stdio};

pub fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

pub fn get_install_command() -> Result<String, AppError> {
    if cfg!(target_os = "linux") {
        if Command::new("which")
            .arg("apt-get")
            .output()
            .is_ok_and(|o| o.status.success())
        {
            Ok("sudo apt-get update && sudo apt-get install git".to_string())
        } else if Command::new("which")
            .arg("dnf")
            .output()
            .is_ok_and(|o| o.status.success())
        {
            Ok("sudo dnf install git".to_string())
        } else if Command::new("which")
            .arg("yum")
            .output()
            .is_ok_and(|o| o.status.success())
        {
            Ok("sudo yum install git".to_string())
        } else if Command::new("which")
            .arg("pacman")
            .output()
            .is_ok_and(|o| o.status.success())
        {
            Ok("sudo pacman -Syu git".to_string())
        } else {
            Err(AppError::PlatformDetectionFailed)
        }
    } else if cfg!(target_os = "macos") {
        Ok("xcode-select --install".to_string())
    } else if cfg!(target_os = "windows") {
        Ok("Visit https://git-scm.com/download/win and run the installer.".to_string())
    } else {
        Err(AppError::PlatformDetectionFailed)
    }
}

pub fn run_command(cmd: &str, args: &[&str]) -> Result<String, AppError> {
    let output =
        Command::new(cmd)
            .args(args)
            .output()
            .map_err(|e| AppError::CommandExecutionError {
                command: cmd.to_string(),
                source: e,
            })?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(AppError::CommandFailed {
            command: format!("{} {}", cmd, args.join(" ")),
            code: output.status.code().unwrap_or(1),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}
