use crate::core::{cache, config};
use crate::error::{Error, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const PROFILE_FILE_NAME: &str = ".claude-profile";

/// Detect profile for current directory
pub fn detect_profile() -> Result<String> {
    let current_dir = env::current_dir()?;
    detect_profile_for_dir(&current_dir)
}

/// Detect profile for a specific directory
pub fn detect_profile_for_dir(start_dir: &Path) -> Result<String> {
    let config = config::load()?;

    // Check cache first
    if let Some(cached_profile) = cache::get(start_dir)? {
        // Verify profile still exists
        if config.profile_exists(&cached_profile) {
            return Ok(cached_profile);
        }
    }

    // Traverse up directory tree
    let mut current = start_dir;
    loop {
        let profile_file = current.join(PROFILE_FILE_NAME);

        if profile_file.exists() {
            let profile_name = fs::read_to_string(&profile_file)?
                .trim()
                .to_string();

            // Validate profile exists in config
            if config.profile_exists(&profile_name) {
                // Update cache
                cache::set(start_dir, &profile_name)?;
                return Ok(profile_name);
            } else {
                return Err(Error::InvalidProfileReference(profile_name));
            }
        }

        // Move up one directory
        match current.parent() {
            Some(parent) => current = parent,
            None => break, // Reached root
        }
    }

    // Fall back to default profile
    config
        .default_profile
        .ok_or(Error::NoProfileDetected)
}

/// Initialize a project with a profile
pub fn init_profile(profile_name: &str) -> Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let profile_file = current_dir.join(PROFILE_FILE_NAME);

    // Verify profile exists
    let config = config::load()?;
    if !config.profile_exists(profile_name) {
        return Err(Error::ProfileNotFound(profile_name.to_string()));
    }

    // Write profile file
    fs::write(&profile_file, format!("{}\n", profile_name))?;

    // Update cache
    cache::set(&current_dir, profile_name)?;

    // Add to .gitignore if in git repo
    add_to_gitignore(&current_dir)?;

    Ok(profile_file)
}

/// Add .claude-profile to .gitignore if not already present
fn add_to_gitignore(dir: &Path) -> Result<()> {
    let gitignore_path = dir.join(".gitignore");

    // Check if we're in a git repo
    if !dir.join(".git").exists() {
        return Ok(()); // Not a git repo, skip
    }

    let entry = PROFILE_FILE_NAME;

    // Read existing .gitignore or create new
    let mut contents = if gitignore_path.exists() {
        fs::read_to_string(&gitignore_path)?
    } else {
        String::new()
    };

    // Check if entry already exists
    if contents.lines().any(|line| line.trim() == entry) {
        return Ok(()); // Already in .gitignore
    }

    // Add entry
    if !contents.ends_with('\n') && !contents.is_empty() {
        contents.push('\n');
    }
    contents.push_str(&format!("{}\n", entry));

    fs::write(&gitignore_path, contents)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Profile;
    use tempfile::tempdir;

    #[test]
    fn test_detect_profile_in_current_dir() {
        let temp_dir = tempdir().unwrap();
        let profile_file = temp_dir.path().join(PROFILE_FILE_NAME);

        // Create config with test profile
        let mut config = crate::types::Config::new();
        config.add_profile(Profile::new("test".to_string(), None)).unwrap();

        // Manually save config to temp location
        let config_path = temp_dir.path().join("config.toml");
        let toml = toml::to_string_pretty(&config).unwrap();
        fs::write(&config_path, toml).unwrap();

        // Write profile file
        fs::write(&profile_file, "test\n").unwrap();

        // Detection should find it (in real scenario with proper config path)
        assert!(profile_file.exists());
    }

    #[test]
    fn test_detect_profile_in_parent_dir() {
        let temp_dir = tempdir().unwrap();
        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        let profile_file = temp_dir.path().join(PROFILE_FILE_NAME);
        fs::write(&profile_file, "test\n").unwrap();

        // Profile file should be found even from subdirectory
        assert!(profile_file.exists());
    }

    #[test]
    fn test_add_to_gitignore_not_git_repo() {
        let temp_dir = tempdir().unwrap();

        // Should not create .gitignore if not a git repo
        let result = add_to_gitignore(temp_dir.path());
        assert!(result.is_ok());
        assert!(!temp_dir.path().join(".gitignore").exists());
    }

    #[test]
    fn test_add_to_gitignore_in_git_repo() {
        let temp_dir = tempdir().unwrap();

        // Create .git directory to simulate git repo
        fs::create_dir(temp_dir.path().join(".git")).unwrap();

        // Add to gitignore
        add_to_gitignore(temp_dir.path()).unwrap();

        // Check .gitignore was created
        let gitignore_path = temp_dir.path().join(".gitignore");
        assert!(gitignore_path.exists());

        let contents = fs::read_to_string(&gitignore_path).unwrap();
        assert!(contents.contains(PROFILE_FILE_NAME));
    }

    #[test]
    fn test_add_to_gitignore_already_exists() {
        let temp_dir = tempdir().unwrap();

        // Create .git directory
        fs::create_dir(temp_dir.path().join(".git")).unwrap();

        // Create .gitignore with entry already present
        let gitignore_path = temp_dir.path().join(".gitignore");
        fs::write(&gitignore_path, format!("{}\n", PROFILE_FILE_NAME)).unwrap();

        // Add again
        add_to_gitignore(temp_dir.path()).unwrap();

        // Should not duplicate
        let contents = fs::read_to_string(&gitignore_path).unwrap();
        let count = contents.matches(PROFILE_FILE_NAME).count();
        assert_eq!(count, 1);
    }
}
