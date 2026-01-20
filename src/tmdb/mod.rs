use anyhow::Result;
use colored::*;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;

use crate::config::ConfigManager;

const TMDB_BASE_URL: &str = "https://api.themoviedb.org/3";
const TMDB_IMAGE_BASE_URL: &str = "https://image.tmdb.org/t/p/w780"; // Higher quality images
const TMDB_IMAGE_ORIGINAL: &str = "https://image.tmdb.org/t/p/original";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TMDBMovie {
    pub id: u32,
    pub title: String,
    pub release_date: Option<String>,
    pub poster_path: Option<String>,
    pub overview: Option<String>,
    pub vote_average: f32,
}

#[derive(Debug, Deserialize)]
struct TMDBSearchResponse {
    results: Vec<TMDBMovie>,
}

pub struct TMDBClient {
    client: reqwest::Client,
}

impl Default for TMDBClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TMDBClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self { client }
    }

    /// Get TMDB API key from environment variable or config file.
    /// Returns an error with helpful instructions if no key is configured.
    fn get_api_key() -> Result<String> {
        // Priority 1: Environment variable
        if let Ok(key) = env::var("TMDB_API_KEY") {
            if !key.is_empty() {
                return Ok(key);
            }
        }

        // Priority 2: Config file
        if let Ok(config_manager) = ConfigManager::new() {
            if let Some(key) = config_manager.get_tmdb_api_key() {
                return Ok(key);
            }
        }

        // No key found - provide helpful error message
        Err(anyhow::anyhow!(
            "TMDB API key not configured.\n\
            \n\
            To enable movie search and poster display, you need a free TMDB API key:\n\
            1. Sign up at https://www.themoviedb.org/signup\n\
            2. Go to Settings -> API and request an API key\n\
            3. Set it via environment variable: export TMDB_API_KEY=your_key_here\n\
            \n\
            Note: lbxd will work without this key, but movie search will be unavailable."
        ))
    }

    pub async fn search_movie(&self, query: &str) -> Result<Option<TMDBMovie>> {
        self.search_movie_with_year(query, None).await
    }

    pub async fn search_movie_with_year(
        &self,
        query: &str,
        year: Option<i32>,
    ) -> Result<Option<TMDBMovie>> {
        let api_key = Self::get_api_key()?;
        let mut url = format!(
            "{}/search/movie?api_key={}&query={}",
            TMDB_BASE_URL,
            api_key,
            urlencoding::encode(query)
        );

        if let Some(year) = year {
            url.push_str(&format!("&year={}", year));
        }

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "TMDB API request failed: {}",
                response.status()
            ));
        }

        let search_result: TMDBSearchResponse = response.json().await?;

        Ok(search_result.results.into_iter().next())
    }

    pub fn get_poster_url(&self, poster_path: &str) -> String {
        format!("{}{}", TMDB_IMAGE_BASE_URL, poster_path)
    }

    pub fn print_tmdb_attribution() {
        println!(
            "{}",
            "Data provided by The Movie Database (TMDB)".color("blue")
        );
    }
}

impl TMDBMovie {
    pub fn get_year(&self) -> Option<i32> {
        self.release_date
            .as_ref()
            .and_then(|date| date.split('-').next())
            .and_then(|year_str| year_str.parse().ok())
    }

    pub fn get_full_poster_url(&self) -> Option<String> {
        self.poster_path
            .as_ref()
            .map(|path| format!("{}{}", TMDB_IMAGE_BASE_URL, path))
    }

    pub fn get_high_quality_poster_url(&self) -> Option<String> {
        self.poster_path
            .as_ref()
            .map(|path| format!("{}{}", TMDB_IMAGE_BASE_URL, path))
    }

    pub fn get_original_poster_url(&self) -> Option<String> {
        self.poster_path
            .as_ref()
            .map(|path| format!("{}{}", TMDB_IMAGE_ORIGINAL, path))
    }
}
