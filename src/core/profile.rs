use crate::core::{config, keychain};
use crate::error::Result;
use crate::types::{CredentialType, Profile};
use crate::utils::validate_profile_name;
use chrono::{DateTime, Utc};

pub struct ProfileManager;

impl ProfileManager {
    /// Add a new profile with API key
    pub fn add(name: &str, description: Option<String>, api_key: &str) -> Result<Profile> {
        validate_profile_name(name)?;

        let mut config = config::load()?;

        let profile = Profile::new(name.to_string(), description);

        config.add_profile(profile.clone())?;

        // Store API key in keychain
        keychain::store(name, api_key)?;

        // Save config
        config::save(&config)?;

        Ok(profile)
    }

    /// Add a new profile with OAuth token (or update if exists)
    pub fn add_oauth(
        name: &str,
        description: Option<String>,
        oauth_token: &str,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<Profile> {
        validate_profile_name(name)?;

        let mut config = config::load()?;

        // Check if profile already exists
        if config.profile_exists(name) {
            // Update existing profile
            if let Some(existing) = config.find_profile_mut(name) {
                existing.description = description;
                existing.credential_type = CredentialType::OAuth;
                existing.expires_at = expires_at;
                existing.touch(); // Update last_used timestamp
            }

            // Store OAuth token in keychain (overwrites existing)
            keychain::store_oauth(name, oauth_token)?;

            // Save config
            config::save(&config)?;

            // Return updated profile
            Ok(config.find_profile(name).unwrap().clone())
        } else {
            // Create new profile
            let mut profile = Profile::new_with_type(
                name.to_string(),
                description,
                CredentialType::OAuth,
            );

            profile.expires_at = expires_at;

            config.add_profile(profile.clone())?;

            // Store OAuth token in keychain
            keychain::store_oauth(name, oauth_token)?;

            // Save config
            config::save(&config)?;

            Ok(profile)
        }
    }

    /// Remove a profile
    pub fn remove(name: &str) -> Result<()> {
        let mut config = config::load()?;

        // Get profile to determine credential type
        let profile = config.find_profile(name)
            .ok_or_else(|| crate::error::Error::ProfileNotFound(name.to_string()))?
            .clone();

        config.remove_profile(name)?;

        // Delete from keychain based on credential type
        keychain::delete_by_type(name, profile.credential_type)?;

        // Save config
        config::save(&config)?;

        Ok(())
    }

    /// List all profiles
    pub fn list() -> Result<Vec<Profile>> {
        let config = config::load()?;
        Ok(config.profiles.clone())
    }

    /// Get a specific profile
    pub fn get(name: &str) -> Result<Profile> {
        let config = config::load()?;
        config
            .find_profile(name)
            .cloned()
            .ok_or_else(|| crate::error::Error::ProfileNotFound(name.to_string()))
    }

    /// Set default profile
    pub fn set_default(name: &str) -> Result<()> {
        let mut config = config::load()?;

        // Verify profile exists
        if !config.profile_exists(name) {
            return Err(crate::error::Error::ProfileNotFound(name.to_string()));
        }

        config.default_profile = Some(name.to_string());
        config::save(&config)?;

        Ok(())
    }

    /// Get API key for profile
    pub fn get_api_key(name: &str) -> Result<String> {
        // Verify profile exists in config
        let config = config::load()?;
        if !config.profile_exists(name) {
            return Err(crate::error::Error::ProfileNotFound(name.to_string()));
        }

        keychain::get(name)
    }

    /// Update last_used timestamp for profile
    pub fn update_last_used(name: &str) -> Result<()> {
        let mut config = config::load()?;

        // Find and update profile
        let profile = config
            .find_profile_mut(name)
            .ok_or_else(|| crate::error::Error::ProfileNotFound(name.to_string()))?;

        profile.touch();

        config::save(&config)?;
        Ok(())
    }
}
