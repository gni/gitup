use crate::domain::{AppConfig, GitUserConfig};
use crate::error::AppError;
use crate::platform;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn get_app_config_path() -> Result<PathBuf, AppError> {
    let home_dir = dirs::home_dir().ok_or(AppError::HomeDirectoryNotFound)?;
    Ok(home_dir.join(".config").join("gitup").join("config.json"))
}

pub fn load_app_config() -> Result<AppConfig, AppError> {
    let path = get_app_config_path()?;
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let content = fs::read_to_string(path).map_err(AppError::GlobalConfigError)?;
    serde_json::from_str(&content).map_err(AppError::SerializationError)
}

pub fn save_app_config(config: &AppConfig) -> Result<(), AppError> {
    let path = get_app_config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(AppError::GlobalConfigError)?;
    }
    let content = serde_json::to_string_pretty(config).map_err(AppError::SerializationError)?;
    fs::write(path, content).map_err(AppError::GlobalConfigError)
}

pub fn get_git_config() -> Result<GitUserConfig, AppError> {
    let name = platform::run_command("git", &["config", "--global", "user.name"]).ok();
    let email = platform::run_command("git", &["config", "--global", "user.email"]).ok();
    let signing_key = platform::run_command("git", &["config", "--global", "user.signingkey"]).ok();
    Ok(GitUserConfig {
        name,
        email,
        signing_key,
    })
}

pub fn set_git_config(config: &GitUserConfig) -> Result<(), AppError> {
    if let Some(name) = &config.name {
        platform::run_command("git", &["config", "--global", "user.name", name])?;
    }
    if let Some(email) = &config.email {
        platform::run_command("git", &["config", "--global", "user.email", email])?;
    }

    match &config.signing_key {
        Some(key) if !key.is_empty() => {
            platform::run_command("git", &["config", "--global", "user.signingkey", key])?;
            platform::run_command("git", &["config", "--global", "commit.gpgsign", "true"])?;
        }
        _ => {
            platform::run_command(
                "git",
                &["config", "--global", "--unset-all", "user.signingkey"],
            )
            .ok();
            platform::run_command("git", &["config", "--global", "commit.gpgsign", "false"])?;
        }
    }

    Ok(())
}

pub fn save_profile(name: &str, config: &GitUserConfig) -> Result<(), AppError> {
    let mut app_config = load_app_config()?;
    if app_config.profiles.is_empty() {
        app_config.profiles = HashMap::new();
    }
    app_config.profiles.insert(name.to_string(), config.clone());
    save_app_config(&app_config)
}

pub fn use_profile(name: &str) -> Result<GitUserConfig, AppError> {
    let mut app_config = load_app_config()?;
    let git_config = app_config
        .profiles
        .get(name)
        .ok_or_else(|| AppError::ProfileNotFound(name.to_string()))?
        .clone();
    set_git_config(&git_config)?;
    app_config.current_profile = Some(name.to_string());
    save_app_config(&app_config)?;
    Ok(git_config)
}
