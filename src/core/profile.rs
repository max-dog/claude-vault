use crate::core::{config, keychain};
use crate::error::Result;
use crate::types::Profile;
use crate::utils::validate_profile_name;

pub struct ProfileManager;

impl ProfileManager {
    /// Add a new profile
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

    /// Remove a profile
    pub fn remove(name: &str) -> Result<()> {
        let mut config = config::load()?;

        config.remove_profile(name)?;

        // Delete from keychain
        keychain::delete(name)?;

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
