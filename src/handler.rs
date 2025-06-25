use crate::{cli, config, domain, error::AppError, platform, ui};
use anyhow::{anyhow, Result};
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use colored::*;
use std::io;

pub fn handle_command(cli: cli::Cli) -> Result<()> {
    match cli.command {
        cli::Commands::Check => handle_check(&cli),
        cli::Commands::Completions { shell } => handle_completions(shell),
        _ => {
            if !platform::is_git_installed() {
                return handle_git_not_installed(&cli);
            }
            match cli.command {
                cli::Commands::Setup(args) => handle_setup(args, cli.json),
                cli::Commands::Set(args) => handle_set(args, cli.json),
                cli::Commands::Save { name } => handle_save_profile(name, cli.json),
                cli::Commands::Use { name } => handle_use_profile(name, cli.json),
                cli::Commands::List => handle_list_profiles(cli.json),
                cli::Commands::Current => handle_current_profile(cli.json),
                cli::Commands::Delete { name, force } => {
                    handle_delete_profile(name, force, cli.json)
                }
                _ => unreachable!(),
            }
        }
    }
}

fn handle_git_not_installed(cli: &cli::Cli) -> Result<()> {
    if cli.json {
        let err_json = serde_json::json!({
            "status": "error",
            "message": "Git is not installed.",
            "data": { "isGitInstalled": false }
        });
        eprintln!("{}", serde_json::to_string(&err_json)?);
        return Err(AppError::GitNotInstalled.into());
    }

    eprintln!("{}", "Git is not installed.".red().bold());
    let install_command = platform::get_install_command()?;
    println!("To install it, please run the following command:");
    println!("\n  {}\n", install_command.cyan());
    Err(AppError::GitNotInstalled.into())
}

fn handle_setup(args: cli::SetupArgs, json: bool) -> Result<()> {
    let current_config = config::get_git_config()?;
    if !json {
        let app_config = config::load_app_config()?;
        ui::print_status(&current_config, Some(&app_config));
        let should_reconfigure = if current_config.name.is_some() || current_config.email.is_some()
        {
            ui::confirm(
                "Git appears to be configured. Do you want to reconfigure it?",
                false,
            )?
        } else {
            true
        };
        if !should_reconfigure {
            println!("Configuration unchanged.");
            return Ok(());
        }
    } else if args.non_interactive {
        return Err(anyhow!("Cannot run interactive setup with --non-interactive and --json flags. Provide values via the 'set' command."));
    }

    let name = ui::prompt_for_input("Enter your Git user name:", current_config.name.as_deref())?;
    let email = ui::prompt_for_input("Enter your Git email:", current_config.email.as_deref())?;
    let signing_key = ui::prompt_for_optional_input(
        "Enter your GPG/SSH signing key (optional):",
        current_config.signing_key.as_deref(),
    )?;

    let new_config = domain::GitUserConfig {
        name: Some(name),
        email: Some(email),
        signing_key: if signing_key.is_empty() {
            None
        } else {
            Some(signing_key)
        },
    };
    config::set_git_config(&new_config)?;

    if json {
        let app_config = config::load_app_config()?;
        ui::print_json_status(&new_config, &app_config);
    } else {
        ui::print_success("Git configuration has been updated.");
        let app_config = config::load_app_config()?;
        ui::print_status(&new_config, Some(&app_config));

        if ui::confirm(
            "\nWould you like to save this configuration as a profile for future use?",
            false,
        )? {
            let profile_name = ui::prompt_for_input(
                "Enter a name for this profile (e.g., 'work', 'personal'):",
                None,
            )?;
            if !profile_name.trim().is_empty() {
                handle_save_profile(profile_name, json)?;
            } else {
                println!("{}", "Info: Profile not saved due to empty name.".yellow());
            }
        }
    }
    Ok(())
}

fn handle_check(cli: &cli::Cli) -> Result<()> {
    if !platform::is_git_installed() {
        return handle_git_not_installed(cli);
    }
    let git_config = config::get_git_config()?;
    let app_config = config::load_app_config()?;
    if cli.json {
        ui::print_json_status(&git_config, &app_config);
    } else {
        ui::print_status(&git_config, Some(&app_config));
    }
    Ok(())
}

fn handle_set(args: cli::ConfigArgs, json: bool) -> Result<()> {
    let mut config_to_set = config::get_git_config()?;

    if let Some(name) = args.name {
        config_to_set.name = Some(name);
    }
    if let Some(email) = args.email {
        config_to_set.email = Some(email);
    }
    if let Some(key) = args.signing_key {
        config_to_set.signing_key = if key.is_empty() { None } else { Some(key) };
    }

    config::set_git_config(&config_to_set)?;
    let final_config = config::get_git_config()?;
    let app_config = config::load_app_config()?;
    if json {
        ui::print_json_status(&final_config, &app_config);
    } else {
        ui::print_success("Git configuration updated successfully.");
        ui::print_status(&final_config, Some(&app_config));
    }
    Ok(())
}

fn handle_save_profile(name: String, json: bool) -> Result<()> {
    let git_config = config::get_git_config()?;
    if git_config.name.as_deref().unwrap_or("").is_empty()
        || git_config.email.as_deref().unwrap_or("").is_empty()
    {
        return Err(anyhow!(
            "Current Git config is incomplete (name or email is missing). Cannot save profile."
        ));
    }
    config::save_profile(&name, &git_config)?;
    if json {
        println!(
            "{}",
            serde_json::json!({"status": "ok", "message": format!("Profile '{}' saved.", name)})
        );
    } else {
        ui::print_success(&format!("Profile '{}' saved successfully.", name));
    }
    Ok(())
}

fn handle_use_profile(name: Option<String>, json: bool) -> Result<()> {
    let profile_name = match name {
        Some(n) => n,
        None => {
            if json {
                return Err(anyhow!(
                    "A profile name is required when using --json output."
                ));
            }
            let app_config = config::load_app_config()?;
            let mut profile_names: Vec<_> = app_config.profiles.keys().cloned().collect();
            profile_names.sort();
            ui::select_profile(&profile_names)?
        }
    };

    let new_config = config::use_profile(&profile_name)?;
    if json {
        let app_config = config::load_app_config()?;
        ui::print_json_status(&new_config, &app_config);
    } else {
        ui::print_success(&format!("Switched to profile '{}'.", profile_name));
        let app_config = config::load_app_config()?;
        ui::print_status(&new_config, Some(&app_config));
    }
    Ok(())
}

fn handle_list_profiles(json: bool) -> Result<()> {
    let app_config = config::load_app_config()?;
    if json {
        ui::print_json_profiles(&app_config);
    } else {
        ui::print_profiles(&app_config);
    }
    Ok(())
}

fn handle_current_profile(json: bool) -> Result<()> {
    let app_config = config::load_app_config()?;
    if json {
        println!(
            "{}",
            serde_json::json!({
                "status": "ok",
                "data": { "activeProfile": app_config.current_profile }
            })
        );
    } else if let Some(profile) = app_config.current_profile {
        println!("Active profile: {}", profile.cyan());
    } else {
        println!("No profile is currently active.");
    }
    Ok(())
}

fn handle_delete_profile(name: String, force: bool, json: bool) -> Result<()> {
    if !json && !force {
        let confirmation_prompt =
            format!("Are you sure you want to delete the profile '{}'?", name);
        if !ui::confirm(&confirmation_prompt, false)? {
            println!("Deletion cancelled.");
            return Ok(());
        }
    }

    let mut app_config = config::load_app_config()?;
    if app_config.profiles.remove(&name).is_some() {
        if app_config.current_profile.as_ref() == Some(&name) {
            app_config.current_profile = None;
        }
        config::save_app_config(&app_config)?;
        if json {
            println!(
                "{}",
                serde_json::json!({"status": "ok", "message": format!("Profile '{}' deleted.", name)})
            );
        } else {
            ui::print_success(&format!("Profile '{}' has been deleted.", name));
        }
    } else {
        return Err(AppError::ProfileNotFound(name).into());
    }
    Ok(())
}

fn handle_completions(shell: Shell) -> Result<()> {
    let mut cmd = cli::Cli::command();
    let bin_name = cmd.get_name().to_string();
    generate(shell, &mut cmd, bin_name, &mut io::stdout());
    Ok(())
}
