use crate::error::{Error, Result};

/// Validate profile name (alphanumeric + hyphen/underscore)
pub fn validate_profile_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(Error::EmptyProfileName);
    }

    if name.len() > 64 {
        return Err(Error::ProfileNameTooLong);
    }

    let valid_chars = name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '-' || c == '_');

    if !valid_chars {
        return Err(Error::InvalidProfileName(name.to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_profile_name_valid() {
        assert!(validate_profile_name("personal").is_ok());
        assert!(validate_profile_name("work-123").is_ok());
        assert!(validate_profile_name("test_profile").is_ok());
    }

    #[test]
    fn test_validate_profile_name_empty() {
        assert!(matches!(
            validate_profile_name(""),
            Err(Error::EmptyProfileName)
        ));
    }

    #[test]
    fn test_validate_profile_name_too_long() {
        let long_name = "a".repeat(65);
        assert!(matches!(
            validate_profile_name(&long_name),
            Err(Error::ProfileNameTooLong)
        ));
    }

    #[test]
    fn test_validate_profile_name_invalid_chars() {
        assert!(matches!(
            validate_profile_name("invalid name"),
            Err(Error::InvalidProfileName(_))
        ));
        assert!(matches!(
            validate_profile_name("invalid@profile"),
            Err(Error::InvalidProfileName(_))
        ));
    }
}
