use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true, help = "Output in JSON format")]
    pub json: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Guides you through the Git setup process.")]
    Setup(SetupArgs),

    #[command(
        about = "Checks the current Git installation and configuration.",
        alias = "status"
    )]
    Check,

    #[command(about = "Sets Git configuration values directly.")]
    Set(ConfigArgs),

    #[command(about = "Saves the current Git config as a new profile.")]
    Save {
        #[arg(help = "The name for the new profile.")]
        name: String,
    },

    #[command(about = "Switches the global Git config to a saved profile.")]
    Use {
        #[arg(
            help = "The name of the profile to use. If omitted, an interactive selector will be shown."
        )]
        name: Option<String>,
    },

    #[command(about = "Lists all saved profiles.", alias = "ls")]
    List,

    #[command(about = "Shows which profile is currently active.", alias = "active")]
    Current,

    #[command(about = "Deletes a saved profile.", alias = "rm")]
    Delete {
        #[arg(help = "The name of the profile to delete.")]
        name: String,
    },
}

#[derive(Parser, Debug)]
pub struct SetupArgs {
    #[arg(long, help = "Run non-interactively, fails if input is required.")]
    pub non_interactive: bool,
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    #[arg(
        short,
        long,
        env = "GITUP_USER_NAME",
        help = "The user name to configure."
    )]
    pub name: Option<String>,

    #[arg(
        short,
        long,
        env = "GITUP_USER_EMAIL",
        help = "The user email to configure."
    )]
    pub email: Option<String>,
}
