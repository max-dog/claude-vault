use crate::cli::commands::{Cli, Commands, Shell};
use crate::core::{detect_profile, init_profile, ProfileManager};
use crate::error::Result;
use clap::CommandFactory;
use clap_complete::{generate, shells};
use dialoguer::{Confirm, Password};
use std::io;
use std::process::Command;

pub fn handle_command(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Add { name, description } => handle_add(name, description),
        Commands::List => handle_list(),
        Commands::Show { name } => handle_show(name),
        Commands::Remove { name, yes } => handle_remove(name, yes),
        Commands::Default { name } => handle_default(name),
        Commands::Detect => handle_detect(),
        Commands::Init { name } => handle_init(name),
        Commands::Exec { profile, command } => handle_exec(profile, command),
        Commands::Env { profile } => handle_env(profile),
        Commands::Completion { shell } => handle_completion(shell),
        Commands::Import {
            import_type,
            profile,
        } => handle_import(import_type, profile),
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
    println!("Type: {}", profile.credential_type);

    if let Some(ref desc) = profile.description {
        println!("Description: {}", desc);
    }

    println!("Created: {}", profile.created_at.to_rfc3339());

    if let Some(last_used) = profile.last_used {
        println!("Last used: {}", last_used.to_rfc3339());
    }

    if let Some(expires_at) = profile.expires_at {
        println!("Expires: {}", expires_at.to_rfc3339());

        if profile.is_expired() {
            println!("Status: ⚠️  EXPIRED");
        } else if profile.expires_soon() {
            println!("Status: ⚠️  Expires soon (within 24 hours)");
        } else {
            println!("Status: ✓ Valid");
        }
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

fn handle_exec(profile_opt: Option<String>, command: Vec<String>) -> Result<()> {
    // Resolve profile name
    let profile_name = resolve_profile(profile_opt)?;

    // Ensure token is valid (auto-refresh if expired)
    crate::core::ensure_token_valid(&profile_name)?;

    // Get profile to check credential type and expiration
    let profile = ProfileManager::get(&profile_name)?;

    // Warn if expiring soon (within 24 hours)
    if profile.expires_soon() {
        eprintln!("⚠️  Warning: Profile '{}' credentials expire soon (within 24 hours)", profile_name);
    }

    // Get credential from keychain based on type
    let credential = crate::core::keychain::get_by_type(&profile_name, profile.credential_type)?;

    // Update last_used timestamp
    ProfileManager::update_last_used(&profile_name)?;

    // Execute command with ANTHROPIC_API_KEY environment variable
    if command.is_empty() {
        return Err(crate::error::Error::ConfigError(
            "No command specified".to_string(),
        ));
    }

    let status = Command::new(&command[0])
        .args(&command[1..])
        .env("ANTHROPIC_API_KEY", credential)
        .status()
        .map_err(|e| {
            crate::error::Error::ConfigError(format!("Failed to execute command: {}", e))
        })?;

    // Exit with the same code as the child process
    std::process::exit(status.code().unwrap_or(1));
}

fn handle_env(profile_opt: Option<String>) -> Result<()> {
    // Resolve profile name
    let profile_name = resolve_profile(profile_opt)?;

    // Ensure token is valid (auto-refresh if expired)
    crate::core::ensure_token_valid(&profile_name)?;

    // Get profile to check credential type and expiration
    let profile = ProfileManager::get(&profile_name)?;

    // Warn if expiring soon (within 24 hours)
    if profile.expires_soon() {
        eprintln!("# Warning: Profile '{}' credentials expire soon", profile_name);
    }

    // Get credential from keychain based on type
    let credential = crate::core::keychain::get_by_type(&profile_name, profile.credential_type)?;

    // Print export statement for shell integration
    println!("export ANTHROPIC_API_KEY=\"{}\"", credential);
    println!("# Profile: {} ({})", profile_name, profile.credential_type);

    Ok(())
}

/// Resolve profile name from option, detection, or default
fn resolve_profile(profile_opt: Option<String>) -> Result<String> {
    if let Some(name) = profile_opt {
        // Verify profile exists
        ProfileManager::get(&name)?;
        Ok(name)
    } else {
        // Try to detect profile, fallback to default
        detect_profile()
    }
}

fn handle_completion(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let bin_name = "claude-vault";

    match shell {
        Shell::Bash => {
            generate(shells::Bash, &mut cmd, bin_name, &mut io::stdout());
        }
        Shell::Zsh => {
            generate(shells::Zsh, &mut cmd, bin_name, &mut io::stdout());
        }
        Shell::Fish => {
            generate(shells::Fish, &mut cmd, bin_name, &mut io::stdout());
        }
        Shell::PowerShell => {
            generate(
                shells::PowerShell,
                &mut cmd,
                bin_name,
                &mut io::stdout(),
            );
        }
    }

    Ok(())
}

fn handle_import(import_type: String, profile_opt: Option<String>) -> Result<()> {
    if import_type != "oauth" {
        return Err(crate::error::Error::ConfigError(format!(
            "Unknown import type '{}'. Currently only 'oauth' is supported.",
            import_type
        )));
    }

    let profile_name = profile_opt.unwrap_or_else(|| "default".to_string());

    println!("Importing OAuth token from Claude Code...");
    println!("Profile: {}", profile_name);

    // Try to read OAuth token from Claude Code's keychain entry
    // Claude Code uses "Claude Code-credentials" as service name and username as account
    let claude_code_service = "Claude Code-credentials";

    // Get current username for keychain account
    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .map_err(|_| crate::error::Error::ConfigError(
            "Could not determine username. Please set USER or USERNAME environment variable.".to_string()
        ))?;

    let entry = keyring::Entry::new(claude_code_service, &username)
        .map_err(|e| crate::error::Error::KeychainError(format!(
            "Failed to access Claude Code keychain: {}", e
        )))?;

    let credentials_json = entry
        .get_password()
        .map_err(|e| crate::error::Error::KeychainError(format!(
            "Failed to retrieve OAuth token from Claude Code.\n\
             Make sure you are logged in to Claude Code (run: claude /login)\n\
             Error: {}", e
        )))?;

    if credentials_json.is_empty() {
        return Err(crate::error::Error::ConfigError(
            "OAuth credentials are empty. Please login to Claude Code first.".to_string(),
        ));
    }

    // Parse JSON to extract access token and expiration
    let credentials: serde_json::Value = serde_json::from_str(&credentials_json)
        .map_err(|e| crate::error::Error::ConfigError(format!(
            "Failed to parse Claude Code credentials: {}", e
        )))?;

    let oauth_token = credentials["claudeAiOauth"]["accessToken"]
        .as_str()
        .ok_or_else(|| crate::error::Error::ConfigError(
            "Could not find accessToken in Claude Code credentials.".to_string()
        ))?
        .to_string();

    // Extract refresh token
    let refresh_token = credentials["claudeAiOauth"]["refreshToken"]
        .as_str()
        .ok_or_else(|| crate::error::Error::ConfigError(
            "Could not find refreshToken in Claude Code credentials.".to_string()
        ))?
        .to_string();

    // Parse expiration time (in milliseconds since epoch)
    let expires_at = credentials["claudeAiOauth"]["expiresAt"]
        .as_i64()
        .and_then(|ms| {
            chrono::DateTime::from_timestamp(ms / 1000, ((ms % 1000) * 1_000_000) as u32)
        });

    // Extract subscription type if available
    let subscription_type = credentials["claudeAiOauth"]["subscriptionType"]
        .as_str()
        .unwrap_or("unknown");

    // Import the OAuth token
    let description = Some(format!(
        "Imported from Claude Code ({}) on {}",
        subscription_type,
        chrono::Utc::now().format("%Y-%m-%d")
    ));

    let profile = ProfileManager::add_oauth(&profile_name, description, &oauth_token, expires_at)?;

    // Store refresh token in keychain
    crate::core::keychain::store_refresh_token(&profile_name, &refresh_token)?;

    println!("✓ OAuth token imported successfully");
    println!("  Profile: {}", profile.name);
    println!("  Type: {}", profile.credential_type);
    println!("  Subscription: {}", subscription_type);
    println!("  Created: {}", profile.created_at.to_rfc3339());

    if let Some(exp) = expires_at {
        println!("  Expires: {}", exp.to_rfc3339());

        let now = chrono::Utc::now();
        let duration = exp.signed_duration_since(now);
        let days = duration.num_days();

        if days > 0 {
            println!("  Valid for: {} days", days);
        } else {
            println!("  Status: ⚠️  Token may be expired or expiring soon");
        }
    }

    println!();
    println!("Note: OAuth tokens expire periodically. You may need to re-import them.");
    println!("      Run 'claude /login' in Claude Code when your token expires.");

    Ok(())
}
