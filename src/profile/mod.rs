use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteFilm {
    pub title: String,
    pub year: Option<u16>,
    pub poster_url: Option<String>,
    pub letterboxd_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedMovie {
    pub title: String,
    pub year: Option<u16>,
    pub director: Option<String>,
    pub genres: Vec<String>,
    pub runtime: Option<u16>, // in minutes
    pub poster_url: Option<String>,
    pub letterboxd_url: String,
    pub tmdb_url: Option<String>,
    pub cast: Vec<String>,
    pub synopsis: Option<String>,
    pub letterboxd_rating: Option<f32>, // Average Letterboxd rating
    // OMDB data
    pub imdb_rating: Option<f32>,
    pub rotten_tomatoes_rating: Option<u8>,
    pub metacritic_rating: Option<u8>,
    pub imdb_id: Option<String>,
    pub release_date: Option<String>,
    pub plot: Option<String>,
    pub awards: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMovieEntry {
    pub movie: DetailedMovie,
    pub user_rating: Option<f32>,
    pub review: Option<String>,
    pub watched_date: Option<chrono::DateTime<chrono::Utc>>,
    pub liked: bool,
    pub rewatched: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserList {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub movies: Vec<DetailedMovie>,
    pub is_public: bool,
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveProfile {
    pub name: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub total_films: u32,
    pub films_this_year: u32,
    pub lists_count: u32,
    pub following_count: u32,
    pub followers_count: u32,
    pub favorite_films: Vec<FavoriteFilm>,
    pub recent_activity: Vec<UserMovieEntry>,
    pub all_movies: Vec<UserMovieEntry>, // Complete film diary
    pub watchlist: Vec<DetailedMovie>,   // User's watchlist
    pub lists: Vec<UserList>,
    pub member_since: Option<String>,
    pub enhanced_stats: Option<EnhancedStatistics>,
    // Pagination support
    pub movies_loaded: usize,
    pub total_movies_available: usize,
    pub watchlist_loaded: usize,
    pub total_watchlist_available: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileStats {
    pub name: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub total_films: u32,
    pub films_this_year: u32,
    pub lists_count: u32,
    pub following_count: u32,
    pub followers_count: u32,
    pub favorite_films: Vec<FavoriteFilm>,
}

#[derive(Debug, Clone)]
pub enum LoadingStage {
    Profile,
    Diary,
    Lists,
    MovieDetails,
    Complete,
}

#[derive(Debug, Clone)]
pub struct LoadingProgress {
    pub stage: LoadingStage,
    pub current: usize,
    pub total: usize,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatistics {
    pub total_viewing_time_hours: f32,
    pub average_film_length: f32,
    pub longest_streak_days: u32,
    pub current_streak_days: u32,
    pub days_with_multiple_films: u32,
    pub unique_directors_count: u32,
    pub unique_countries_count: u32,
    pub unique_genres_count: u32,
    pub average_rating: f32,
    pub most_watched_year: Option<u16>,
    pub most_watched_decade: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreStats {
    pub name: String,
    pub count: u32,
    pub percentage: f32,
    pub average_rating: f32,
    pub emoji: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryStats {
    pub name: String,
    pub count: u32,
    pub percentage: f32,
    pub flag_emoji: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectorStats {
    pub name: String,
    pub film_count: u32,
    pub average_rating: f32,
    pub favorite_film: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyBreakdown {
    pub year: u16,
    pub film_count: u32,
    pub total_runtime: u32,
    pub average_rating: f32,
    pub top_genre: Option<String>,
    pub favorite_film: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingDistribution {
    pub rating: f32,
    pub count: u32,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewingPattern {
    pub month: u32,
    pub films_watched: u32,
    pub busiest_day: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedStatistics {
    pub basic_stats: UserStatistics,
    pub genre_breakdown: Vec<GenreStats>,
    pub country_breakdown: Vec<CountryStats>,
    pub director_stats: Vec<DirectorStats>,
    pub yearly_breakdown: Vec<YearlyBreakdown>,
    pub rating_distribution: Vec<RatingDistribution>,
    pub viewing_patterns: Vec<ViewingPattern>,
    pub data_source: String, // "premium" or "calculated" or "letterboxdpy"
}
