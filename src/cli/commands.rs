use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "claude-vault")]
#[command(about = "Secure credential management for Claude API", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new profile
    Add {
        /// Profile name
        name: String,

        /// Profile description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// List all profiles
    List,

    /// Show profile details
    Show {
        /// Profile name
        name: String,
    },

    /// Remove a profile
    Remove {
        /// Profile name
        name: String,

        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Set default profile
    Default {
        /// Profile name
        name: String,
    },

    /// Detect profile for current directory
    Detect,

    /// Initialize project with a profile
    Init {
        /// Profile name
        name: String,
    },

    /// Execute command with profile credentials
    Exec {
        /// Profile name (optional, uses detected/default profile)
        #[arg(short, long)]
        profile: Option<String>,

        /// Command to execute
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
        command: Vec<String>,
    },

    /// Print environment variables for shell integration
    Env {
        /// Profile name (optional, uses detected/default profile)
        #[arg(short, long)]
        profile: Option<String>,
    },
}
