use anyhow::Result;
use clap::Parser;
use colored::*;
use std::process::exit;

mod cli;
mod config;
mod domain;
mod error;
mod handler;
mod platform;
mod ui;

fn main() {
    if let Err(e) = run() {
        let cli = cli::Cli::try_parse();
        let is_json_output = match cli {
            Ok(c) => c.json,
            Err(_) => false,
        };

        if is_json_output {
            let error_json = serde_json::json!({
                "status": "error",
                "message": e.to_string(),
            });
            eprintln!("{}", serde_json::to_string(&error_json).unwrap());
        } else {
            eprintln!("{}: {}", "Error".red().bold(), e);
        }
        exit(1);
    }
}

fn run() -> Result<()> {
    let cli = cli::Cli::parse();
    handler::handle_command(cli)
}
