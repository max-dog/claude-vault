use crate::cli::commands::{Cli, Commands};
use crate::core::{detect_profile, init_profile, ProfileManager};
use crate::error::Result;
use dialoguer::{Confirm, Password};

pub fn handle_command(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Add { name, description } => handle_add(name, description),
        Commands::List => handle_list(),
        Commands::Show { name } => handle_show(name),
        Commands::Remove { name, yes } => handle_remove(name, yes),
        Commands::Default { name } => handle_default(name),
        Commands::Detect => handle_detect(),
        Commands::Init { name } => handle_init(name),
    }
}

fn handle_add(name: String, description: Option<String>) -> Result<()> {
    println!("Adding profile '{}'", name);

    // Prompt for API key (hidden input)
    let api_key = Password::new()
        .with_prompt("Claude API key")
        .interact()
        .map_err(|e| crate::error::Error::ConfigError(format!("Failed to read input: {}", e)))?;

    let profile = ProfileManager::add(&name, description, &api_key)?;

    println!("✓ Profile '{}' added successfully", profile.name);
    if let Some(desc) = profile.description {
        println!("  Description: {}", desc);
    }
    println!("  Created: {}", profile.created_at.to_rfc3339());

    Ok(())
}

fn handle_list() -> Result<()> {
    let profiles = ProfileManager::list()?;

    if profiles.is_empty() {
        println!("No profiles found.");
        println!("Add a profile with: claude-vault add <name>");
        return Ok(());
    }

    println!("Profiles:");
    for profile in profiles {
        print!("  • {}", profile.name);
        if let Some(desc) = profile.description {
            print!(" - {}", desc);
        }
        println!();
        if let Some(last_used) = profile.last_used {
            println!("    Last used: {}", last_used.to_rfc3339());
        }
    }

    Ok(())
}

fn handle_show(name: String) -> Result<()> {
    let profile = ProfileManager::get(&name)?;

    println!("Profile: {}", profile.name);
    if let Some(desc) = profile.description {
        println!("Description: {}", desc);
    }
    println!("Created: {}", profile.created_at.to_rfc3339());
    if let Some(last_used) = profile.last_used {
        println!("Last used: {}", last_used.to_rfc3339());
    }

    Ok(())
}

fn handle_remove(name: String, yes: bool) -> Result<()> {
    // Verify profile exists
    ProfileManager::get(&name)?;

    // Confirm removal unless --yes flag
    if !yes {
        let confirmed = Confirm::new()
            .with_prompt(format!("Remove profile \"{}\"?", name))
            .interact()
            .map_err(|e| {
                crate::error::Error::ConfigError(format!("Failed to read input: {}", e))
            })?;

        if !confirmed {
            println!("Cancelled");
            return Ok(());
        }
    }

    ProfileManager::remove(&name)?;
    println!("✓ Profile '{}' removed", name);

    Ok(())
}

fn handle_default(name: String) -> Result<()> {
    ProfileManager::set_default(&name)?;
    println!("✓ Default profile set to '{}'", name);
    Ok(())
}

fn handle_detect() -> Result<()> {
    match detect_profile() {
        Ok(profile) => {
            println!("Detected profile: {}", profile);
            Ok(())
        }
        Err(crate::error::Error::NoProfileDetected) => {
            println!("No profile detected.");
            println!("Suggestions:");
            println!("  - Run 'claude-vault init <profile>' to set up this project");
            println!("  - Run 'claude-vault default <profile>' to set a default profile");
            Err(crate::error::Error::NoProfileDetected)
        }
        Err(e) => Err(e),
    }
}

fn handle_init(name: String) -> Result<()> {
    let profile_file = init_profile(&name)?;

    println!("✓ Initialized project with profile '{}'", name);
    println!("  Created: {}", profile_file.display());

    // Check if .gitignore was updated
    let current_dir = std::env::current_dir()?;
    if current_dir.join(".git").exists() {
        println!("  Updated: .gitignore");
    }

    Ok(())
}
