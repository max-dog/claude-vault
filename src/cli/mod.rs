mod commands;
mod handlers;

use crate::error::Result;
use clap::Parser;
use commands::Cli;

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    handlers::handle_command(cli)
}
