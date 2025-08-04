use crate::models::UserEntry;
use crate::tmdb::TMDBClient;
use colored::*;
use std::io::{self, Write};
use tokio::time::{interval, Duration};

pub struct BatchLoader;

#[derive(Debug)]
pub struct BatchResult {
    pub entry: UserEntry,
    pub poster_url: Option<String>,
    pub tmdb_movie: Option<crate::tmdb::TMDBMovie>,
}

impl BatchLoader {
    pub fn new() -> Self {
        Self
    }

    pub async fn process_entries_with_progress(&self, entries: &[&UserEntry]) -> Vec<BatchResult> {
        let total = entries.len();
        let mut results = Vec::new();

        // Start the progress indicator
        let progress_handle = tokio::spawn(Self::show_unified_progress_static(total));

        // Process all entries concurrently
        let mut handles = Vec::new();
        for entry in entries {
            let tmdb_client = TMDBClient::new();
            let entry_clone = (*entry).clone();

            let handle = tokio::spawn(async move {
                let cleaned_title = Self::clean_title_for_search(&entry_clone.movie.title);

                // Try with year first
                let tmdb_result = if let Some(year) = entry_clone.movie.year {
                    match tmdb_client
                        .search_movie_with_year(&cleaned_title, Some(year))
                        .await
                    {
                        Ok(Some(movie)) => Some(movie),
                        Ok(None) => {
                            // Fallback without year
                            tmdb_client
                                .search_movie_with_year(&cleaned_title, None)
                                .await
                                .ok()
                                .flatten()
                        }
                        Err(_) => tmdb_client
                            .search_movie_with_year(&cleaned_title, None)
                            .await
                            .ok()
                            .flatten(),
                    }
                } else {
                    tmdb_client
                        .search_movie_with_year(&cleaned_title, None)
                        .await
                        .ok()
                        .flatten()
                };

                let poster_url = tmdb_result
                    .as_ref()
                    .and_then(|movie| movie.get_full_poster_url());

                BatchResult {
                    entry: entry_clone,
                    poster_url,
                    tmdb_movie: tmdb_result,
                }
            });

            handles.push(handle);
        }

        // Collect all results
        for handle in handles {
            if let Ok(result) = handle.await {
                results.push(result);
            }
        }

        // Stop progress indicator
        progress_handle.abort();

        // Clear the progress line
        print!("\r\x1b[2K");
        io::stdout().flush().unwrap();

        results
    }

    async fn show_unified_progress_static(total: usize) {
        let frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let mut frame_index = 0;
        let mut interval = interval(Duration::from_millis(100));

        loop {
            interval.tick().await;
            print!(
                "\r{} Loading {} movie{} and poster{}...",
                frames[frame_index].to_string().cyan().bold(),
                total.to_string().cyan().bold(),
                if total == 1 { "" } else { "s" },
                if total == 1 { "" } else { "s" }
            );
            io::stdout().flush().unwrap();
            frame_index = (frame_index + 1) % frames.len();
        }
    }

    fn clean_title_for_search(title: &str) -> String {
        // Remove common problematic characters and patterns that might interfere with TMDB search
        let mut cleaned = title.to_string();

        // Remove trailing asterisks (like "Thunderbolts*")
        cleaned = cleaned.trim_end_matches('*').to_string();

        // Remove extra whitespace and normalize
        cleaned = cleaned.trim().to_string();

        // Replace multiple spaces with single space
        let re = regex::Regex::new(r"\s+").unwrap();
        cleaned = re.replace_all(&cleaned, " ").to_string();

        cleaned
    }
}
