use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

// Default API key - users can override with OMDB_API_KEY environment variable
const DEFAULT_OMDB_API_KEY: &str = "ad032cc2";
const OMDB_BASE_URL: &str = "http://www.omdbapi.com/";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OMDBMovie {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(rename = "Rated")]
    pub rated: Option<String>,
    #[serde(rename = "Released")]
    pub released: Option<String>,
    #[serde(rename = "Runtime")]
    pub runtime: Option<String>,
    #[serde(rename = "Genre")]
    pub genre: Option<String>,
    #[serde(rename = "Director")]
    pub director: Option<String>,
    #[serde(rename = "Writer")]
    pub writer: Option<String>,
    #[serde(rename = "Actors")]
    pub actors: Option<String>,
    #[serde(rename = "Plot")]
    pub plot: Option<String>,
    #[serde(rename = "Language")]
    pub language: Option<String>,
    #[serde(rename = "Country")]
    pub country: Option<String>,
    #[serde(rename = "Awards")]
    pub awards: Option<String>,
    #[serde(rename = "Poster")]
    pub poster: Option<String>,
    #[serde(rename = "Ratings")]
    pub ratings: Option<Vec<OMDBRating>>,
    #[serde(rename = "Metascore")]
    pub metascore: Option<String>,
    #[serde(rename = "imdbRating")]
    pub imdb_rating: Option<String>,
    #[serde(rename = "imdbVotes")]
    pub imdb_votes: Option<String>,
    #[serde(rename = "imdbID")]
    pub imdb_id: Option<String>,
    #[serde(rename = "Type")]
    pub movie_type: Option<String>,
    #[serde(rename = "BoxOffice")]
    pub box_office: Option<String>,
    #[serde(rename = "Response")]
    pub response: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OMDBRating {
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OMDBSearchResult {
    #[serde(rename = "Search")]
    pub search: Option<Vec<OMDBSearchMovie>>,
    #[serde(rename = "totalResults")]
    pub total_results: Option<String>,
    #[serde(rename = "Response")]
    pub response: String,
    #[serde(rename = "Error")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OMDBSearchMovie {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(rename = "imdbID")]
    pub imdb_id: String,
    #[serde(rename = "Type")]
    pub movie_type: String,
    #[serde(rename = "Poster")]
    pub poster: Option<String>,
}

pub struct OMDBClient {
    client: Client,
}

impl OMDBClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    /// Get OMDB API key from environment variable or use default
    fn get_api_key() -> String {
        env::var("OMDB_API_KEY").unwrap_or_else(|_| DEFAULT_OMDB_API_KEY.to_string())
    }

    pub async fn get_movie_by_title(
        &self,
        title: &str,
        year: Option<u16>,
    ) -> Result<Option<OMDBMovie>> {
        let api_key = Self::get_api_key();
        let mut url = format!(
            "{}?apikey={}&t={}",
            OMDB_BASE_URL,
            api_key,
            urlencoding::encode(title)
        );

        if let Some(year) = year {
            url.push_str(&format!("&y={}", year));
        }

        let response = self.client.get(&url).send().await?;
        let omdb_movie: OMDBMovie = response.json().await?;

        if omdb_movie.response == "True" {
            Ok(Some(omdb_movie))
        } else {
            Ok(None)
        }
    }

    pub async fn search_movies(
        &self,
        query: &str,
        year: Option<u16>,
    ) -> Result<Vec<OMDBSearchMovie>> {
        let api_key = Self::get_api_key();
        let mut url = format!(
            "{}?apikey={}&s={}",
            OMDB_BASE_URL,
            api_key,
            urlencoding::encode(query)
        );

        if let Some(year) = year {
            url.push_str(&format!("&y={}", year));
        }

        let response = self.client.get(&url).send().await?;
        let search_result: OMDBSearchResult = response.json().await?;

        if search_result.response == "True" {
            Ok(search_result.search.unwrap_or_default())
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn get_movie_by_imdb_id(&self, imdb_id: &str) -> Result<Option<OMDBMovie>> {
        let api_key = Self::get_api_key();
        let url = format!("{}?apikey={}&i={}", OMDB_BASE_URL, api_key, imdb_id);

        let response = self.client.get(&url).send().await?;
        let omdb_movie: OMDBMovie = response.json().await?;

        if omdb_movie.response == "True" {
            Ok(Some(omdb_movie))
        } else {
            Ok(None)
        }
    }

    // Helper methods to extract specific ratings
    pub fn get_imdb_rating(&self, movie: &OMDBMovie) -> Option<f32> {
        movie
            .imdb_rating
            .as_ref()
            .and_then(|rating| rating.parse::<f32>().ok())
    }

    pub fn get_rotten_tomatoes_rating(&self, movie: &OMDBMovie) -> Option<u8> {
        movie
            .ratings
            .as_ref()?
            .iter()
            .find(|rating| rating.source == "Rotten Tomatoes")?
            .value
            .trim_end_matches('%')
            .parse::<u8>()
            .ok()
    }

    pub fn get_metacritic_rating(&self, movie: &OMDBMovie) -> Option<u8> {
        movie
            .ratings
            .as_ref()?
            .iter()
            .find(|rating| rating.source == "Metacritic")?
            .value
            .split('/')
            .next()?
            .parse::<u8>()
            .ok()
    }
}

impl Default for OMDBClient {
    fn default() -> Self {
        Self::new()
    }
}
