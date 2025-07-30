use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: Option<i32>,
    pub director: Option<String>,
    pub letterboxd_url: String,
    pub poster_url: Option<String>,
    pub tmdb_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntry {
    pub movie: Movie,
    pub rating: Option<f32>,
    pub review: Option<String>,
    pub watched_date: Option<DateTime<Utc>>,
    pub entry_type: EntryType,
    pub liked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryType {
    Watch,
    Review,
    Like,
    List,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub rss_url: String,
    pub entries: Vec<UserEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewingSummary {
    pub username: String,
    pub year: i32,
    pub total_movies: usize,
    pub total_reviews: usize,
    pub average_rating: Option<f32>,
    pub top_movies: Vec<(Movie, f32)>,
    pub favorite_directors: Vec<(String, usize)>,
    pub months_breakdown: Vec<(String, usize)>,
}