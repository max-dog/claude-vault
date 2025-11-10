use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used: Option<DateTime<Utc>>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl Profile {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            name,
            description,
            created_at: Utc::now(),
            last_used: None,
            metadata: HashMap::new(),
        }
    }

    pub fn touch(&mut self) {
        self.last_used = Some(Utc::now());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_profile: Option<String>,
    pub profiles: Vec<Profile>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            version: "1.0".to_string(),
            default_profile: None,
            profiles: Vec::new(),
        }
    }

    pub fn find_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.iter().find(|p| p.name == name)
    }

    pub fn find_profile_mut(&mut self, name: &str) -> Option<&mut Profile> {
        self.profiles.iter_mut().find(|p| p.name == name)
    }

    pub fn profile_exists(&self, name: &str) -> bool {
        self.find_profile(name).is_some()
    }

    pub fn add_profile(&mut self, profile: Profile) -> crate::error::Result<()> {
        if self.profile_exists(&profile.name) {
            return Err(crate::error::Error::ProfileAlreadyExists(profile.name));
        }
        self.profiles.push(profile);
        Ok(())
    }

    pub fn remove_profile(&mut self, name: &str) -> crate::error::Result<()> {
        let pos = self
            .profiles
            .iter()
            .position(|p| p.name == name)
            .ok_or_else(|| crate::error::Error::ProfileNotFound(name.to_string()))?;

        self.profiles.remove(pos);

        // Clear default if removing default profile
        if self.default_profile.as_deref() == Some(name) {
            self.default_profile = None;
        }

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_creation() {
        let profile = Profile::new("test".to_string(), Some("Test profile".to_string()));
        assert_eq!(profile.name, "test");
        assert_eq!(profile.description, Some("Test profile".to_string()));
        assert!(profile.last_used.is_none());
    }

    #[test]
    fn test_profile_touch() {
        let mut profile = Profile::new("test".to_string(), None);
        assert!(profile.last_used.is_none());
        profile.touch();
        assert!(profile.last_used.is_some());
    }

    #[test]
    fn test_config_add_profile() {
        let mut config = Config::new();
        let profile = Profile::new("test".to_string(), None);

        assert!(config.add_profile(profile).is_ok());
        assert_eq!(config.profiles.len(), 1);
    }

    #[test]
    fn test_config_duplicate_profile() {
        let mut config = Config::new();
        let profile1 = Profile::new("test".to_string(), None);
        let profile2 = Profile::new("test".to_string(), None);

        assert!(config.add_profile(profile1).is_ok());
        assert!(config.add_profile(profile2).is_err());
    }

    #[test]
    fn test_config_remove_profile() {
        let mut config = Config::new();
        let profile = Profile::new("test".to_string(), None);
        config.add_profile(profile).unwrap();

        assert!(config.remove_profile("test").is_ok());
        assert_eq!(config.profiles.len(), 0);
    }

    #[test]
    fn test_config_remove_default_profile() {
        let mut config = Config::new();
        let profile = Profile::new("test".to_string(), None);
        config.add_profile(profile).unwrap();
        config.default_profile = Some("test".to_string());

        config.remove_profile("test").unwrap();
        assert!(config.default_profile.is_none());
    }
}
