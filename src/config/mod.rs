use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub username: Option<String>,
    pub use_pixelated_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: None,
            use_pixelated_mode: true,
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_dir = Self::get_config_dir()?;
        fs::create_dir_all(&config_dir)?;
        
        let config_path = config_dir.join("config.json");
        
        Ok(Self { config_path })
    }

    pub fn load_config(&self) -> Result<Config> {
        if !self.config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&self.config_path)?;
        let config: Config = serde_json::from_str(&content)
            .unwrap_or_else(|_| Config::default());
        
        Ok(config)
    }

    pub fn save_config(&self, config: &Config) -> Result<()> {
        let content = serde_json::to_string_pretty(config)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }

    pub fn set_username(&self, username: String) -> Result<()> {
        let mut config = self.load_config()?;
        config.username = Some(username);
        self.save_config(&config)
    }

    pub fn get_username(&self) -> Result<Option<String>> {
        let config = self.load_config()?;
        Ok(config.username)
    }

    pub fn is_first_run(&self) -> bool {
        // First run if config doesn't exist or username is not set
        self.load_config()
            .map(|c| c.username.is_none())
            .unwrap_or(true)
    }

    pub fn set_pixelated_mode(&self, use_pixelated: bool) -> Result<()> {
        let mut config = self.load_config()?;
        config.use_pixelated_mode = use_pixelated;
        self.save_config(&config)
    }

    pub fn get_pixelated_mode(&self) -> Result<bool> {
        let config = self.load_config()?;
        Ok(config.use_pixelated_mode)
    }

    pub fn change_username(&self, new_username: String) -> Result<()> {
        let mut config = self.load_config()?;
        config.username = Some(new_username);
        self.save_config(&config)
    }

    pub fn get_all_config(&self) -> Result<Config> {
        self.load_config()
    }

    fn get_config_dir() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        
        Ok(home_dir.join(".config").join("lbxd"))
    }
}