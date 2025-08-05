use anyhow::Result;
use colored::*;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;

// Default API key - users can override with TMDB_API_KEY environment variable
const DEFAULT_TMDB_API_KEY: &str = "bce5788c33b687c14b610654579ff6aa";
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

    /// Get TMDB API key from environment variable or use default
    fn get_api_key() -> String {
        env::var("TMDB_API_KEY").unwrap_or_else(|_| DEFAULT_TMDB_API_KEY.to_string())
    }

    pub async fn search_movie(&self, query: &str) -> Result<Option<TMDBMovie>> {
        self.search_movie_with_year(query, None).await
    }

    pub async fn search_movie_with_year(
        &self,
        query: &str,
        year: Option<i32>,
    ) -> Result<Option<TMDBMovie>> {
        let api_key = Self::get_api_key();
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
