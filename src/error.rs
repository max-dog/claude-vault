use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Profile '{0}' not found")]
    ProfileNotFound(String),

    #[error("Profile '{0}' already exists")]
    ProfileAlreadyExists(String),

    #[error("Invalid profile name: {0}")]
    InvalidProfileName(String),

    #[error("Empty profile name")]
    EmptyProfileName,

    #[error("Profile name too long (max 64 characters)")]
    ProfileNameTooLong,

    #[error("Invalid API key format")]
    InvalidApiKey,

    #[error("Keychain error: {0}")]
    KeychainError(String),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TOML error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("No profile detected and no default profile set")]
    NoProfileDetected,

    #[error("Profile '{0}' in .claude-profile does not exist")]
    InvalidProfileReference(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::ProfileNotFound("test".to_string());
        assert_eq!(err.to_string(), "Profile 'test' not found");
    }

    #[test]
    fn test_error_profile_already_exists() {
        let err = Error::ProfileAlreadyExists("work".to_string());
        assert_eq!(err.to_string(), "Profile 'work' already exists");
    }
}
