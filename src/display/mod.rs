use crate::models::{UserEntry, UserProfile, ViewingSummary};
use crate::ascii::AsciiConverter;
use crate::tmdb::{TMDBMovie, TMDBClient};
use crate::viu::ViuViewer;
use colored::*;
use std::time::Duration;
use tokio::time::interval;
use terminal_size::{Width, Height, terminal_size};
use regex::Regex;

pub struct DisplayEngine {
    ascii_converter: AsciiConverter,
    tmdb_client: TMDBClient,
    viu_viewer: ViuViewer,
}

impl DisplayEngine {
    pub fn new() -> Self {
        Self {
            ascii_converter: AsciiConverter::new(),
            tmdb_client: TMDBClient::new(),
            viu_viewer: ViuViewer::new(),
        }
    }

    pub async fn show_user_activity(&self, profile: &UserProfile, limit: Option<usize>, vertical: bool, ascii_mode: bool, width: u32) {
        // Use the new activity header method (no "lbxd" logo for activity)
        self.print_activity_header(&profile.username);
        
        let entries_to_show: Vec<_> = if let Some(limit) = limit {
            profile.entries.iter().take(limit).collect()
        } else {
            profile.entries.iter().collect()
        };

        if vertical {
            for entry in entries_to_show.iter() {
                self.display_entry_with_tmdb_lookup(entry, ascii_mode, width).await;
            }
        } else {
            self.display_entries_horizontal_grid_tmdb(&entries_to_show, ascii_mode, width).await;
        }
    }

    pub fn show_summary(&self, summary: &ViewingSummary) {
        self.print_header(&format!("📊 {} - {} Summary", summary.username, summary.year));
        
        println!("  🎬 Total Movies: {}", summary.total_movies.to_string().cyan().bold());
        println!("  📝 Total Reviews: {}", summary.total_reviews.to_string().cyan().bold());
        
        if let Some(avg) = summary.average_rating {
            println!("  ⭐ Average Rating: {:.1}/5", avg.to_string().color("#00d735").bold());
        }

        if !summary.top_movies.is_empty() {
            println!("\n  🏆 Top Rated Movies:");
            for (i, (movie, rating)) in summary.top_movies.iter().take(5).enumerate() {
                println!("    {}. {} {} - {:.1}★", 
                    (i + 1).to_string().green(),
                    movie.title.white().bold(),
                    movie.year.map(|y| format!("({})", y)).unwrap_or_default().dimmed(),
                    rating.to_string().yellow()
                );
            }
        }

        if !summary.favorite_directors.is_empty() {
            println!("\n  🎭 Favorite Directors:");
            for (director, count) in summary.favorite_directors.iter().take(3) {
                println!("    • {} - {} films", director.white().bold(), count.to_string().cyan());
            }
        }
    }

    fn print_header(&self, title: &str) {
        println!("\n{}", AsciiConverter::create_gradient_border(60, "═"));
        println!("{}", title.white().bold());
        println!("{}\n", AsciiConverter::create_gradient_border(60, "═"));
    }

    fn print_minimal_header(&self, title: &str) {
        println!("\n{} {}", AsciiConverter::create_colored_triple_stars(), title.white().bold());
        println!("{}", AsciiConverter::create_gradient_border(50, "─"));
        println!();
    }
    
    fn print_activity_header(&self, username: &str) {
        println!("\n{}", AsciiConverter::create_activity_header(username));
        println!("{}", AsciiConverter::create_gradient_border(50, "─"));
        println!();
    }



    pub fn print_ascii_art(&self) {
        let art = r#"
    ██╗     ██████╗ ██╗  ██╗██████╗ 
    ██║     ██╔══██╗╚██╗██╔╝██╔══██╗
    ██║     ██████╔╝ ╚███╔╝ ██║  ██║
    ██║     ██╔══██╗ ██╔██╗ ██║  ██║
    ███████╗██████╔╝██╔╝ ██╗██████╔╝
    ╚══════╝╚═════╝ ╚═╝  ╚═╝╚═════╝ 
        "#;
        
        println!("{}", art.cyan().bold());
        println!("{}", "Letterboxd in your terminal".dimmed());
    }

    pub fn print_minimal_logo(&self) {
        println!("{}", AsciiConverter::create_minimal_header());
    }

    pub fn print_error(&self, message: &str) {
        println!("{} {}", "✗".red().bold(), message.red());
    }

    pub fn print_success(&self, message: &str) {
        println!("{} {}", "✓".green().bold(), message.green());
    }

    pub fn print_warning(&self, message: &str) {
        println!("{} {}", "⚠".yellow().bold(), message.yellow());
    }

    pub fn print_info(&self, message: &str) {
        println!("{} {}", "ℹ".color("#06B4E4").bold(), message.color("#06B4E4"));
    }

    pub async fn print_loading_animation(&self, message: &str, duration_ms: u64) {
        let frames = ['·', '✢', '✳', '∗', '✻', '✽'];
        let mut frame_index = 0;
        let mut interval = interval(Duration::from_millis(100));
        
        let start_time = std::time::Instant::now();
        
        while start_time.elapsed().as_millis() < duration_ms as u128 {
            print!("\r{} {} ", frames[frame_index].to_string().yellow().bold(), message.yellow());
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            frame_index = (frame_index + 1) % frames.len();
            interval.tick().await;
        }
        
        print!("\r{} {}\n", "✓".green().bold(), message.green());
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
    }

    fn clean_html(&self, text: &str) -> String {
        use regex::Regex;
        
        // Remove HTML tags
        let re = Regex::new(r"<[^>]*>").unwrap();
        let no_tags = re.replace_all(text, "");
        
        // If the result is empty or only contains URLs/image references, return empty
        let cleaned = no_tags.replace("&nbsp;", " ")
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .trim()
            .to_string();
        
        // Check if this is just a URL or image reference
        if cleaned.starts_with("http") || cleaned.contains("src=") || cleaned.len() < 10 {
            return String::new();
        }
        
        cleaned
    }



    pub async fn show_tmdb_movie(&self, movie: &TMDBMovie, ascii_mode: bool, width: u32) {
        self.print_minimal_header(&format!("Movie: {}", movie.title));
        
        // Use the unified display function
        self.display_movie_with_poster(&movie.title, movie.get_year(), movie.get_full_poster_url(), Some(movie.vote_average), movie.release_date.as_ref(), movie.overview.as_ref(), None, None, None, ascii_mode, width).await;
        
        println!();
        TMDBClient::print_tmdb_attribution();
    }

    // Unified function to display a movie with poster and metadata
    pub async fn display_movie_with_poster(
        &self, 
        title: &str, 
        year: Option<i32>, 
        poster_url: Option<String>, 
        tmdb_rating: Option<f32>,
        release_date: Option<&String>,
        overview: Option<&String>,
        user_rating: Option<f32>,
        review: Option<&String>,
        watched_date: Option<chrono::DateTime<chrono::Utc>>,
        ascii_mode: bool,
        width: u32
    ) {
        if ascii_mode {
            // ASCII Art Mode
            let ascii_art = if let Some(url) = poster_url {
                self.print_loading_animation("Fetching poster...", 500).await;
                match self.ascii_converter.convert_poster_to_ascii(&url, width).await {
                    Ok((art, _aspect_ratio)) => art,
                    Err(_) => {
                        let (fallback_width, _) = AsciiConverter::get_optimal_poster_size(width, None);
                        AsciiConverter::get_colored_fallback_poster_ascii(fallback_width)
                    }
                }
            } else {
                let (fallback_width, _) = AsciiConverter::get_optimal_poster_size(width, None);
                AsciiConverter::get_colored_fallback_poster_ascii(fallback_width)
            };

            // Print ASCII art as a complete block without mixing with metadata
            println!("{}", AsciiConverter::create_gradient_border(80, "─"));
            println!();
            
            // Display the ASCII art cleanly
            println!("{}", ascii_art);
            
            println!();
            println!("{}", AsciiConverter::create_gradient_border(80, "─"));
        } else {
            // Viu Mode (default)
            if let Some(ref url) = poster_url {
                // Check if viu is available
                if ViuViewer::is_available() {
                    self.print_loading_animation("Loading poster...", 300).await;
                    match self.viu_viewer.display_image_url(&url, width).await {
                        Ok(_) => {
                            // viu successfully displayed the image
                            println!(); // Add some spacing after viu display
                        },
                        Err(_) => {
                            self.print_warning("Failed to display image, falling back to ASCII");
                            // Fallback to ASCII
                            match self.ascii_converter.convert_poster_to_ascii(&url, width).await {
                                Ok((art, _)) => println!("{}", art),
                                Err(_) => {
                                    let (fallback_width, _) = AsciiConverter::get_optimal_poster_size(width, None);
                                    println!("{}", AsciiConverter::get_colored_fallback_poster_ascii(fallback_width));
                                }
                            }
                        }
                    }
                } else {
                    self.print_warning("viu not found. Install viu for better image display or use --ascii flag");
                    println!("{}", ViuViewer::get_installation_instructions());
                    
                    // Fallback to ASCII
                    if let Some(url) = &poster_url {
                        match self.ascii_converter.convert_poster_to_ascii(url, width).await {
                            Ok((art, _)) => println!("{}", art),
                            Err(_) => {
                                let (fallback_width, _) = AsciiConverter::get_optimal_poster_size(width, None);
                                println!("{}", AsciiConverter::get_colored_fallback_poster_ascii(fallback_width));
                            }
                        }
                    }
                }
            } else {
                self.print_warning("No poster URL available");
            }
        }
        
        // Display movie metadata separately below the ASCII art
        let title_with_year = if let Some(year) = year {
            format!("{} ({})", title, year)
        } else {
            title.to_string()
        };
        println!("\n{}", title_with_year.white().bold());
        
        // Show user rating (Letterboxd style) with grey background stars and green filled stars
        if let Some(rating) = user_rating {
            let full_stars = rating as usize;
            let half_star = rating % 1.0 > 0.0;
            let mut rating_display = String::new();
            
            // Create 5 stars total, with filled stars for the rating
            for i in 0..5 {
                if i < full_stars {
                    rating_display.push_str(&"★".color("#00d735").bold().to_string());
                } else if i == full_stars && half_star {
                    rating_display.push_str(&"★".color("#00d735").bold().to_string());
                } else {
                    rating_display.push_str(&"★".truecolor(100, 100, 100).to_string());
                }
            }
            
            println!("{} ({:.1}/5)", rating_display, rating.to_string().color("#00d735").bold());
        }
        
        // Show TMDB rating if available and no user rating
        if user_rating.is_none() {
            if let Some(tmdb_rating) = tmdb_rating {
                if tmdb_rating > 0.0 {
                    println!("⭐ {:.1}/10 (TMDB)", tmdb_rating.to_string().color("#00d735").bold());
                }
            }
        }
        
        if let Some(date) = watched_date {
            println!("📅 {}", date.format("%B %d, %Y").to_string().dimmed());
        } else if let Some(release_date) = release_date {
            println!("📅 {}", release_date.dimmed());
        }
        
        // Show review if available
        if let Some(review_text) = review {
            let clean_review = self.clean_html(review_text);
            if !clean_review.is_empty() {
                println!("\n{}", clean_review.white());
            }
        } else if let Some(overview) = overview {
            if !overview.is_empty() {
                println!("\n{}", overview.white());
            }
        }
    }

    // New method to display an entry by fetching TMDB data like the movie command
    async fn display_entry_with_tmdb_lookup(&self, entry: &UserEntry, ascii_mode: bool, width: u32) {
        // Clean the title for better TMDB search results  
        let cleaned_title = self.clean_title_for_search(&entry.movie.title);
        
        // Create search query with year if available for better accuracy
        let search_query = if let Some(year) = entry.movie.year {
            format!("{} {}", cleaned_title, year)
        } else {
            cleaned_title.clone()
        };
        
        eprintln!("🔍 Searching TMDB for: '{}' (cleaned from: '{}')", search_query, entry.movie.title);
        
        // Search TMDB for the movie (exactly like movie command)
        match self.tmdb_client.search_movie(&search_query).await {
            Ok(Some(movie)) => {
                // Use the unified display function with user data
                self.display_movie_with_poster(
                    &entry.movie.title,
                    entry.movie.year,
                    movie.get_full_poster_url(),
                    Some(movie.vote_average),
                    movie.release_date.as_ref(),
                    movie.overview.as_ref(),
                    entry.rating,
                    entry.review.as_ref(),
                    entry.watched_date,
                    ascii_mode,
                    width
                ).await;
            },
            Ok(None) => {
                // Try searching without year if first search failed
                if entry.movie.year.is_some() {
                    eprintln!("🔍 Retrying TMDB search without year: '{}'", cleaned_title);
                    match self.tmdb_client.search_movie(&cleaned_title).await {
                        Ok(Some(movie)) => {
                            self.display_movie_with_poster(
                                &entry.movie.title,
                                entry.movie.year,
                                movie.get_full_poster_url(),
                                Some(movie.vote_average),
                                movie.release_date.as_ref(),
                                movie.overview.as_ref(),
                                entry.rating,
                                entry.review.as_ref(),
                                entry.watched_date,
                                ascii_mode,
                                width
                            ).await;
                        },
                        Ok(None) => {
                            eprintln!("❌ No TMDB results for '{}' (no year)", cleaned_title);
                            // Show without poster
                            self.display_movie_with_poster(
                                &entry.movie.title,
                                entry.movie.year,
                                None,
                                None,
                                None,
                                None,
                                entry.rating,
                                entry.review.as_ref(),
                                entry.watched_date,
                                ascii_mode,
                                width
                            ).await;
                        },
                        Err(e) => {
                            eprintln!("💥 TMDB API error for '{}' (no year): {}", cleaned_title, e);
                            // Show without poster
                            self.display_movie_with_poster(
                                &entry.movie.title,
                                entry.movie.year,
                                None,
                                None,
                                None,
                                None,
                                entry.rating,
                                entry.review.as_ref(),
                                entry.watched_date,
                                ascii_mode,
                                width
                            ).await;
                        }
                    }
                } else {
                    eprintln!("❌ No TMDB results for '{}'", search_query);
                    // Show without poster
                    self.display_movie_with_poster(
                        &entry.movie.title,
                        entry.movie.year,
                        None,
                        None,
                        None,
                        None,
                        entry.rating,
                        entry.review.as_ref(),
                        entry.watched_date,
                        ascii_mode,
                        width
                    ).await;
                }
            },
            Err(e) => {
                eprintln!("💥 TMDB API error for '{}': {}", search_query, e);
                // Show without poster
                self.display_movie_with_poster(
                    &entry.movie.title,
                    entry.movie.year,
                    None,
                    None,
                    None,
                    None,
                    entry.rating,
                    entry.review.as_ref(),
                    entry.watched_date,
                    ascii_mode,
                    width
                ).await;
            }
        }
    }

    fn clean_title_for_search(&self, title: &str) -> String {
        // Remove common problematic characters and patterns that might interfere with TMDB search
        let mut cleaned = title.to_string();
        
        // Remove trailing asterisks (like "Thunderbolts*")
        cleaned = cleaned.trim_end_matches('*').to_string();
        
        // Remove extra whitespace and normalize
        cleaned = cleaned.trim().to_string();
        
        // Replace multiple spaces with single space
        let re = Regex::new(r"\s+").unwrap();
        cleaned = re.replace_all(&cleaned, " ").to_string();
        
        cleaned
    }

    // Horizontal grid layout with TMDB integration
    async fn display_entries_horizontal_grid_tmdb(&self, entries: &[&UserEntry], ascii_mode: bool, width: u32) {
        if entries.is_empty() {
            return;
        }

        let term_width = if let Some((Width(w), Height(_))) = terminal_size() {
            w as usize
        } else {
            80 // fallback width
        };

        // Calculate spacing: poster + padding + margin  
        let column_width = width as usize + 4; // 4 chars for spacing
        let posters_per_row = std::cmp::max(1, term_width / column_width);
        
        // Print with better spacing and organization
        for (chunk_idx, chunk) in entries.chunks(posters_per_row).enumerate() {
            if chunk_idx > 0 {
                // Add elegant row separator  
                println!("{}", AsciiConverter::create_gradient_border(term_width, "·"));
                println!();
            }
            
            self.print_poster_row_tmdb(chunk, ascii_mode, width).await;
            println!(); // spacing between rows
        }
    }

    // Generate a row of posters using TMDB for each entry
    async fn print_poster_row_tmdb(&self, entries: &[&UserEntry], ascii_mode: bool, width: u32) {
        if ascii_mode {
            // ASCII mode: Use the original grid layout
            self.print_ascii_poster_row_tmdb(entries, width).await;
        } else {
            // viu mode: Display each poster individually using viu
            self.print_viu_poster_row_tmdb(entries, width).await;
        }
    }

    // ASCII grid layout (original implementation)
    async fn print_ascii_poster_row_tmdb(&self, entries: &[&UserEntry], width: u32) {
        // Show loading animation for poster fetching
        if entries.len() > 1 {
            self.print_loading_animation(&format!("Loading {} posters...", entries.len()), 300).await;
        }
        
        // Collect all movie data with TMDB lookups first
        let mut movie_data = Vec::new();
        for entry in entries {
            // Clean the title for better TMDB search results  
            let cleaned_title = self.clean_title_for_search(&entry.movie.title);
            
            // Create search query with year if available for better accuracy
            let search_query = if let Some(year) = entry.movie.year {
                format!("{} {}", cleaned_title, year)
            } else {
                cleaned_title.clone()
            };
            
            eprintln!("🔍 Searching TMDB for: '{}' (cleaned from: '{}')", search_query, entry.movie.title);
            
            // Search TMDB for the movie (exactly like vertical mode)
            let poster_url = match self.tmdb_client.search_movie(&search_query).await {
                Ok(Some(movie)) => movie.get_full_poster_url(),
                Ok(None) => {
                    // Try searching without year if first search failed
                    if entry.movie.year.is_some() {
                        eprintln!("🔍 Retrying TMDB search without year: '{}'", cleaned_title);
                        match self.tmdb_client.search_movie(&cleaned_title).await {
                            Ok(Some(movie)) => movie.get_full_poster_url(),
                            Ok(None) => {
                                eprintln!("❌ No TMDB results for '{}' (no year)", cleaned_title);
                                None
                            },
                            Err(e) => {
                                eprintln!("💥 TMDB API error for '{}' (no year): {}", cleaned_title, e);
                                None
                            }
                        }
                    } else {
                        eprintln!("❌ No TMDB results for '{}'", search_query);
                        None
                    }
                },
                Err(e) => {
                    eprintln!("💥 TMDB API error for '{}': {}", search_query, e);
                    None
                }
            };
            
            movie_data.push((entry, poster_url));
        }
        
        // Generate ASCII arts for all posters
        let mut ascii_arts = Vec::new();
        for (entry, poster_url) in &movie_data {
            let ascii_art = if let Some(url) = poster_url {
                match self.ascii_converter.convert_poster_to_ascii(url, width).await {
                    Ok((art, _aspect_ratio)) => art,
                    Err(e) => {
                        eprintln!("⚠ Failed to convert poster for '{}': {} (using fallback)", entry.movie.title, e);
                        let (fallback_width, _) = AsciiConverter::get_optimal_poster_size(width, None);
                        AsciiConverter::get_colored_fallback_poster_ascii(fallback_width)
                    }
                }
            } else {
                eprintln!("ℹ No poster URL found for '{},' (using fallback)", entry.movie.title);
                let (fallback_width, _) = AsciiConverter::get_optimal_poster_size(width, None);
                AsciiConverter::get_colored_fallback_poster_ascii(fallback_width)
            };
            ascii_arts.push(ascii_art);
        }

        // Print titles
        for (i, (entry, _)) in movie_data.iter().enumerate() {
            let title_with_year = if let Some(year) = entry.movie.year {
                format!("{} ({})", entry.movie.title, year)
            } else {
                entry.movie.title.clone()
            };
            let max_title_width = width as usize - 2;
            let truncated_title = if title_with_year.len() > max_title_width {
                format!("{}...", &title_with_year[..max_title_width.saturating_sub(3)])
            } else {
                title_with_year
            };
            print!("{:<width$}", truncated_title.white().bold(), width = width as usize + 2);
            if i < movie_data.len() - 1 {
                print!("  ");
            }
        }
        println!();

        // Print ASCII posters line by line
        let max_lines = ascii_arts.iter().map(|art| art.lines().count()).max().unwrap_or(0);
        for line_idx in 0..max_lines {
            for (art_idx, ascii_art) in ascii_arts.iter().enumerate() {
                let lines: Vec<&str> = ascii_art.lines().collect();
                if line_idx < lines.len() {
                    print!("{:<width$}", lines[line_idx], width = width as usize + 2);
                } else {
                    print!("{:<width$}", "", width = width as usize + 2);
                }
                if art_idx < ascii_arts.len() - 1 {
                    print!("  ");
                }
            }
            println!();
        }

        // Print ratings with grey background stars and green filled stars
        for (i, (entry, _)) in movie_data.iter().enumerate() {
            if let Some(rating) = entry.rating {
                let full_stars = rating as usize;
                let half_star = rating % 1.0 > 0.0;
                let mut rating_display = String::new();
                
                // Create 5 stars total, with filled stars for the rating
                for star_idx in 0..5 {
                    if star_idx < full_stars {
                        rating_display.push_str(&"★".color("#00d735").bold().to_string());
                    } else if star_idx == full_stars && half_star {
                        rating_display.push_str(&"★".color("#00d735").bold().to_string());
                    } else {
                        rating_display.push_str(&"★".truecolor(100, 100, 100).to_string());
                    }
                }
                
                let rating_str = format!("{} ({:.1})", rating_display, rating);
                let max_rating_width = width as usize + 10; // Account for ANSI codes
                let truncated_rating = if rating_str.chars().count() > max_rating_width {
                    format!("{}...", &rating_str[..max_rating_width.saturating_sub(3)])
                } else {
                    rating_str
                };
                print!("{:<width$}", truncated_rating, width = width as usize + 2);
            } else {
                print!("{:<width$}", "", width = width as usize + 2);
            }
            if i < movie_data.len() - 1 {
                print!("  ");
            }
        }
        println!();

        // Print dates
        for (i, (entry, _)) in movie_data.iter().enumerate() {
            if let Some(date) = entry.watched_date {
                let date_str = date.format("%B %d, %Y").to_string();
                let max_date_width = width as usize;
                let truncated_date = if date_str.len() > max_date_width {
                    format!("{}...", &date_str[..max_date_width.saturating_sub(3)])
                } else {
                    date_str
                };
                print!("{:<width$}", truncated_date.dimmed(), width = width as usize + 2);
            } else {
                print!("{:<width$}", "", width = width as usize + 2);
            }
            if i < movie_data.len() - 1 {
                print!("  ");
            }
        }
        println!();
    }

    // viu display for horizontal layout - display each poster individually
    async fn print_viu_poster_row_tmdb(&self, entries: &[&UserEntry], width: u32) {
        // Show loading animation for poster fetching
        if entries.len() > 1 {
            self.print_loading_animation(&format!("Loading {} posters...", entries.len()), 300).await;
        }
        
        // Display each entry individually using the unified display method (like vertical mode)
        for entry in entries {
            self.display_entry_with_tmdb_lookup(entry, false, width).await;
            println!(); // Add spacing between entries
        }
    }
}