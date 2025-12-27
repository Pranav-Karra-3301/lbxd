use crate::models::{EntryType, Movie, UserEntry, UserProfile};
use anyhow::{anyhow, Result};
use feed_rs::parser;
use regex::Regex;
use reqwest;
use std::time::Duration;

pub struct FeedParser {
    client: reqwest::Client,
}

impl Default for FeedParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedParser {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("lbxd/3.0.0 (https://github.com/Pranav-Karra-3301/lbxd)")
            .build()
            .unwrap_or_default();

        Self { client }
    }

    pub async fn fetch_user_feed(&self, username: &str) -> Result<UserProfile> {
        let rss_url = format!("https://letterboxd.com/{}/rss/", username);

        let response = self
            .client
            .get(&rss_url)
            .header("User-Agent", "lbxd/3.0.0")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch RSS feed for user: {}", username));
        }

        let content = response.text().await?;
        let feed = parser::parse(content.as_bytes())?;

        let mut entries = Vec::new();

        for item in feed.entries {
            if let Some(entry) = self.parse_entry(item).await {
                entries.push(entry);
            }
        }

        Ok(UserProfile {
            username: username.to_string(),
            display_name: feed.title.map(|t| t.content),
            avatar_url: None,
            rss_url,
            entries,
        })
    }

    async fn parse_entry(&self, item: feed_rs::model::Entry) -> Option<UserEntry> {
        let title = item.title?.content;
        let link = item.links.first()?.href.clone();

        let movie = self.extract_movie_info(&title, &link).await?;
        let entry_type = self.determine_entry_type(&title);
        let rating = self.extract_rating(&title);
        let review = item.summary.map(|s| s.content);
        let watched_date = item.published.or(item.updated);

        Some(UserEntry {
            movie,
            rating,
            review,
            watched_date,
            entry_type,
            liked: title.contains("♥"),
        })
    }

    async fn extract_movie_info(&self, title: &str, url: &str) -> Option<Movie> {
        let re = Regex::new(r"(.+?)\s*(\d{4})").ok()?;

        let (movie_title, year) = if let Some(caps) = re.captures(title) {
            let title = caps.get(1)?.as_str().trim().to_string();
            let year = caps.get(2)?.as_str().parse().ok();
            (title, year)
        } else {
            (title.to_string(), None)
        };

        // Don't fetch poster URL here - let the display handle TMDB lookup
        Some(Movie {
            title: movie_title,
            year,
            director: None,
            letterboxd_url: url.to_string(),
            poster_url: None, // Will be fetched by display layer using TMDB
            tmdb_id: None,
        })
    }

    fn determine_entry_type(&self, title: &str) -> EntryType {
        if title.contains("★") {
            EntryType::Review
        } else if title.contains("♥") {
            EntryType::Like
        } else {
            EntryType::Watch
        }
    }

    fn extract_rating(&self, title: &str) -> Option<f32> {
        let star_count = title.matches('★').count() as f32;
        let half_star_count = title.matches('½').count() as f32;

        if star_count > 0.0 || half_star_count > 0.0 {
            Some(star_count + (half_star_count * 0.5))
        } else {
            None
        }
    }
}
