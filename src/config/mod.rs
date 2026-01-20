use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColorMode {
    Color,
    Grayscale,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DisplayMode {
    Pixelated,
    FullResolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub username: Option<String>,
    pub color_mode: ColorMode,
    pub display_mode: DisplayMode,
    /// Optional TMDB API key (users can get free keys at https://www.themoviedb.org/settings/api)
    #[serde(default)]
    pub tmdb_api_key: Option<String>,
    /// Optional OMDB API key (users can get free keys at https://www.omdbapi.com/apikey.aspx)
    #[serde(default)]
    pub omdb_api_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: None,
            color_mode: ColorMode::Color,
            display_mode: DisplayMode::Pixelated,
            tmdb_api_key: None,
            omdb_api_key: None,
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
        let config: Config = serde_json::from_str(&content).unwrap_or_else(|_| Config::default());

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

    pub fn set_display_mode(&self, mode: DisplayMode) -> Result<()> {
        let mut config = self.load_config()?;
        config.display_mode = mode;
        self.save_config(&config)
    }

    pub fn get_display_mode(&self) -> Result<DisplayMode> {
        let config = self.load_config()?;
        Ok(config.display_mode)
    }

    pub fn set_color_mode(&self, mode: ColorMode) -> Result<()> {
        let mut config = self.load_config()?;
        config.color_mode = mode;
        self.save_config(&config)
    }

    pub fn get_color_mode(&self) -> Result<ColorMode> {
        let config = self.load_config()?;
        Ok(config.color_mode)
    }

    pub fn change_username(&self, new_username: String) -> Result<()> {
        let mut config = self.load_config()?;
        config.username = Some(new_username);
        self.save_config(&config)
    }

    pub fn get_all_config(&self) -> Result<Config> {
        self.load_config()
    }

    /// Get TMDB API key from config (if set)
    pub fn get_tmdb_api_key(&self) -> Option<String> {
        self.load_config().ok()?.tmdb_api_key
    }

    /// Set TMDB API key in config
    pub fn set_tmdb_api_key(&self, api_key: String) -> Result<()> {
        let mut config = self.load_config()?;
        config.tmdb_api_key = Some(api_key);
        self.save_config(&config)
    }

    /// Get OMDB API key from config (if set)
    pub fn get_omdb_api_key(&self) -> Option<String> {
        self.load_config().ok()?.omdb_api_key
    }

    /// Set OMDB API key in config
    pub fn set_omdb_api_key(&self, api_key: String) -> Result<()> {
        let mut config = self.load_config()?;
        config.omdb_api_key = Some(api_key);
        self.save_config(&config)
    }

    fn get_config_dir() -> Result<PathBuf> {
        let home_dir =
            dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

        Ok(home_dir.join(".config").join("lbxd"))
    }
}
