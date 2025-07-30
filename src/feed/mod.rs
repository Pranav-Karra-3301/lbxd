use crate::models::{Movie, UserEntry, UserProfile, EntryType};
use anyhow::{anyhow, Result};
use feed_rs::parser;
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use std::time::Duration;

pub struct FeedParser {
    client: reqwest::Client,
}

impl FeedParser {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("lbxd/1.0.0 (https://github.com/your-repo/lbxd)")
            .build()
            .unwrap_or_default();
            
        Self { client }
    }

    pub async fn fetch_user_feed(&self, username: &str) -> Result<UserProfile> {
        let rss_url = format!("https://letterboxd.com/{}/rss/", username);
        
        let response = self.client
            .get(&rss_url)
            .header("User-Agent", "lbxd/1.0.0")
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
        
        // Extract poster URL from the movie page
        let poster_url = self.scrape_poster_url(url).await;
        
        Some(Movie {
            title: movie_title,
            year,
            director: None,
            letterboxd_url: url.to_string(),
            poster_url,
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

    async fn scrape_poster_url(&self, movie_url: &str) -> Option<String> {
        // Try to fetch the movie page to extract poster URL
        let response = self.client
            .get(movie_url)
            .header("User-Agent", "lbxd/1.0.0")
            .send()
            .await
            .ok()?;
            
        if !response.status().is_success() {
            return None;
        }
        
        let html_content = response.text().await.ok()?;
        let document = Html::parse_document(&html_content);
        
        // Try multiple selectors for poster images
        let selectors = [
            "img.image",  // Main movie poster
            "img[alt*='poster']",  // Images with 'poster' in alt text
            ".film-poster img",  // Film poster container
            "img[src*='image']",  // Generic image selector
            "meta[property='og:image']",  // Open Graph image
        ];
        
        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    // For img tags
                    if let Some(src) = element.value().attr("src") {
                        if self.is_valid_poster_url(src) {
                            return Some(self.normalize_poster_url(src));
                        }
                    }
                    // For meta tags
                    if let Some(content) = element.value().attr("content") {
                        if self.is_valid_poster_url(content) {
                            return Some(self.normalize_poster_url(content));
                        }
                    }
                }
            }
        }
        
        None
    }
    
    fn is_valid_poster_url(&self, url: &str) -> bool {
        // Check if it's a valid poster URL from Letterboxd's CDN
        (url.contains("ltrbxd.com") || url.contains("letterboxd.com")) &&
        (url.contains(".jpg") || url.contains(".jpeg") || url.contains(".png") || url.contains(".webp")) &&
        !url.contains("avatar") && // Exclude user avatars
        !url.contains("backdrop") // Exclude backdrop images
    }
    
    fn normalize_poster_url(&self, url: &str) -> String {
        // Ensure we have a full URL
        if url.starts_with("//") {
            format!("https:{}", url)
        } else if url.starts_with("/") {
            format!("https://letterboxd.com{}", url)
        } else if !url.starts_with("http") {
            format!("https://{}", url)
        } else {
            // Try to get a higher quality version by modifying the URL
            let high_quality = url
                .replace("-0-70-0-105-crop", "-0-230-0-345-crop")  // Larger size
                .replace("-70-105", "-230-345")  // Alternative format
                .replace("w92", "w500")  // For TMDB-style URLs
                .replace("w185", "w500");  // For TMDB-style URLs
            high_quality
        }
    }
}