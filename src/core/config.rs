use crate::error::{Error, Result};
use crate::types::Config;
use std::fs;
use std::path::{Path, PathBuf};

/// Get the path to the config file
pub fn get_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| Error::ConfigError("Home directory not found".into()))?;

    Ok(home.join(".claude-vault").join("config.toml"))
}

/// Get the base directory for claude-vault
pub fn get_vault_dir() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| Error::ConfigError("Home directory not found".into()))?;

    Ok(home.join(".claude-vault"))
}

/// Load config from disk, creating new if doesn't exist
pub fn load() -> Result<Config> {
    let path = get_config_path()?;

    if !path.exists() {
        return Ok(Config::new());
    }

    let contents = fs::read_to_string(&path)
        .map_err(|e| Error::ConfigError(format!("Failed to read config: {}", e)))?;

    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}

/// Save config to disk atomically
pub fn save(config: &Config) -> Result<()> {
    let path = get_config_path()?;

    // Create directory if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let toml = toml::to_string_pretty(config)?;

    // Write atomically via temp file
    let temp_path = path.with_extension("tmp");
    fs::write(&temp_path, toml)?;
    fs::rename(&temp_path, &path)?;

    // Set permissions (Unix only)
    set_file_permissions(&path)?;

    Ok(())
}

/// Set restrictive file permissions
#[cfg(unix)]
fn set_file_permissions(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let perms = fs::Permissions::from_mode(0o600);
    fs::set_permissions(path, perms)?;
    Ok(())
}

#[cfg(not(unix))]
fn set_file_permissions(_path: &Path) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_load_save_roundtrip() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join(".claude-vault").join("config.toml");

        // Create directory
        std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();

        let mut config = Config::new();
        config.default_profile = Some("test".to_string());

        // Manually save to test path
        let toml = toml::to_string_pretty(&config).unwrap();
        std::fs::write(&config_path, toml).unwrap();

        // Read back
        let contents = std::fs::read_to_string(&config_path).unwrap();
        let loaded: Config = toml::from_str(&contents).unwrap();

        assert_eq!(loaded.default_profile, Some("test".to_string()));
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::new();
        let toml = toml::to_string_pretty(&config).unwrap();
        let loaded: Config = toml::from_str(&toml).unwrap();

        assert_eq!(loaded.version, "1.0");
        assert_eq!(loaded.profiles.len(), 0);
        assert!(loaded.default_profile.is_none());
    }
}
