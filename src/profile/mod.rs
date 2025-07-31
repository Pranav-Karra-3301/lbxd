use anyhow::Result;
use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteFilm {
    pub title: String,
    pub year: Option<u16>,
    pub poster_url: Option<String>,
    pub letterboxd_url: String,
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

pub struct ProfileScraper {
    client: reqwest::Client,
}

impl ProfileScraper {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .build()
            .unwrap_or_default();
            
        Self { client }
    }

    pub async fn scrape_profile(&self, username: &str) -> Result<ProfileStats> {
        let url = format!("https://letterboxd.com/{}/", username);
        
        // Add small delay to be respectful
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch profile: HTTP {}", response.status()));
        }
        
        let html = response.text().await?;
        self.parse_profile(&html, username).await
    }

    async fn parse_profile(&self, html: &str, username: &str) -> Result<ProfileStats> {
        let document = Html::parse_document(html);
        
        // Extract profile name - try multiple selectors
        let name = document
            .select(&Selector::parse(".profile-person h1").unwrap())
            .next()
            .or_else(|| document.select(&Selector::parse("h1.title-1").unwrap()).next())
            .or_else(|| document.select(&Selector::parse("h1").unwrap()).next())
            .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
            .filter(|s| !s.is_empty() && !s.contains("Letterboxd"))
            .unwrap_or_else(|| username.to_string());
            
        // Extract avatar URL
        let avatar_selector = Selector::parse("img.avatar").unwrap();
        let avatar_url = document
            .select(&avatar_selector)
            .next()
            .and_then(|el| el.value().attr("src"))
            .map(|src| {
                if src.starts_with("//") {
                    format!("https:{}", src)
                } else if src.starts_with("/") {
                    format!("https://letterboxd.com{}", src)
                } else {
                    src.to_string()
                }
            });

        // Extract stats using link parsing
        let stats = self.extract_stats(&document, username)?;
        
        // Extract favorite films
        let favorite_films = self.extract_favorite_films(&document).await?;

        Ok(ProfileStats {
            name,
            username: username.to_string(),
            avatar_url,
            total_films: stats.0,
            films_this_year: stats.1,
            lists_count: stats.2,
            following_count: stats.3,
            followers_count: stats.4,
            favorite_films,
        })
    }

    fn extract_stats(&self, document: &Html, username: &str) -> Result<(u32, u32, u32, u32, u32)> {
        let number_regex = Regex::new(r"(\d+)").unwrap();
        
        // Helper function to extract number from link text
        let extract_number = |href_pattern: &str| -> u32 {
            let selector = Selector::parse(&format!("a[href*='{}']", href_pattern)).unwrap();
            document
                .select(&selector)
                .next()
                .and_then(|el| {
                    let text = el.text().collect::<Vec<_>>().join(" ");
                    number_regex.find(&text)
                        .and_then(|m| m.as_str().parse::<u32>().ok())
                })
                .unwrap_or(0)
        };

        let total_films = extract_number(&format!("/{}/films/", username));
        let films_this_year = extract_number(&format!("/{}/films/diary/for/", username));
        let lists_count = extract_number(&format!("/{}/lists/", username));
        let following_count = extract_number(&format!("/{}/following/", username));
        let followers_count = extract_number(&format!("/{}/followers/", username));

        Ok((total_films, films_this_year, lists_count, following_count, followers_count))
    }

    async fn extract_favorite_films(&self, document: &Html) -> Result<Vec<FavoriteFilm>> {
        let mut favorites = Vec::new();
        
        // Look for favorite films section
        let favorites_selector = Selector::parse(".poster-list li").unwrap();
        
        for element in document.select(&favorites_selector).take(4) { // Limit to 4 favorites
            if let Some(link) = element.select(&Selector::parse("a").unwrap()).next() {
                let letterboxd_url = link.value().attr("href")
                    .map(|href| {
                        if href.starts_with("/") {
                            format!("https://letterboxd.com{}", href)
                        } else {
                            href.to_string()
                        }
                    })
                    .unwrap_or_default();
                
                // Extract title from link or img alt
                let title = link.value().attr("title")
                    .or_else(|| {
                        element.select(&Selector::parse("img").unwrap())
                            .next()
                            .and_then(|img| img.value().attr("alt"))
                    })
                    .unwrap_or("Unknown")
                    .to_string();
                
                // Extract poster URL
                let poster_url = element.select(&Selector::parse("img").unwrap())
                    .next()
                    .and_then(|img| img.value().attr("src"))
                    .map(|src| {
                        if src.starts_with("//") {
                            format!("https:{}", src)
                        } else if src.starts_with("/") {
                            format!("https://letterboxd.com{}", src)
                        } else {
                            src.to_string()
                        }
                    });
                
                // Try to extract year from title or URL
                let year_regex = Regex::new(r"\((\d{4})\)").unwrap();
                let year = year_regex.captures(&title)
                    .and_then(|cap| cap.get(1))
                    .and_then(|m| m.as_str().parse::<u16>().ok());

                favorites.push(FavoriteFilm {
                    title: title.replace(&format!(" ({})", year.unwrap_or(0)), "").trim().to_string(),
                    year,
                    poster_url,
                    letterboxd_url,
                });
            }
        }
        
        Ok(favorites)
    }
}