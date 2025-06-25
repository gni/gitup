use crate::domain::{AppConfig, GitUserConfig};
use crate::error::AppError;
use colored::*;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Confirm, Input, Select};

pub fn confirm(prompt: &str, default: bool) -> Result<bool, AppError> {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(default)
        .interact()
        .map_err(|_| AppError::OperationCancelled)
}

pub fn prompt_for_input(prompt: &str, default: Option<&str>) -> Result<String, AppError> {
    let theme = ColorfulTheme::default();
    let mut builder = Input::with_theme(&theme).with_prompt(prompt);

    if let Some(val) = default {
        builder = builder.default(val.to_string());
    }

    builder
        .interact_text()
        .map_err(|_| AppError::OperationCancelled)
}

pub fn prompt_for_optional_input(prompt: &str, default: Option<&str>) -> Result<String, AppError> {
    let theme = ColorfulTheme::default();
    let mut builder = Input::with_theme(&theme).with_prompt(prompt);

    if let Some(val) = default {
        builder = builder.default(val.to_string());
    }

    builder
        .allow_empty(true)
        .interact_text()
        .map_err(|_| AppError::OperationCancelled)
}

pub fn select_profile(profiles: &[String]) -> Result<String, AppError> {
    if profiles.is_empty() {
        return Err(AppError::ProfileNotFound(
            "No profiles exist to choose from.".to_string(),
        ));
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a profile to use")
        .items(profiles)
        .default(0)
        .interact()
        .map_err(|_| AppError::OperationCancelled)?;
    Ok(profiles[selection].clone())
}

pub fn print_status(config: &GitUserConfig, app_config: Option<&AppConfig>) {
    println!("{}", "Git Configuration Status".bold().underline());
    match &config.name {
        Some(name) if !name.is_empty() => println!("  {:<12}: {}", "Name".green(), name),
        _ => println!("  {:<12}: Not Set", "Name".yellow()),
    }
    match &config.email {
        Some(email) if !email.is_empty() => println!("  {:<12}: {}", "Email".green(), email),
        _ => println!("  {:<12}: Not Set", "Email".yellow()),
    }
    match &config.signing_key {
        Some(key) if !key.is_empty() => println!("  {:<12}: {}", "Signing Key".green(), key),
        _ => (),
    }
    if let Some(ac) = app_config {
        if let Some(profile) = &ac.current_profile {
            println!(
                "  {:<12}: {} ({})",
                "Profile".green(),
                profile,
                "active".cyan()
            );
        }
    }
}

pub fn print_profiles(config: &AppConfig) {
    println!("{}", "Saved Profiles".bold().underline());
    if config.profiles.is_empty() {
        println!("  No profiles saved.");
        return;
    }
    let mut sorted_profiles: Vec<_> = config.profiles.keys().collect();
    sorted_profiles.sort();

    for name in sorted_profiles {
        if Some(name) == config.current_profile.as_ref() {
            println!("  - {} ({})", name.bold(), "active".cyan());
        } else {
            println!("  - {}", name);
        }
    }
}

pub fn print_json_status(config: &GitUserConfig, app_config: &AppConfig) {
    let json = serde_json::json!({
        "status": "ok",
        "data": {
            "isGitInstalled": true,
            "config": config,
            "activeProfile": app_config.current_profile
        }
    });
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

pub fn print_json_profiles(config: &AppConfig) {
    let json = serde_json::json!({
        "status": "ok",
        "data": config,
    });
    println!("{}", serde_json::to_string_pretty(&json).unwrap());
}

pub fn print_success(message: &str) {
    println!("{} {}", "Success:".green().bold(), message);
}
