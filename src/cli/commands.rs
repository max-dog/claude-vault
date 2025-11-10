use clap::{Parser, Subcommand, ValueEnum};

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

    /// Generate shell completion scripts
    Completion {
        /// Shell type
        shell: Shell,
    },

    /// Import OAuth token from Claude Code
    Import {
        /// Import type (currently only "oauth" supported)
        import_type: String,

        /// Profile name (optional, uses "default" if not specified)
        #[arg(short, long)]
        profile: Option<String>,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Shell {
    /// Bash shell
    Bash,
    /// Zsh shell
    Zsh,
    /// Fish shell
    Fish,
    /// PowerShell
    PowerShell,
}
