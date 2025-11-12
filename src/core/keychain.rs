use crate::error::{Error, Result};
use crate::types::CredentialType;
use keyring::Entry;

const SERVICE_NAME: &str = "claude-vault";
const OAUTH_SERVICE_NAME: &str = "claude-vault-oauth";
const REFRESH_TOKEN_SERVICE_NAME: &str = "claude-vault-oauth-refresh";

/// Store credential in system keychain
pub fn store(profile: &str, credential: &str) -> Result<()> {
    validate_api_key(credential)?;

    let entry = Entry::new(SERVICE_NAME, profile)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    entry
        .set_password(credential)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    Ok(())
}

/// Store OAuth token in system keychain
pub fn store_oauth(profile: &str, token: &str) -> Result<()> {
    if token.is_empty() {
        return Err(Error::ConfigError("OAuth token cannot be empty".to_string()));
    }

    let entry = Entry::new(OAUTH_SERVICE_NAME, profile)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    entry
        .set_password(token)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    Ok(())
}

/// Retrieve credential from system keychain (API key)
pub fn get(profile: &str) -> Result<String> {
    let entry = Entry::new(SERVICE_NAME, profile)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    let key = entry.get_password().map_err(|e| {
        Error::KeychainError(format!("Failed to get key for profile '{}': {}", profile, e))
    })?;

    validate_api_key(&key)?;

    Ok(key)
}

/// Retrieve OAuth token from system keychain
pub fn get_oauth(profile: &str) -> Result<String> {
    let entry = Entry::new(OAUTH_SERVICE_NAME, profile)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    let token = entry.get_password().map_err(|e| {
        Error::KeychainError(format!("Failed to get OAuth token for profile '{}': {}", profile, e))
    })?;

    if token.is_empty() {
        return Err(Error::KeychainError("OAuth token is empty".to_string()));
    }

    Ok(token)
}

/// Retrieve credential based on type
pub fn get_by_type(profile: &str, cred_type: CredentialType) -> Result<String> {
    match cred_type {
        CredentialType::ApiKey => get(profile),
        CredentialType::OAuth => get_oauth(profile),
    }
}

/// Delete API key from system keychain
pub fn delete(profile: &str) -> Result<()> {
    let entry = Entry::new(SERVICE_NAME, profile)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    entry
        .delete_password()
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    Ok(())
}

/// Delete OAuth token from system keychain
pub fn delete_oauth(profile: &str) -> Result<()> {
    let entry = Entry::new(OAUTH_SERVICE_NAME, profile)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    entry
        .delete_password()
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    Ok(())
}

/// Delete credential based on type
pub fn delete_by_type(profile: &str, cred_type: CredentialType) -> Result<()> {
    match cred_type {
        CredentialType::ApiKey => delete(profile),
        CredentialType::OAuth => {
            delete_oauth(profile)?;
            // Also try to delete refresh token (ignore error if not exists)
            let _ = delete_refresh_token(profile);
            Ok(())
        }
    }
}

/// Store refresh token in system keychain
pub fn store_refresh_token(profile: &str, token: &str) -> Result<()> {
    if token.is_empty() {
        return Err(Error::ConfigError("Refresh token cannot be empty".to_string()));
    }

    let entry = Entry::new(REFRESH_TOKEN_SERVICE_NAME, profile)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    entry
        .set_password(token)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    Ok(())
}

/// Retrieve refresh token from system keychain
pub fn get_refresh_token(profile: &str) -> Result<String> {
    let entry = Entry::new(REFRESH_TOKEN_SERVICE_NAME, profile)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    let token = entry.get_password().map_err(|e| {
        Error::KeychainError(format!("Failed to get refresh token for profile '{}': {}", profile, e))
    })?;

    if token.is_empty() {
        return Err(Error::KeychainError("Refresh token is empty".to_string()));
    }

    Ok(token)
}

/// Delete refresh token from system keychain
pub fn delete_refresh_token(profile: &str) -> Result<()> {
    let entry = Entry::new(REFRESH_TOKEN_SERVICE_NAME, profile)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    entry
        .delete_password()
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    Ok(())
}

/// Validate Claude API key format
fn validate_api_key(key: &str) -> Result<()> {
    if !key.starts_with("sk-ant-") {
        return Err(Error::InvalidApiKey);
    }

    if key.len() < 20 {
        return Err(Error::InvalidApiKey);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_api_key_valid() {
        assert!(validate_api_key("sk-ant-1234567890abcdefghij").is_ok());
    }

    #[test]
    fn test_validate_api_key_invalid_prefix() {
        assert!(matches!(
            validate_api_key("invalid-key"),
            Err(Error::InvalidApiKey)
        ));
    }

    #[test]
    fn test_validate_api_key_too_short() {
        assert!(matches!(
            validate_api_key("sk-ant-short"),
            Err(Error::InvalidApiKey)
        ));
    }
}
