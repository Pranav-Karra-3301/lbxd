use anyhow::Result;
use chrono::Datelike;
use rustboxd::{User, Movie, DiaryMovieEntry, WatchlistMovie};
use std::collections::HashMap;
use tokio::sync::mpsc;

use crate::omdb::OMDBClient;
use crate::profile::{
    ComprehensiveProfile, DetailedMovie, DirectorStats, EnhancedStatistics, FavoriteFilm,
    GenreStats, LoadingProgress, LoadingStage, RatingDistribution, UserMovieEntry, UserStatistics,
    ViewingPattern, YearlyBreakdown,
};

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
                message: "Fetching user profile...".to_string(),
            });
        }

        // Get user data using rustboxd
        let user = User::new(username).await?;
        
        if let Some(ref tx) = progress_tx {
            let _ = tx.send(LoadingProgress {
                stage: LoadingStage::Diary,
                current: 1,
                total: 4,
                message: "Fetching diary entries...".to_string(),
            });
        }

        // Get diary entries
        let diary_entries = user.get_diary_entries().await?;

        if let Some(ref tx) = progress_tx {
            let _ = tx.send(LoadingProgress {
                stage: LoadingStage::Lists,
                current: 2,
                total: 5,
                message: "Fetching watchlist...".to_string(),
            });
        }

        // Get watchlist data
        let watchlist_movies = user.get_watchlist_movies().await?;

        if let Some(ref tx) = progress_tx {
            let _ = tx.send(LoadingProgress {
                stage: LoadingStage::MovieDetails,
                current: 3,
                total: 5,
                message: "Processing profile data...".to_string(),
            });
        }

        // Convert the data to our Rust structures
        let mut comprehensive_profile = self
            .convert_user_data_to_profile(user, diary_entries, username)
            .await?;

        // Add watchlist data
        let watchlist_movies_vec = self
            .convert_watchlist_to_movies(watchlist_movies)
            .await?;
        let total_watchlist_available = watchlist_movies_vec.len();

        comprehensive_profile.watchlist = watchlist_movies_vec;
        comprehensive_profile.watchlist_loaded = comprehensive_profile.watchlist.len();
        comprehensive_profile.total_watchlist_available = total_watchlist_available;

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

    async fn convert_user_data_to_profile(
        &self,
        user: User,
        diary_entries: Vec<DiaryMovieEntry>,
        username: &str,
    ) -> Result<ComprehensiveProfile> {
        // Extract basic profile information
        let display_name = user.display_name.clone();
        
        let stats = user.stats.as_ref();
        let total_films = stats.map(|s| s.films).unwrap_or(0);
        let films_this_year = stats.map(|s| s.this_year).unwrap_or(0);
        let lists_count = stats.map(|s| s.lists).unwrap_or(0);
        let following_count = stats.map(|s| s.following).unwrap_or(0);
        let followers_count = stats.map(|s| s.followers).unwrap_or(0);

        // Extract favorites
        let favorite_films = self.extract_favorites(&user)?;

        // Convert diary entries to UserMovieEntry
        let all_movies = self.convert_diary_entries(diary_entries)?;
        let total_movies_available = all_movies.len();
        let recent_activity = all_movies.iter().take(10).cloned().collect();

        // No lists support for now
        let lists = Vec::new();

        // Calculate enhanced statistics from the movie data
        let enhanced_stats = self.calculate_enhanced_stats(&all_movies)?;

        Ok(ComprehensiveProfile {
            name: display_name,
            username: username.to_string(),
            avatar_url: user.avatar,
            bio: user.bio,
            location: user.location,
            website: user.website,
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
            // Pagination fields
            movies_loaded: 10.min(total_movies_available),
            total_movies_available,
            watchlist_loaded: 0, // Will be updated when watchlist is loaded
            total_watchlist_available: 0, // Will be updated when watchlist is loaded
        })
    }

    fn extract_favorites(&self, user: &User) -> Result<Vec<FavoriteFilm>> {
        let mut favorites = Vec::new();

        if let Some(ref favorites_map) = user.favorites {
            for (_id, movie_data) in favorites_map {
                let title = movie_data.name.clone();
                let slug = movie_data.slug.clone();
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

    fn convert_diary_entries(&self, diary_entries: Vec<DiaryMovieEntry>) -> Result<Vec<UserMovieEntry>> {
        let mut movies = Vec::new();

        for entry in diary_entries {
            let movie = DetailedMovie {
                title: entry.title.clone(),
                year: entry.year,
                director: entry.director,
                genres: entry.genres,
                runtime: entry.runtime,
                poster_url: None, // Will get from TMDB when needed
                letterboxd_url: format!("https://letterboxd.com/film/{}", entry.slug),
                tmdb_url: None,
                cast: Vec::new(),
                synopsis: entry.description.clone(),
                letterboxd_rating: entry.rating,
                // OMDB fields - will be filled later
                imdb_rating: None,
                rotten_tomatoes_rating: None,
                metacritic_rating: None,
                imdb_id: None,
                release_date: None,
                plot: entry.description,
                awards: None,
            };

            // Create a watched date from month/day (assuming current year)
            let watched_date = chrono::Utc::now()
                .with_month(entry.month)
                .and_then(|d| d.with_day(entry.day))
                .unwrap_or(chrono::Utc::now());

            movies.push(UserMovieEntry {
                movie,
                user_rating: None, // Could extract from rustboxd later
                review: None,
                watched_date: Some(watched_date),
                liked: false,
                rewatched: false,
                tags: Vec::new(),
            });
        }

        Ok(movies)
    }

    async fn convert_watchlist_to_movies(
        &self,
        watchlist: HashMap<String, WatchlistMovie>,
    ) -> Result<Vec<DetailedMovie>> {
        let mut movies = Vec::new();

        // Limit to first 10 entries for performance
        for (_slug, movie_data) in watchlist.into_iter().take(10) {
            let movie = DetailedMovie {
                title: movie_data.name.clone(),
                year: None,         // Will be filled by OMDB
                director: None,     // Will be filled by OMDB
                genres: Vec::new(), // Will be filled by OMDB
                runtime: None,      // Will be filled by OMDB
                poster_url: None,   // Will be filled by TMDB
                letterboxd_url: movie_data.url,
                tmdb_url: None,
                cast: Vec::new(),
                synopsis: None,
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

        Ok(movies)
    }

    fn calculate_enhanced_stats(&self, movies: &[UserMovieEntry]) -> Result<EnhancedStatistics> {
        let total_films = movies.len() as u32;

        // Calculate basic statistics
        let total_runtime: u32 = movies
            .iter()
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
        let ratings: Vec<f32> = movies.iter().filter_map(|m| m.user_rating).collect();
        let average_rating = if !ratings.is_empty() {
            ratings.iter().sum::<f32>() / ratings.len() as f32
        } else {
            0.0
        };

        // Genre analysis
        let genre_breakdown = self.calculate_real_genre_stats(movies);

        // Director analysis
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
            data_source: "rustboxd".to_string(),
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
                    genre_ratings
                        .entry(genre.clone())
                        .or_insert(Vec::new())
                        .push(rating);
                }
            }
        }

        let total_films = movies.len() as f32;
        let empty_ratings = Vec::new();
        let mut genre_stats: Vec<GenreStats> = genre_counts
            .into_iter()
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
                let entry =
                    director_data
                        .entry(director.clone())
                        .or_insert((0, Vec::new(), Vec::new()));
                entry.0 += 1;
                if let Some(rating) = movie.user_rating {
                    entry.1.push(rating);
                }
                entry.2.push(movie.movie.title.clone());
            }
        }

        let mut director_stats: Vec<DirectorStats> = director_data
            .into_iter()
            .map(|(name, (film_count, ratings, titles))| {
                let average_rating = if !ratings.is_empty() {
                    ratings.iter().sum::<f32>() / ratings.len() as f32
                } else {
                    0.0
                };

                let favorite_film = if !ratings.is_empty() {
                    let max_rating = ratings.iter().fold(0.0f32, |a, &b| a.max(b));
                    titles
                        .iter()
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
        let mut result: Vec<RatingDistribution> = distribution
            .into_iter()
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
        use chrono::Datelike;
        use std::collections::HashMap;

        let mut yearly_data: HashMap<u16, (u32, u32, Vec<f32>, Vec<String>)> = HashMap::new();

        for movie in movies {
            if let Some(date) = movie.watched_date {
                let watch_year = date.year() as u16;
                let entry = yearly_data
                    .entry(watch_year)
                    .or_insert((0, 0, Vec::new(), Vec::new()));
                entry.0 += 1;
                entry.1 += movie.movie.runtime.unwrap_or(0) as u32;
                if let Some(rating) = movie.user_rating {
                    entry.2.push(rating);
                }
                entry.3.push(movie.movie.title.clone());
            }
        }

        let mut yearly_breakdown: Vec<YearlyBreakdown> = yearly_data
            .into_iter()
            .map(|(year, (film_count, total_runtime, ratings, titles))| {
                let average_rating = if !ratings.is_empty() {
                    ratings.iter().sum::<f32>() / ratings.len() as f32
                } else {
                    0.0
                };

                let favorite_film = if !ratings.is_empty() {
                    let max_rating = ratings.iter().fold(0.0f32, |a, &b| a.max(b));
                    titles
                        .iter()
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
        use chrono::Datelike;
        use std::collections::HashMap;

        let mut monthly_counts = HashMap::new();

        for movie in movies {
            if let Some(date) = movie.watched_date {
                let month = date.month();
                *monthly_counts.entry(month).or_insert(0) += 1;
            }
        }

        let mut patterns: Vec<ViewingPattern> = monthly_counts
            .into_iter()
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

    async fn enrich_with_omdb(
        &self,
        mut profile: ComprehensiveProfile,
    ) -> Result<ComprehensiveProfile> {
        let omdb_client = OMDBClient::new();

        // Enrich recent activity movies (limit to 10 to avoid rate limits)
        for entry in profile.recent_activity.iter_mut().take(10) {
            if let Ok(Some(omdb_movie)) = omdb_client
                .get_movie_by_title(&entry.movie.title, entry.movie.year)
                .await
            {
                entry.movie.imdb_rating = omdb_client.get_imdb_rating(&omdb_movie);
                entry.movie.rotten_tomatoes_rating =
                    omdb_client.get_rotten_tomatoes_rating(&omdb_movie);
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
            if let Ok(Some(omdb_movie)) = omdb_client
                .get_movie_by_title(&movie.title, movie.year)
                .await
            {
                movie.year = omdb_movie.year.parse().ok();
                movie.director = omdb_movie.director.clone();
                movie.runtime = omdb_movie
                    .runtime
                    .as_ref()
                    .and_then(|r| r.trim_end_matches(" min").parse().ok());
                movie.genres = omdb_movie
                    .genre
                    .as_ref()
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

    pub async fn load_more_movies(
        &self,
        username: &str,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<crate::profile::UserMovieEntry>> {
        // Get user and diary entries
        let user = User::new(username).await?;
        let all_diary_entries = user.get_diary_entries().await?;

        // Convert all entries
        let all_movies = self.convert_diary_entries(all_diary_entries)?;

        // Return the requested slice
        let end_index = (offset + limit).min(all_movies.len());
        if offset >= all_movies.len() {
            return Ok(Vec::new());
        }

        let mut batch = all_movies[offset..end_index].to_vec();

        // Enrich with OMDB data
        let omdb_client = crate::omdb::OMDBClient::new();
        for entry in batch.iter_mut() {
            if let Ok(Some(omdb_movie)) = omdb_client
                .get_movie_by_title(&entry.movie.title, entry.movie.year)
                .await
            {
                entry.movie.imdb_rating = omdb_client.get_imdb_rating(&omdb_movie);
                entry.movie.rotten_tomatoes_rating =
                    omdb_client.get_rotten_tomatoes_rating(&omdb_movie);
                entry.movie.metacritic_rating = omdb_client.get_metacritic_rating(&omdb_movie);
                entry.movie.imdb_id = omdb_movie.imdb_id.clone();
                entry.movie.release_date = omdb_movie.released.clone();
                entry.movie.plot = omdb_movie.plot.clone();
                entry.movie.awards = omdb_movie.awards.clone();
            }

            // Small delay to respect rate limits
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        Ok(batch)
    }

    pub async fn load_more_watchlist(
        &self,
        username: &str,
        offset: usize,
        limit: usize,
    ) -> Result<Vec<crate::profile::DetailedMovie>> {
        // Get user and watchlist
        let user = User::new(username).await?;
        let watchlist_data = user.get_watchlist_movies().await?;

        let watchlist_vec: Vec<_> = watchlist_data.into_iter().collect();
        if offset >= watchlist_vec.len() {
            return Ok(Vec::new());
        }

        let mut movies = Vec::new();
        for (_slug, movie_data) in watchlist_vec.into_iter().skip(offset).take(limit) {
            let movie = crate::profile::DetailedMovie {
                title: movie_data.name.clone(),
                year: None,
                director: None,
                genres: Vec::new(),
                runtime: None,
                poster_url: None,
                letterboxd_url: movie_data.url,
                tmdb_url: None,
                cast: Vec::new(),
                synopsis: None,
                letterboxd_rating: None,
                // OMDB fields - will be filled below
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

        // Enrich with OMDB data
        let omdb_client = crate::omdb::OMDBClient::new();
        for movie in movies.iter_mut() {
            if let Ok(Some(omdb_movie)) = omdb_client
                .get_movie_by_title(&movie.title, movie.year)
                .await
            {
                movie.year = omdb_movie.year.parse().ok();
                movie.director = omdb_movie.director.clone();
                movie.runtime = omdb_movie
                    .runtime
                    .as_ref()
                    .and_then(|r| r.trim_end_matches(" min").parse().ok());
                movie.genres = omdb_movie
                    .genre
                    .as_ref()
                    .map(|g| g.split(", ").map(String::from).collect())
                    .unwrap_or_default();
                movie.imdb_rating = omdb_client.get_imdb_rating(&omdb_movie);
                movie.rotten_tomatoes_rating =
                    omdb_client.get_rotten_tomatoes_rating(&omdb_movie);
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

        Ok(movies)
    }
}