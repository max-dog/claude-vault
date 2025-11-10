use crate::error::{Error, Result};
use keyring::Entry;

const SERVICE_NAME: &str = "claude-vault";

/// Store API key in system keychain
pub fn store(profile: &str, api_key: &str) -> Result<()> {
    validate_api_key(api_key)?;

    let entry = Entry::new(SERVICE_NAME, profile)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    entry
        .set_password(api_key)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    Ok(())
}

/// Retrieve API key from system keychain
pub fn get(profile: &str) -> Result<String> {
    let entry = Entry::new(SERVICE_NAME, profile)
        .map_err(|e| Error::KeychainError(e.to_string()))?;

    let key = entry.get_password().map_err(|e| {
        Error::KeychainError(format!("Failed to get key for profile '{}': {}", profile, e))
    })?;

    validate_api_key(&key)?;

    Ok(key)
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
