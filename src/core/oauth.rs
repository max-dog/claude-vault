use crate::core::{config, keychain, ProfileManager};
use crate::error::{Error, Result};
use crate::types::CredentialType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const ANTHROPIC_TOKEN_ENDPOINT: &str = "https://api.anthropic.com/v1/oauth/token";

#[derive(Debug, Serialize)]
struct RefreshTokenRequest {
    grant_type: String,
    refresh_token: String,
}

#[derive(Debug, Deserialize)]
struct RefreshTokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<i64>,
}

/// Refresh an OAuth token using the refresh token
pub fn refresh_oauth_token(profile_name: &str) -> Result<()> {
    // Get refresh token from keychain
    let refresh_token = keychain::get_refresh_token(profile_name)?;

    // Prepare request
    let request = RefreshTokenRequest {
        grant_type: "refresh_token".to_string(),
        refresh_token: refresh_token.clone(),
    };

    // Call Anthropic token endpoint
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(ANTHROPIC_TOKEN_ENDPOINT)
        .json(&request)
        .send()
        .map_err(|e| Error::ConfigError(format!("Failed to refresh token: {}", e)))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(Error::ConfigError(format!(
            "Token refresh failed ({}): {}",
            status, body
        )));
    }

    let token_response: RefreshTokenResponse = response
        .json()
        .map_err(|e| Error::ConfigError(format!("Failed to parse refresh response: {}", e)))?;

    // Calculate new expiration time
    let expires_at = if let Some(expires_in) = token_response.expires_in {
        let now = Utc::now();
        Some(now + chrono::Duration::seconds(expires_in))
    } else {
        None
    };

    // Update profile with new tokens
    let mut config = config::load()?;
    if let Some(profile) = config.find_profile_mut(profile_name) {
        profile.expires_at = expires_at;
        profile.touch();
    }
    config::save(&config)?;

    // Store new access token
    keychain::store_oauth(profile_name, &token_response.access_token)?;

    // Store new refresh token if provided
    if let Some(new_refresh_token) = token_response.refresh_token {
        keychain::store_refresh_token(profile_name, &new_refresh_token)?;
    }

    Ok(())
}

/// Check if token is expired and refresh if needed
pub fn ensure_token_valid(profile_name: &str) -> Result<()> {
    let profile = ProfileManager::get(profile_name)?;

    // Only handle OAuth profiles
    if profile.credential_type != CredentialType::OAuth {
        return Ok(());
    }

    // Check if token is expired
    if !profile.is_expired() {
        return Ok(());
    }

    // Try to refresh the token
    eprintln!("🔄 Token expired. Attempting automatic refresh...");
    match refresh_oauth_token(profile_name) {
        Ok(()) => {
            eprintln!("✓ Token refreshed successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("✗ Failed to refresh token: {}", e);
            eprintln!("   Please re-login to Claude Code and re-import:");
            eprintln!("   claude /login");
            eprintln!("   claude-vault import oauth --profile {}", profile_name);
            Err(Error::ConfigError("Token refresh failed".to_string()))
        }
    }
}
