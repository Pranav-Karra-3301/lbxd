use crate::models::UserProfile;
use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde_json;
use std::fs;
use std::path::PathBuf;

pub struct CacheManager {
    cache_dir: PathBuf,
}

impl CacheManager {
    pub fn new() -> Result<Self> {
        let cache_dir = Self::get_cache_dir()?;
        fs::create_dir_all(&cache_dir)?;

        Ok(Self { cache_dir })
    }

    pub fn get_cached_profile(&self, username: &str) -> Option<UserProfile> {
        let cache_file = self.cache_dir.join(format!("{username}.json"));

        if !cache_file.exists() {
            return None;
        }

        let metadata = fs::metadata(&cache_file).ok()?;
        let modified = metadata.modified().ok()?;
        let modified_dt: DateTime<Utc> = modified.into();

        if Utc::now() - modified_dt > Duration::hours(6) {
            return None;
        }

        let content = fs::read_to_string(&cache_file).ok()?;
        serde_json::from_str(&content).ok()
    }

    pub fn cache_profile(&self, profile: &UserProfile) -> Result<()> {
        let cache_file = self.cache_dir.join(format!("{}.json", profile.username));
        let content = serde_json::to_string_pretty(profile)?;
        fs::write(cache_file, content)?;
        Ok(())
    }

    pub fn clear_cache(&self) -> Result<()> {
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            if entry.path().extension() == Some("json".as_ref()) {
                fs::remove_file(entry.path())?;
            }
        }
        Ok(())
    }

    fn get_cache_dir() -> Result<PathBuf> {
        let home_dir =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

        Ok(home_dir.join(".cache").join("lbxd"))
    }
}
