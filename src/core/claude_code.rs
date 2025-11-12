use crate::core::{keychain, ProfileManager};
use crate::error::{Error, Result};
use crate::types::CredentialType;
use keyring::Entry;

const CLAUDE_CODE_SERVICE: &str = "Claude Code-credentials";

/// Backup current Claude Code keychain credentials
pub fn backup_claude_code_keychain() -> Result<Option<String>> {
    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .map_err(|_| Error::ConfigError(
            "Could not determine username for Claude Code keychain backup".to_string()
        ))?;

    let entry = Entry::new(CLAUDE_CODE_SERVICE, &username)
        .map_err(|e| Error::KeychainError(format!("Failed to access Claude Code keychain: {}", e)))?;

    // Try to get current credentials (may not exist)
    match entry.get_password() {
        Ok(credentials) => Ok(Some(credentials)),
        Err(_) => Ok(None), // No existing credentials
    }
}

/// Restore Claude Code keychain credentials from backup
pub fn restore_claude_code_keychain(backup: Option<&str>) -> Result<()> {
    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .map_err(|_| Error::ConfigError(
            "Could not determine username for Claude Code keychain restore".to_string()
        ))?;

    let entry = Entry::new(CLAUDE_CODE_SERVICE, &username)
        .map_err(|e| Error::KeychainError(format!("Failed to access Claude Code keychain: {}", e)))?;

    match backup {
        Some(credentials) => {
            // Restore previous credentials
            entry
                .set_password(credentials)
                .map_err(|e| Error::KeychainError(format!("Failed to restore Claude Code keychain: {}", e)))?;
        }
        None => {
            // No previous credentials, try to delete (ignore error if doesn't exist)
            let _ = entry.delete_password();
        }
    }

    Ok(())
}

/// Switch Claude Code keychain to use specified profile's OAuth token
pub fn switch_to_profile(profile_name: &str) -> Result<String> {
    let username = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .map_err(|_| Error::ConfigError(
            "Could not determine username for Claude Code keychain switch".to_string()
        ))?;

    // Get profile
    let profile = ProfileManager::get(profile_name)?;

    // Only support OAuth profiles
    if profile.credential_type != CredentialType::OAuth {
        return Err(Error::ConfigError(
            format!("Profile '{}' is not an OAuth profile. Claude Code integration requires OAuth tokens.", profile_name)
        ));
    }

    // Get OAuth token and refresh token
    let access_token = keychain::get_oauth(profile_name)?;
    let refresh_token = keychain::get_refresh_token(profile_name)
        .unwrap_or_else(|_| String::new()); // Optional

    // Calculate expiration in milliseconds
    let expires_at_ms = profile.expires_at
        .map(|dt| dt.timestamp_millis())
        .unwrap_or(0);

    // Get subscription type from profile description
    let subscription_type = if let Some(desc) = &profile.description {
        if desc.contains("max") {
            "max"
        } else if desc.contains("pro") {
            "pro"
        } else {
            "unknown"
        }
    } else {
        "unknown"
    };

    // Build Claude Code credentials JSON
    let credentials_json = serde_json::json!({
        "claudeAiOauth": {
            "accessToken": access_token,
            "refreshToken": refresh_token,
            "expiresAt": expires_at_ms,
            "scopes": ["user:inference", "user:profile", "user:sessions:claude_code"],
            "subscriptionType": subscription_type
        }
    });

    let credentials_str = serde_json::to_string(&credentials_json)
        .map_err(|e| Error::ConfigError(format!("Failed to serialize credentials: {}", e)))?;

    // Update Claude Code keychain
    let entry = Entry::new(CLAUDE_CODE_SERVICE, &username)
        .map_err(|e| Error::KeychainError(format!("Failed to access Claude Code keychain: {}", e)))?;

    entry
        .set_password(&credentials_str)
        .map_err(|e| Error::KeychainError(format!("Failed to update Claude Code keychain: {}", e)))?;

    Ok(credentials_str)
}

/// Execute a function with Claude Code switched to specified profile, then restore
pub fn with_profile<F, R>(profile_name: &str, f: F) -> Result<R>
where
    F: FnOnce() -> Result<R>,
{
    // Backup current Claude Code keychain
    let backup = backup_claude_code_keychain()?;

    // Switch to profile
    switch_to_profile(profile_name)?;

    // Execute function and capture result
    let result = f();

    // Always restore, regardless of success or failure
    if let Err(e) = restore_claude_code_keychain(backup.as_deref()) {
        eprintln!("⚠️  Warning: Failed to restore Claude Code keychain: {}", e);
        eprintln!("   You may need to run: claude /login");
    }

    result
}
