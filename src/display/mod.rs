use crate::models::{UserEntry, UserProfile, ViewingSummary};
use crate::ascii::AsciiConverter;
use crate::tmdb::{TMDBMovie, TMDBClient};
use colored::*;
use std::time::Duration;
use tokio::time::interval;
use terminal_size::{Width, Height, terminal_size};

pub struct DisplayEngine {
    ascii_converter: AsciiConverter,
}

impl DisplayEngine {
    pub fn new() -> Self {
        Self {
            ascii_converter: AsciiConverter::new(),
        }
    }

    pub async fn show_user_activity(&self, profile: &UserProfile, limit: Option<usize>, vertical: bool) {
        // Use the new activity header method (no "lbxd" logo for activity)
        self.print_activity_header(&profile.username);
        
        let entries_to_show: Vec<_> = if let Some(limit) = limit {
            profile.entries.iter().take(limit).collect()
        } else {
            profile.entries.iter().collect()
        };

        if vertical {
            for entry in entries_to_show.iter() {
                self.print_entry_with_ascii_vertical(entry).await;
            }
        } else {
            self.print_entries_horizontal_grid(&entries_to_show).await;
        }
    }

    pub fn show_summary(&self, summary: &ViewingSummary) {
        self.print_header(&format!("📊 {} - {} Summary", summary.username, summary.year));
        
        println!("  🎬 Total Movies: {}", summary.total_movies.to_string().cyan().bold());
        println!("  📝 Total Reviews: {}", summary.total_reviews.to_string().cyan().bold());
        
        if let Some(avg) = summary.average_rating {
            println!("  ⭐ Average Rating: {:.1}/5", avg.to_string().yellow().bold());
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
        println!("\n{} {}", AsciiConverter::create_letterboxd_logo(), title.white().bold());
        println!("{}", AsciiConverter::create_gradient_border(50, "─"));
        println!();
    }
    
    fn print_activity_header(&self, username: &str) {
        println!("\n{}", AsciiConverter::create_activity_header(username));
        println!("{}", AsciiConverter::create_gradient_border(50, "─"));
        println!();
    }


    async fn print_entry_with_ascii_vertical(&self, entry: &UserEntry) {
        let title_with_year = if let Some(year) = entry.movie.year {
            format!("{} ({})", entry.movie.title, year)
        } else {
            entry.movie.title.clone()
        };
        
        // Get dynamic poster size
        let term_width = if let Some((Width(w), Height(_))) = terminal_size() {
            w as u32
        } else {
            80
        };
        
        let (poster_width, _) = AsciiConverter::get_dynamic_poster_size(term_width);

        let ascii_art = if let Some(poster_url) = &entry.movie.poster_url {
            // Show loading animation for individual poster
            self.print_loading_animation(&format!("Loading poster for {}...", entry.movie.title), 200).await;
            
            if poster_url.starts_with("http") && (poster_url.contains(".jpg") || poster_url.contains(".png") || poster_url.contains(".jpeg") || poster_url.contains(".webp")) {
                match self.ascii_converter.convert_poster_to_ascii(poster_url, poster_width).await {
                    Ok(art) => art,
                    Err(e) => {
                        eprintln!("Failed to convert poster for {}: {}", entry.movie.title, e);
                        AsciiConverter::get_colored_fallback_poster_ascii(poster_width)
                    }
                }
            } else {
                AsciiConverter::get_colored_fallback_poster_ascii(poster_width)
            }
        } else {
            AsciiConverter::get_colored_fallback_poster_ascii(poster_width)
        };

        let lines: Vec<&str> = ascii_art.lines().collect();
        let max_lines = lines.len();

        println!("{}", AsciiConverter::create_gradient_border(80, "─"));
        println!();

        for (i, line) in lines.iter().enumerate() {
            print!("{:<32}", line.dimmed());
            
            if i == 0 {
                println!("{}", title_with_year.white().bold());
            } else if i == 2 {
                if let Some(rating) = entry.rating {
                    let stars = "★".repeat(rating as usize);
                    let half_star = if rating % 1.0 > 0.0 { "½" } else { "" };
                    println!("{}{} ({:.1}/5)", stars.yellow(), half_star.yellow(), rating.to_string().yellow().bold());
                } else {
                    println!();
                }
            } else if i == 4 && entry.liked {
                println!("{} Liked", "♥".red());
            } else if i == 6 {
                if let Some(review) = &entry.review {
                    let clean_review = self.clean_html(review);
                    if !clean_review.is_empty() {
                        let truncated = if clean_review.len() > 80 {
                            format!("{}...", &clean_review[..80])
                        } else {
                            clean_review
                        };
                        println!("{}", truncated.white());
                    } else {
                        println!();
                    }
                } else {
                    println!();
                }
            } else if i == max_lines - 2 {
                if let Some(date) = entry.watched_date {
                    println!("{}", date.format("%B %d, %Y").to_string().dimmed());
                } else {
                    println!();
                }
            } else {
                println!();
            }
        }
        
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

    async fn print_entries_horizontal_grid(&self, entries: &[&UserEntry]) {
        if entries.is_empty() {
            return;
        }

        let term_width = if let Some((Width(w), Height(_))) = terminal_size() {
            w as usize
        } else {
            80 // fallback width
        };

        // Get dynamic poster size for this terminal
        let (poster_width, _) = AsciiConverter::get_dynamic_poster_size(term_width as u32);
        
        // Calculate spacing: poster + padding + margin
        let column_width = poster_width as usize + 4; // 4 chars for spacing
        let posters_per_row = std::cmp::max(1, term_width / column_width);
        
        // Print with better spacing and organization
        for (chunk_idx, chunk) in entries.chunks(posters_per_row).enumerate() {
            if chunk_idx > 0 {
                // Add elegant row separator  
                println!("{}", AsciiConverter::create_gradient_border(term_width, "·"));
                println!();
            }
            
            self.print_poster_row(chunk).await;
            println!(); // spacing between rows
        }
    }

    async fn print_poster_row(&self, entries: &[&UserEntry]) {
        // Get dynamic poster size based on terminal width
        let term_width = if let Some((Width(w), Height(_))) = terminal_size() {
            w as u32
        } else {
            80
        };
        
        let (poster_width, _poster_height) = AsciiConverter::get_dynamic_poster_size(term_width);
        
        // Show loading animation for poster fetching
        if entries.len() > 1 {
            self.print_loading_animation(&format!("Loading {} posters...", entries.len()), 300).await;
        }
        
        // Collect all ASCII arts first
        let mut ascii_arts = Vec::new();
        for entry in entries {
            let ascii_art = if let Some(poster_url) = &entry.movie.poster_url {
                if poster_url.starts_with("http") && (poster_url.contains(".jpg") || poster_url.contains(".png") || poster_url.contains(".jpeg") || poster_url.contains(".webp")) {
                    match self.ascii_converter.convert_poster_to_ascii(poster_url, poster_width).await {
                        Ok(art) => art,
                        Err(e) => {
                            eprintln!("Failed to convert poster for {}: {}", entry.movie.title, e);
                            AsciiConverter::get_colored_fallback_poster_ascii(poster_width)
                        }
                    }
                } else {
                    AsciiConverter::get_colored_fallback_poster_ascii(poster_width)
                }
            } else {
                AsciiConverter::get_colored_fallback_poster_ascii(poster_width)
            };
            ascii_arts.push(ascii_art);
        }

        // Print titles
        for (i, entry) in entries.iter().enumerate() {
            let title_with_year = if let Some(year) = entry.movie.year {
                format!("{} ({})", entry.movie.title, year)
            } else {
                entry.movie.title.clone()
            };
            let max_title_width = poster_width as usize - 2;
            let truncated_title = if title_with_year.len() > max_title_width {
                format!("{}...", &title_with_year[..max_title_width.saturating_sub(3)])
            } else {
                title_with_year
            };
            print!("{:<width$}", truncated_title.white().bold(), width = poster_width as usize + 2);
            if i < entries.len() - 1 {
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
                    print!("{:<width$}", lines[line_idx], width = poster_width as usize + 2);
                } else {
                    print!("{:<width$}", "", width = poster_width as usize + 2);
                }
                if art_idx < ascii_arts.len() - 1 {
                    print!("  ");
                }
            }
            println!();
        }

        // Print ratings
        for (i, entry) in entries.iter().enumerate() {
            if let Some(rating) = entry.rating {
                let stars = "★".repeat(rating as usize);
                let half_star = if rating % 1.0 > 0.0 { "½" } else { "" };
                let rating_str = format!("{}{} ({:.1}/5)", stars, half_star, rating);
                let max_rating_width = poster_width as usize;
                let truncated_rating = if rating_str.len() > max_rating_width {
                    format!("{}...", &rating_str[..max_rating_width.saturating_sub(3)])
                } else {
                    rating_str
                };
                print!("{:<width$}", truncated_rating.yellow(), width = poster_width as usize + 2);
            } else {
                print!("{:<width$}", "", width = poster_width as usize + 2);
            }
            if i < entries.len() - 1 {
                print!("  ");
            }
        }
        println!();

        // Print dates
        for (i, entry) in entries.iter().enumerate() {
            if let Some(date) = entry.watched_date {
                let date_str = date.format("%B %d, %Y").to_string();
                let max_date_width = poster_width as usize;
                let truncated_date = if date_str.len() > max_date_width {
                    format!("{}...", &date_str[..max_date_width.saturating_sub(3)])
                } else {
                    date_str
                };
                print!("{:<width$}", truncated_date.dimmed(), width = poster_width as usize + 2);
            } else {
                print!("{:<width$}", "", width = poster_width as usize + 2);
            }
            if i < entries.len() - 1 {
                print!("  ");
            }
        }
        println!();
    }

    pub async fn show_tmdb_movie(&self, movie: &TMDBMovie) {
        self.print_minimal_header(&format!("Movie: {}", movie.title));
        
        let ascii_art = if let Some(poster_url) = movie.get_full_poster_url() {
            self.print_loading_animation("Fetching poster...", 500).await;
            match self.ascii_converter.convert_poster_to_ascii(&poster_url, 30).await {
                Ok(art) => art,
                Err(_) => AsciiConverter::get_colored_fallback_poster_ascii(30)
            }
        } else {
            AsciiConverter::get_colored_fallback_poster_ascii(30)
        };

        let lines: Vec<&str> = ascii_art.lines().collect();
        let max_lines = lines.len();

        println!("{}", AsciiConverter::create_gradient_border(80, "─"));
        println!();

        for (i, line) in lines.iter().enumerate() {
            print!("{:<32}", line.dimmed());
            
            if i == 0 {
                let title_with_year = if let Some(year) = movie.get_year() {
                    format!("{} ({})", movie.title, year)
                } else {
                    movie.title.clone()
                };
                println!("{}", title_with_year.white().bold());
            } else if i == 2 {
                if movie.vote_average > 0.0 {
                    println!("⭐ {:.1}/10 (TMDB)", movie.vote_average.to_string().yellow().bold());
                } else {
                    println!();
                }
            } else if i == 4 {
                if let Some(release_date) = &movie.release_date {
                    println!("📅 {}", release_date.dimmed());
                } else {
                    println!();
                }
            } else if i >= 6 && i < max_lines - 2 {
                if let Some(overview) = &movie.overview {
                    if !overview.is_empty() && i == 6 {
                        let truncated = if overview.len() > 80 {
                            format!("{}...", &overview[..80])
                        } else {
                            overview.clone()
                        };
                        println!("{}", truncated.white());
                    } else {
                        println!();
                    }
                } else {
                    println!();
                }
            } else {
                println!();
            }
        }
        
        println!();
        TMDBClient::print_tmdb_attribution();
    }
}