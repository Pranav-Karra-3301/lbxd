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
    pub lists: Vec<UserList>,
    pub member_since: Option<String>,
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

    pub async fn scrape_comprehensive_profile(
        &self, 
        username: &str,
        progress_tx: Option<tokio::sync::mpsc::UnboundedSender<LoadingProgress>>
    ) -> Result<ComprehensiveProfile> {
        // Stage 1: Basic profile
        if let Some(ref tx) = progress_tx {
            let _ = tx.send(LoadingProgress {
                stage: LoadingStage::Profile,
                current: 0,
                total: 4,
                message: "Loading profile information...".to_string(),
            });
        }

        let basic_profile = self.scrape_profile(username).await?;
        
        // Stage 2: Film diary
        if let Some(ref tx) = progress_tx {
            let _ = tx.send(LoadingProgress {
                stage: LoadingStage::Diary,
                current: 1,
                total: 4,
                message: "Loading film diary...".to_string(),
            });
        }

        let diary_entries = self.scrape_film_diary(username, progress_tx.clone()).await?;
        
        // Stage 3: Lists
        if let Some(ref tx) = progress_tx {
            let _ = tx.send(LoadingProgress {
                stage: LoadingStage::Lists,
                current: 2,
                total: 4,
                message: "Loading user lists...".to_string(),
            });
        }

        let lists = self.scrape_user_lists(username, progress_tx.clone()).await?;
        
        // Stage 4: Complete
        if let Some(ref tx) = progress_tx {
            let _ = tx.send(LoadingProgress {
                stage: LoadingStage::Complete,
                current: 4,
                total: 4,
                message: "Processing complete!".to_string(),
            });
        }

        Ok(ComprehensiveProfile {
            name: basic_profile.name,
            username: basic_profile.username,
            avatar_url: basic_profile.avatar_url,
            bio: None, // TODO: Extract bio from profile page
            location: None, // TODO: Extract location
            website: None, // TODO: Extract website
            total_films: basic_profile.total_films,
            films_this_year: basic_profile.films_this_year,
            lists_count: basic_profile.lists_count,
            following_count: basic_profile.following_count,
            followers_count: basic_profile.followers_count,
            favorite_films: basic_profile.favorite_films,
            recent_activity: diary_entries[..std::cmp::min(10, diary_entries.len())].to_vec(),
            all_movies: diary_entries,
            lists,
            member_since: None, // TODO: Extract member since date
        })
    }

    async fn scrape_film_diary(
        &self, 
        username: &str,
        progress_tx: Option<tokio::sync::mpsc::UnboundedSender<LoadingProgress>>
    ) -> Result<Vec<UserMovieEntry>> {
        let mut all_entries = Vec::new();
        let mut page = 1;
        
        loop {
            let url = format!("https://letterboxd.com/{}/films/diary/page/{}/", username, page);
            
            // Rate limiting
            tokio::time::sleep(Duration::from_millis(1000)).await;
            
            let response = self.client.get(&url).send().await?;
            if response.status() == 404 {
                break; // No more pages
            }
            
            let html = response.text().await?;
            let document = Html::parse_document(&html);
            
            let entries = self.parse_diary_page(document)?;
            if entries.is_empty() {
                break; // No more entries
            }
            
            if let Some(ref tx) = progress_tx {
                let _ = tx.send(LoadingProgress {
                    stage: LoadingStage::Diary,
                    current: all_entries.len(),
                    total: all_entries.len() + entries.len(),
                    message: format!("Loading page {}...", page),
                });
            }
            
            all_entries.extend(entries);
            page += 1;
            
            // Limit to prevent excessive scraping
            if page > 20 || all_entries.len() > 1000 {
                break;
            }
        }
        
        Ok(all_entries)
    }

    fn parse_diary_page(&self, document: Html) -> Result<Vec<UserMovieEntry>> {
        let mut entries = Vec::new();
        let entry_selector = Selector::parse("tr.diary-entry-row").unwrap();
        
        for entry_element in document.select(&entry_selector) {
            // Extract movie title
            let title_selector = Selector::parse("h3.headline-3 a").unwrap();
            let title = entry_element
                .select(&title_selector)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .unwrap_or_default();
            
            if title.is_empty() {
                continue;
            }
            
            // Extract year
            let year_selector = Selector::parse(".film-title-wrapper small").unwrap();
            let year_text = entry_element
                .select(&year_selector)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join(" "))
                .unwrap_or_default();
            let year = Regex::new(r"(\d{4})")
                .unwrap()
                .captures(&year_text)
                .and_then(|cap| cap.get(1))
                .and_then(|m| m.as_str().parse::<u16>().ok());
            
            // Extract rating
            let rating_selector = Selector::parse(".rating .rating-green").unwrap();
            let rating = entry_element
                .select(&rating_selector)
                .next()
                .and_then(|el| el.value().attr("class"))
                .and_then(|class| {
                    let rating_match = Regex::new(r"rating-(\d+)")
                        .unwrap()
                        .captures(class)?;
                    let rating_num = rating_match.get(1)?.as_str().parse::<f32>().ok()?;
                    Some(rating_num / 2.0) // Convert from 10-point to 5-point scale
                });
            
            // Extract review if present
            let review_selector = Selector::parse(".body-text").unwrap();
            let review = entry_element
                .select(&review_selector)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .filter(|r| !r.is_empty());
            
            // Extract watch date
            let date_selector = Selector::parse("td.diary-day a").unwrap();
            let watched_date = entry_element
                .select(&date_selector)
                .next()
                .and_then(|el| {
                    let date_str = el.text().collect::<Vec<_>>().join(" ");
                    chrono::NaiveDate::parse_from_str(&date_str, "%d %b %Y")
                        .ok()?
                        .and_hms_opt(0, 0, 0)?
                        .and_utc()
                        .into()
                });
            
            // Check if liked
            let like_selector = Selector::parse(".icon-liked").unwrap();
            let liked = entry_element.select(&like_selector).next().is_some();
            
            // Extract movie URL
            let url_selector = Selector::parse("h3.headline-3 a").unwrap();
            let letterboxd_url = entry_element
                .select(&url_selector)
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|href| {
                    if href.starts_with("/") {
                        format!("https://letterboxd.com{}", href)
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();
            
            let movie = DetailedMovie {
                title,
                year,
                director: None, // Will be filled in later if needed
                genres: Vec::new(), // Will be filled in later if needed
                runtime: None,
                poster_url: None, // Will be filled in later if needed
                letterboxd_url,
                tmdb_url: None,
                cast: Vec::new(),
                synopsis: None,
                letterboxd_rating: None,
            };
            
            entries.push(UserMovieEntry {
                movie,
                user_rating: rating,
                review,
                watched_date,
                liked,
                rewatched: false, // TODO: Detect rewatches
                tags: Vec::new(), // TODO: Extract tags
            });
        }
        
        Ok(entries)
    }

    async fn scrape_user_lists(
        &self, 
        username: &str,
        _progress_tx: Option<tokio::sync::mpsc::UnboundedSender<LoadingProgress>>
    ) -> Result<Vec<UserList>> {
        let url = format!("https://letterboxd.com/{}/lists/", username);
        
        // Rate limiting
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            return Ok(Vec::new()); // Return empty if lists page is not accessible
        }
        
        let html = response.text().await?;
        let document = Html::parse_document(&html);
        
        let mut lists = Vec::new();
        let list_selector = Selector::parse(".film-list-summary").unwrap();
        
        for list_element in document.select(&list_selector).take(5) { // Limit to 5 lists
            let name_selector = Selector::parse("h2 a").unwrap();
            let name = list_element
                .select(&name_selector)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .unwrap_or_default();
            
            if name.is_empty() {
                continue;
            }
            
            let url_selector = Selector::parse("h2 a").unwrap();
            let list_url = list_element
                .select(&url_selector)
                .next()
                .and_then(|el| el.value().attr("href"))
                .map(|href| {
                    if href.starts_with("/") {
                        format!("https://letterboxd.com{}", href)
                    } else {
                        href.to_string()
                    }
                })
                .unwrap_or_default();
            
            lists.push(UserList {
                name,
                description: None, // TODO: Extract description
                url: list_url,
                movies: Vec::new(), // TODO: Load list contents
                is_public: true, // Assume public if we can access it
                created_date: None,
            });
        }
        
        Ok(lists)
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
        let favorite_films = self.extract_favorite_films(document)?;

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

    fn extract_favorite_films(&self, document: Html) -> Result<Vec<FavoriteFilm>> {
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