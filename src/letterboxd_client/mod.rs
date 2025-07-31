use anyhow::Result;
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::process::Command;
use std::process::Stdio;
use chrono::Datelike;

use crate::profile::{
    ComprehensiveProfile, DetailedMovie, UserMovieEntry, FavoriteFilm,
    LoadingProgress, LoadingStage, EnhancedStatistics, UserStatistics,
    GenreStats, DirectorStats, YearlyBreakdown, RatingDistribution, ViewingPattern
};
use crate::omdb::OMDBClient;

pub struct LetterboxdClient {}

impl LetterboxdClient {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub async fn get_comprehensive_profile(
        &self,
        username: &str,
        progress_tx: Option<mpsc::UnboundedSender<LoadingProgress>>,
    ) -> Result<ComprehensiveProfile> {
        // Send initial progress
        if let Some(ref tx) = progress_tx {
            let _ = tx.send(LoadingProgress {
                stage: LoadingStage::Profile,
                current: 0,
                total: 4,
                message: "Checking letterboxdpy installation...".to_string(),
            });
        }

        // Check if letterboxdpy is installed, install if not
        self.ensure_letterboxdpy().await?;

        if let Some(ref tx) = progress_tx {
            let _ = tx.send(LoadingProgress {
                stage: LoadingStage::Diary,
                current: 1,
                total: 4,
                message: "Fetching user data from Letterboxd...".to_string(),
            });
        }

        // Get user data using Python subprocess
        let user_data = self.get_user_data(username).await?;

        if let Some(ref tx) = progress_tx {
            let _ = tx.send(LoadingProgress {
                stage: LoadingStage::Lists,
                current: 2,
                total: 5,
                message: "Fetching watchlist...".to_string(),
            });
        }

        // Get watchlist data
        let watchlist_data = self.get_watchlist_data(username).await?;

        if let Some(ref tx) = progress_tx {
            let _ = tx.send(LoadingProgress {
                stage: LoadingStage::MovieDetails,
                current: 3,
                total: 5,
                message: "Processing profile data...".to_string(),
            });
        }

        // Convert the JSON data to our Rust structures
        let mut comprehensive_profile = self.convert_user_data_to_profile(user_data, username).await?;
        
        // Add watchlist data
        comprehensive_profile.watchlist = self.convert_watchlist_to_movies(watchlist_data).await?;

        if let Some(ref tx) = progress_tx {
            let _ = tx.send(LoadingProgress {
                stage: LoadingStage::Complete,
                current: 5,
                total: 5,
                message: "Enriching with OMDB data...".to_string(),
            });
        }

        // Enrich with OMDB data
        let comprehensive_profile = self.enrich_with_omdb(comprehensive_profile).await?;

        if let Some(ref tx) = progress_tx {
            let _ = tx.send(LoadingProgress {
                stage: LoadingStage::Complete,
                current: 5,
                total: 5,
                message: "Profile loading complete!".to_string(),
            });
        }

        Ok(comprehensive_profile)
    }

    async fn ensure_letterboxdpy(&self) -> Result<()> {
        // Create a Python script to check/install letterboxdpy
        let check_script = r#"
import sys
import subprocess

try:
    import letterboxdpy
    print("letterboxdpy already installed")
except ImportError:
    print("Installing letterboxdpy...")
    subprocess.check_call([sys.executable, "-m", "pip", "install", "letterboxdpy"])
    print("letterboxdpy installed successfully")
"#;

        let child = Command::new("python3")
            .arg("-c")
            .arg(check_script)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = child.wait_with_output().await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to install letterboxdpy: {}", stderr));
        }

        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    }

    async fn get_user_data(&self, username: &str) -> Result<Value> {
        let python_script = format!(r#"
import json
from letterboxdpy.user import User
from letterboxdpy.movie import Movie

try:
    user = User("{}")
    
    # Get diary entries from recent data with movie details
    diary_entries = []
    recent_data = getattr(user, 'recent', {{}})
    if 'diary' in recent_data and 'months' in recent_data['diary']:
        for month, days in recent_data['diary']['months'].items():
            for day, entries in days.items():
                for entry in entries:
                    try:
                        # Get detailed movie information
                        movie = Movie(entry['slug'])
                        movie_dict = {{
                            'name': entry['name'],
                            'slug': entry['slug'],
                            'month': month,
                            'day': day,
                            'title': movie.title,
                            'year': movie.year,
                            'director': movie.crew.get('director', [{{}}])[0].get('name', None) if movie.crew.get('director') else None,
                            'genres': [g['name'] for g in movie.genres if g['type'] == 'genre'],
                            'runtime': movie.runtime,
                            'rating': movie.rating,
                            'description': movie.description
                        }}
                        diary_entries.append(movie_dict)
                    except Exception as movie_error:
                        # Fallback to basic data if movie details fail
                        diary_entries.append({{
                            'name': entry['name'],
                            'slug': entry['slug'],
                            'month': month,
                            'day': day,
                            'title': entry['name'],
                            'year': None,
                            'director': None,
                            'genres': [],
                            'runtime': None,
                            'rating': None,
                            'description': None
                        }})
    
    # Convert the user object to a dictionary
    user_dict = {{
        "username": user.username,
        "display_name": user.display_name,
        "bio": getattr(user, 'bio', None),
        "location": getattr(user, 'location', None),
        "website": getattr(user, 'website', None),
        "stats": getattr(user, 'stats', {{}}) or {{}},
        "favorites": getattr(user, 'favorites', {{}}) or {{}},
        "diary_entries": diary_entries
    }}
    
    print(json.dumps(user_dict, indent=2))
except Exception as e:
    print(f"Error: {{e}}")
    import traceback
    traceback.print_exc()
"#, username);

        let child = Command::new("python3")
            .arg("-c")
            .arg(&python_script)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = child.wait_with_output().await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to fetch user data: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let user_data: Value = serde_json::from_str(&stdout)?;
        Ok(user_data)
    }

    async fn convert_user_data_to_profile(
        &self,
        user_data: Value,
        username: &str,
    ) -> Result<ComprehensiveProfile> {
        // Extract basic profile information
        let display_name = user_data["display_name"]
            .as_str()
            .unwrap_or(username)
            .to_string();

        let stats = &user_data["stats"];
        let total_films = stats["films"].as_u64().unwrap_or(0) as u32;
        let films_this_year = stats["this_year"].as_u64().unwrap_or(0) as u32;
        let lists_count = stats["lists"].as_u64().unwrap_or(0) as u32;
        let following_count = stats["following"].as_u64().unwrap_or(0) as u32;
        let followers_count = stats["followers"].as_u64().unwrap_or(0) as u32;

        // No avatar support

        // Extract favorites
        let favorite_films = self.extract_favorites(&user_data["favorites"])?;

        // Get real diary entries from letterboxdpy
        let all_movies = self.extract_diary_entries(&user_data["diary_entries"])?;
        let recent_activity = all_movies.iter().take(10).cloned().collect();

        // No lists support
        let lists = Vec::new();

        // Calculate enhanced statistics from the movie data
        let enhanced_stats = self.calculate_enhanced_stats(&all_movies)?;

        Ok(ComprehensiveProfile {
            name: display_name,
            username: username.to_string(),
            avatar_url: None, // No avatar support
            bio: user_data["bio"].as_str().map(String::from),
            location: user_data["location"].as_str().map(String::from),
            website: user_data["website"].as_str().map(String::from),
            total_films,
            films_this_year,
            lists_count,
            following_count,
            followers_count,
            favorite_films,
            recent_activity,
            all_movies,
            watchlist: Vec::new(), // Will be filled after this method
            lists,
            member_since: None,
            enhanced_stats: Some(enhanced_stats),
        })
    }

    fn extract_favorites(&self, favorites_data: &Value) -> Result<Vec<FavoriteFilm>> {
        let mut favorites = Vec::new();
        
        if let Some(favorites_obj) = favorites_data.as_object() {
            for (_id, movie_data) in favorites_obj {
                let title = movie_data["name"].as_str().unwrap_or("Unknown").to_string();
                let slug = movie_data["slug"].as_str().unwrap_or("");
                let letterboxd_url = format!("https://letterboxd.com/film/{}", slug);
                
                favorites.push(FavoriteFilm {
                    title,
                    year: None,
                    poster_url: None,
                    letterboxd_url,
                });
            }
        }
        
        Ok(favorites)
    }

    fn extract_diary_entries(&self, diary_data: &Value) -> Result<Vec<UserMovieEntry>> {
        let mut movies = Vec::new();
        
        if let Some(entries_array) = diary_data.as_array() {
            for entry in entries_array {
                let title = entry["title"].as_str().unwrap_or("Unknown").to_string();
                let slug = entry["slug"].as_str().unwrap_or("");
                let month = entry["month"].as_u64().unwrap_or(7) as u32;
                let day = entry["day"].as_str().unwrap_or("1").parse::<u32>().unwrap_or(1);
                let year = entry["year"].as_u64().map(|y| y as u16);
                let director = entry["director"].as_str().map(String::from);
                let runtime = entry["runtime"].as_u64().map(|r| r as u16);
                let letterboxd_rating = entry["rating"].as_f64().map(|r| r as f32);
                let description = entry["description"].as_str().map(String::from);
                
                // Extract genres
                let genres = if let Some(genres_array) = entry["genres"].as_array() {
                    genres_array.iter()
                        .filter_map(|g| g.as_str())
                        .map(String::from)
                        .collect()
                } else {
                    Vec::new()
                };
                
                // Create detailed movie entry from letterboxdpy data
                let movie = DetailedMovie {
                    title: title.clone(),
                    year,
                    director,
                    genres,
                    runtime,
                    poster_url: None, // Will get from TMDB when needed
                    letterboxd_url: format!("https://letterboxd.com/film/{}", slug),
                    tmdb_url: None,
                    cast: Vec::new(),
                    synopsis: description,
                    letterboxd_rating,
                    // OMDB fields - will be filled later
                    imdb_rating: None,
                    rotten_tomatoes_rating: None,
                    metacritic_rating: None,
                    imdb_id: None,
                    release_date: None,
                    plot: None,
                    awards: None,
                };
                
                // Create a watched date from month/day (assuming current year)
                let watched_date = chrono::Utc::now()
                    .with_month(month)
                    .and_then(|d| d.with_day(day))
                    .unwrap_or(chrono::Utc::now());
                
                movies.push(UserMovieEntry {
                    movie,
                    user_rating: None, // Could extract from letterboxdpy later
                    review: None, 
                    watched_date: Some(watched_date),
                    liked: false,
                    rewatched: false,
                    tags: Vec::new(),
                });
            }
        }
        
        Ok(movies)
    }

    fn calculate_enhanced_stats(&self, movies: &[UserMovieEntry]) -> Result<EnhancedStatistics> {
        let total_films = movies.len() as u32;
        
        // Calculate basic statistics
        let total_runtime: u32 = movies.iter()
            .filter_map(|m| m.movie.runtime)
            .map(|r| r as u32)
            .sum();
        
        let total_viewing_time_hours = total_runtime as f32 / 60.0;
        let average_film_length = if total_films > 0 { 
            total_runtime as f32 / total_films as f32 
        } else { 
            0.0 
        };
        
        // Calculate ratings
        let ratings: Vec<f32> = movies.iter()
            .filter_map(|m| m.user_rating)
            .collect();
        let average_rating = if !ratings.is_empty() {
            ratings.iter().sum::<f32>() / ratings.len() as f32
        } else {
            0.0
        };
        
        // Genre analysis (now with real data!)
        let genre_breakdown = self.calculate_real_genre_stats(movies);
        
        // Director analysis (now with real data!)
        let director_stats = self.calculate_real_director_stats(movies);
        
        // Rating distribution
        let rating_distribution = self.calculate_rating_distribution(&ratings);
        
        // Yearly breakdown
        let yearly_breakdown = self.calculate_yearly_breakdown(movies);
        
        // Viewing patterns
        let viewing_patterns = self.calculate_viewing_patterns(movies);
        
        Ok(EnhancedStatistics {
            basic_stats: UserStatistics {
                total_viewing_time_hours,
                average_film_length,
                longest_streak_days: 0,
                current_streak_days: 0,
                days_with_multiple_films: 0,
                unique_directors_count: director_stats.len() as u32,
                unique_countries_count: 0,
                unique_genres_count: genre_breakdown.len() as u32,
                average_rating,
                most_watched_year: yearly_breakdown.first().map(|y| y.year),
                most_watched_decade: None,
            },
            genre_breakdown,
            country_breakdown: Vec::new(),
            director_stats,
            yearly_breakdown,
            rating_distribution,
            viewing_patterns,
            data_source: "letterboxdpy".to_string(),
        })
    }

    fn calculate_real_genre_stats(&self, movies: &[UserMovieEntry]) -> Vec<GenreStats> {
        use std::collections::HashMap;
        
        let mut genre_counts = HashMap::new();
        let mut genre_ratings = HashMap::new();
        
        for movie in movies {
            for genre in &movie.movie.genres {
                *genre_counts.entry(genre.clone()).or_insert(0) += 1;
                if let Some(rating) = movie.user_rating {
                    genre_ratings.entry(genre.clone()).or_insert(Vec::new()).push(rating);
                }
            }
        }
        
        let total_films = movies.len() as f32;
        let empty_ratings = Vec::new();
        let mut genre_stats: Vec<GenreStats> = genre_counts.into_iter()
            .map(|(name, count)| {
                let percentage = (count as f32 / total_films) * 100.0;
                let ratings = genre_ratings.get(&name).unwrap_or(&empty_ratings);
                let average_rating = if !ratings.is_empty() {
                    ratings.iter().sum::<f32>() / ratings.len() as f32
                } else {
                    0.0
                };
                
                let emoji = self.get_genre_emoji(&name);
                
                GenreStats {
                    name,
                    count,
                    percentage,
                    average_rating,
                    emoji,
                }
            })
            .collect();
        
        genre_stats.sort_by(|a, b| b.count.cmp(&a.count));
        genre_stats.truncate(10);
        genre_stats
    }

    fn calculate_real_director_stats(&self, movies: &[UserMovieEntry]) -> Vec<DirectorStats> {
        use std::collections::HashMap;
        
        let mut director_data: HashMap<String, (u32, Vec<f32>, Vec<String>)> = HashMap::new();
        
        for movie in movies {
            if let Some(ref director) = movie.movie.director {
                let entry = director_data.entry(director.clone()).or_insert((0, Vec::new(), Vec::new()));
                entry.0 += 1;
                if let Some(rating) = movie.user_rating {
                    entry.1.push(rating);
                }
                entry.2.push(movie.movie.title.clone());
            }
        }
        
        let mut director_stats: Vec<DirectorStats> = director_data.into_iter()
            .map(|(name, (film_count, ratings, titles))| {
                let average_rating = if !ratings.is_empty() {
                    ratings.iter().sum::<f32>() / ratings.len() as f32
                } else {
                    0.0
                };
                
                let favorite_film = if !ratings.is_empty() {
                    let max_rating = ratings.iter().fold(0.0f32, |a, &b| a.max(b));
                    titles.iter()
                        .zip(ratings.iter())
                        .find(|(_, &rating)| rating == max_rating)
                        .map(|(title, _)| title.clone())
                } else {
                    titles.first().cloned()
                };
                
                DirectorStats {
                    name,
                    film_count,
                    average_rating,
                    favorite_film,
                }
            })
            .collect();
        
        director_stats.sort_by(|a, b| b.film_count.cmp(&a.film_count));
        director_stats.truncate(10);
        director_stats
    }

    fn calculate_rating_distribution(&self, ratings: &[f32]) -> Vec<RatingDistribution> {
        use std::collections::HashMap;
        
        let mut distribution = HashMap::new();
        for &rating in ratings {
            let rounded = (rating * 2.0).round() / 2.0;
            let key = format!("{:.1}", rounded);
            *distribution.entry(key).or_insert(0) += 1;
        }
        
        let total = ratings.len() as f32;
        let mut result: Vec<RatingDistribution> = distribution.into_iter()
            .map(|(rating_str, count)| RatingDistribution {
                rating: rating_str.parse::<f32>().unwrap_or(0.0),
                count,
                percentage: (count as f32 / total) * 100.0,
            })
            .collect();
        
        result.sort_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap());
        result
    }

    fn calculate_yearly_breakdown(&self, movies: &[UserMovieEntry]) -> Vec<YearlyBreakdown> {
        use std::collections::HashMap;
        use chrono::Datelike;
        
        let mut yearly_data: HashMap<u16, (u32, u32, Vec<f32>, Vec<String>)> = HashMap::new();
        
        for movie in movies {
            if let Some(date) = movie.watched_date {
                let watch_year = date.year() as u16;
                let entry = yearly_data.entry(watch_year).or_insert((0, 0, Vec::new(), Vec::new()));
                entry.0 += 1;
                entry.1 += movie.movie.runtime.unwrap_or(0) as u32;
                if let Some(rating) = movie.user_rating {
                    entry.2.push(rating);
                }
                entry.3.push(movie.movie.title.clone());
            }
        }
        
        let mut yearly_breakdown: Vec<YearlyBreakdown> = yearly_data.into_iter()
            .map(|(year, (film_count, total_runtime, ratings, titles))| {
                let average_rating = if !ratings.is_empty() {
                    ratings.iter().sum::<f32>() / ratings.len() as f32
                } else {
                    0.0
                };
                
                let favorite_film = if !ratings.is_empty() {
                    let max_rating = ratings.iter().fold(0.0f32, |a, &b| a.max(b));
                    titles.iter()
                        .zip(ratings.iter())
                        .find(|(_, &rating)| rating == max_rating)
                        .map(|(title, _)| title.clone())
                } else {
                    titles.first().cloned()
                };
                
                YearlyBreakdown {
                    year,
                    film_count,
                    total_runtime,
                    average_rating,
                    top_genre: None,
                    favorite_film,
                }
            })
            .collect();
        
        yearly_breakdown.sort_by(|a, b| b.year.cmp(&a.year));
        yearly_breakdown
    }

    fn calculate_viewing_patterns(&self, movies: &[UserMovieEntry]) -> Vec<ViewingPattern> {
        use std::collections::HashMap;
        use chrono::Datelike;
        
        let mut monthly_counts = HashMap::new();
        
        for movie in movies {
            if let Some(date) = movie.watched_date {
                let month = date.month();
                *monthly_counts.entry(month).or_insert(0) += 1;
            }
        }
        
        let mut patterns: Vec<ViewingPattern> = monthly_counts.into_iter()
            .map(|(month, films_watched)| ViewingPattern {
                month,
                films_watched,
                busiest_day: None,
            })
            .collect();
        
        patterns.sort_by_key(|p| p.month);
        patterns
    }

    fn get_genre_emoji(&self, genre: &str) -> String {
        match genre.to_lowercase().as_str() {
            "action" => "💥".to_string(),
            "adventure" => "🗺️".to_string(),
            "animation" => "🎨".to_string(),
            "comedy" => "😂".to_string(),
            "crime" => "🔫".to_string(),
            "documentary" => "📹".to_string(),
            "drama" => "🎭".to_string(),
            "family" => "👨‍👩‍👧‍👦".to_string(),
            "fantasy" => "🧙‍♂️".to_string(),
            "history" => "📜".to_string(),
            "horror" => "👻".to_string(),
            "music" => "🎵".to_string(),
            "mystery" => "🔍".to_string(),
            "romance" => "💕".to_string(),
            "science fiction" | "sci-fi" => "🚀".to_string(),
            "thriller" => "😱".to_string(),
            "war" => "⚔️".to_string(),
            "western" => "🤠".to_string(),
            _ => "🎬".to_string(),
        }
    }

    async fn get_watchlist_data(&self, username: &str) -> Result<Value> {
        let python_script = format!(r#"
import json
from letterboxdpy.user import User

try:
    user = User("{}")
    watchlist = user.get_watchlist_movies()
    
    # Convert watchlist to our format
    watchlist_entries = []
    for movie_id, movie_data in watchlist.items():
        watchlist_entries.append({{
            'id': movie_id,
            'title': movie_data.get('name', 'Unknown'),
            'slug': movie_data.get('slug', ''),
            'url': movie_data.get('url', '')
        }})
    
    print(json.dumps(watchlist_entries, indent=2))
except Exception as e:
    print(f"Error: {{e}}")
    import traceback
    traceback.print_exc()
"#, username);

        let child = Command::new("python3")
            .arg("-c")
            .arg(&python_script)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let output = child.wait_with_output().await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to fetch watchlist data: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let watchlist_data: Value = serde_json::from_str(&stdout)?;
        Ok(watchlist_data)
    }

    async fn convert_watchlist_to_movies(&self, watchlist_data: Value) -> Result<Vec<DetailedMovie>> {
        let mut movies = Vec::new();
        
        if let Some(entries_array) = watchlist_data.as_array() {
            // Limit to first 50 entries for performance
            for entry in entries_array.iter().take(50) {
                let title = entry["title"].as_str().unwrap_or("Unknown").to_string();
                let slug = entry["slug"].as_str().unwrap_or("");
                
                let movie = DetailedMovie {
                    title: title.clone(),
                    year: None, // Will be filled by OMDB
                    director: None, // Will be filled by OMDB
                    genres: Vec::new(), // Will be filled by OMDB
                    runtime: None, // Will be filled by OMDB
                    poster_url: None, // Will be filled by TMDB
                    letterboxd_url: format!("https://letterboxd.com/film/{}", slug),
                    tmdb_url: None,
                    cast: Vec::new(),
                    synopsis: None, // Will be filled by OMDB
                    letterboxd_rating: None,
                    // OMDB fields - will be filled later
                    imdb_rating: None,
                    rotten_tomatoes_rating: None,
                    metacritic_rating: None,
                    imdb_id: None,
                    release_date: None,
                    plot: None,
                    awards: None,
                };
                
                movies.push(movie);
            }
        }
        
        Ok(movies)
    }

    async fn enrich_with_omdb(&self, mut profile: ComprehensiveProfile) -> Result<ComprehensiveProfile> {
        let omdb_client = OMDBClient::new();
        
        // Enrich recent activity movies (limit to 10 to avoid rate limits)
        for entry in profile.recent_activity.iter_mut().take(10) {
            if let Ok(Some(omdb_movie)) = omdb_client.get_movie_by_title(&entry.movie.title, entry.movie.year).await {
                entry.movie.imdb_rating = omdb_client.get_imdb_rating(&omdb_movie);
                entry.movie.rotten_tomatoes_rating = omdb_client.get_rotten_tomatoes_rating(&omdb_movie);
                entry.movie.metacritic_rating = omdb_client.get_metacritic_rating(&omdb_movie);
                entry.movie.imdb_id = omdb_movie.imdb_id.clone();
                entry.movie.release_date = omdb_movie.released.clone();
                entry.movie.plot = omdb_movie.plot.clone();
                entry.movie.awards = omdb_movie.awards.clone();
            }
            
            // Small delay to respect rate limits
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        
        // Enrich first 10 watchlist movies
        for movie in profile.watchlist.iter_mut().take(10) {
            if let Ok(Some(omdb_movie)) = omdb_client.get_movie_by_title(&movie.title, movie.year).await {
                movie.year = omdb_movie.year.parse().ok();
                movie.director = omdb_movie.director.clone();
                movie.runtime = omdb_movie.runtime.as_ref()
                    .and_then(|r| r.trim_end_matches(" min").parse().ok());
                movie.genres = omdb_movie.genre.as_ref()
                    .map(|g| g.split(", ").map(String::from).collect())
                    .unwrap_or_default();
                movie.imdb_rating = omdb_client.get_imdb_rating(&omdb_movie);
                movie.rotten_tomatoes_rating = omdb_client.get_rotten_tomatoes_rating(&omdb_movie);
                movie.metacritic_rating = omdb_client.get_metacritic_rating(&omdb_movie);
                movie.imdb_id = omdb_movie.imdb_id.clone();
                movie.release_date = omdb_movie.released.clone();
                movie.plot = omdb_movie.plot.clone();
                movie.awards = omdb_movie.awards.clone();
                movie.synopsis = omdb_movie.plot.clone();
            }
            
            // Small delay to respect rate limits
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        
        Ok(profile)
    }
}