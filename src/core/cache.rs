use crate::core::config;
use crate::error::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const CACHE_FILE_NAME: &str = "cache.json";
const DEFAULT_TTL_SECONDS: i64 = 3600; // 1 hour

#[derive(Debug, Serialize, Deserialize)]
struct CacheEntry {
    profile: String,
    cached_at: DateTime<Utc>,
    ttl_seconds: i64,
}

impl CacheEntry {
    fn new(profile: String, ttl_seconds: i64) -> Self {
        Self {
            profile,
            cached_at: Utc::now(),
            ttl_seconds,
        }
    }

    fn is_expired(&self) -> bool {
        let expiry = self.cached_at + Duration::seconds(self.ttl_seconds);
        Utc::now() > expiry
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Cache {
    entries: HashMap<String, CacheEntry>,
}

impl Cache {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    fn load() -> Result<Self> {
        let cache_path = get_cache_path()?;

        if !cache_path.exists() {
            return Ok(Self::new());
        }

        let contents = fs::read_to_string(&cache_path)?;
        let cache: Cache = serde_json::from_str(&contents)
            .unwrap_or_else(|_| Cache::new());

        Ok(cache)
    }

    fn save(&self) -> Result<()> {
        let cache_path = get_cache_path()?;

        // Create directory if needed
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(&cache_path, json)?;

        Ok(())
    }

    fn get(&self, dir: &Path) -> Option<String> {
        let key = path_to_key(dir);
        self.entries
            .get(&key)
            .filter(|entry| !entry.is_expired())
            .map(|entry| entry.profile.clone())
    }

    fn set(&mut self, dir: &Path, profile: &str) {
        let key = path_to_key(dir);
        let entry = CacheEntry::new(profile.to_string(), DEFAULT_TTL_SECONDS);
        self.entries.insert(key, entry);
    }

    fn clear_expired(&mut self) {
        self.entries.retain(|_, entry| !entry.is_expired());
    }
}

fn get_cache_path() -> Result<PathBuf> {
    let vault_dir = config::get_vault_dir()?;
    Ok(vault_dir.join(CACHE_FILE_NAME))
}

fn path_to_key(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

/// Get cached profile for directory
pub fn get(dir: &Path) -> Result<Option<String>> {
    let mut cache = Cache::load()?;
    cache.clear_expired();
    Ok(cache.get(dir))
}

/// Set cached profile for directory
pub fn set(dir: &Path, profile: &str) -> Result<()> {
    let mut cache = Cache::load()?;
    cache.set(dir, profile);
    cache.clear_expired();
    cache.save()?;
    Ok(())
}

/// Clear all cache entries
pub fn clear() -> Result<()> {
    let cache_path = get_cache_path()?;
    if cache_path.exists() {
        fs::remove_file(&cache_path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_cache_entry_expiration() {
        let entry = CacheEntry::new("test".to_string(), -1); // Already expired
        assert!(entry.is_expired());

        let entry = CacheEntry::new("test".to_string(), 3600); // Not expired
        assert!(!entry.is_expired());
    }

    #[test]
    fn test_cache_set_and_get() {
        let mut cache = Cache::new();
        let test_path = Path::new("/tmp/test");

        cache.set(test_path, "test-profile");

        let result = cache.get(test_path);
        assert_eq!(result, Some("test-profile".to_string()));
    }

    #[test]
    fn test_cache_expired_entries() {
        let mut cache = Cache::new();
        let test_path = Path::new("/tmp/test");

        // Add expired entry manually
        let key = path_to_key(test_path);
        let mut entry = CacheEntry::new("test".to_string(), 0);
        entry.cached_at = Utc::now() - Duration::seconds(10);
        cache.entries.insert(key, entry);

        // Clear expired
        cache.clear_expired();

        assert!(cache.get(test_path).is_none());
    }

    #[test]
    fn test_path_to_key() {
        let path = Path::new("/tmp/test/dir");
        let key = path_to_key(path);
        assert!(key.contains("test"));
    }
}
