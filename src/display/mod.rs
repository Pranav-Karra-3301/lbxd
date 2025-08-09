use crate::batch_loader::BatchLoader;
use crate::config::{ColorMode, ConfigManager, DisplayMode};
use crate::models::{UserEntry, UserProfile, ViewingSummary};
use crate::profile::ProfileStats;
use crate::tmdb::{TMDBClient, TMDBMovie};
use crate::viu::ViuViewer;
use colored::*;
use regex::Regex;
use std::time::Duration;
use terminal_size::{terminal_size, Height, Width};
use tokio::time::interval;

pub struct DisplayEngine {
    tmdb_client: TMDBClient,
    viu_viewer: ViuViewer,
}

impl Default for DisplayEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DisplayEngine {
    pub fn new() -> Self {
        Self {
            tmdb_client: TMDBClient::new(),
            viu_viewer: ViuViewer::new(),
        }
    }

    fn get_display_mode(&self) -> bool {
        ConfigManager::new()
            .map(|cm| {
                cm.get_display_mode().unwrap_or(DisplayMode::Pixelated) == DisplayMode::Pixelated
            })
            .unwrap_or(true)
    }

    fn get_color_mode(&self) -> ColorMode {
        ConfigManager::new()
            .map(|cm| cm.get_color_mode().unwrap_or(ColorMode::Color))
            .unwrap_or(ColorMode::Color)
    }

    fn apply_ansi_color(&self, text: &str, color: &str) -> String {
        match self.get_color_mode() {
            ColorMode::Color => match color {
                "red" => text.red().to_string(),
                "green" => text.green().to_string(),
                "yellow" => text.yellow().to_string(),
                "blue" => text.blue().to_string(),
                "magenta" => text.magenta().to_string(),
                "cyan" => text.cyan().to_string(),
                "white" => text.white().to_string(),
                "bright_red" => text.bright_red().to_string(),
                "bright_green" => text.bright_green().to_string(),
                "bright_yellow" => text.bright_yellow().to_string(),
                "bright_blue" => text.bright_blue().to_string(),
                "bright_magenta" => text.bright_magenta().to_string(),
                "bright_cyan" => text.bright_cyan().to_string(),
                "bright_white" => text.bright_white().to_string(),
                _ => text.normal().to_string(),
            },
            ColorMode::Grayscale => text.normal().to_string(),
        }
    }

    fn apply_style_with_ansi_color(&self, text: &str, style: &str, color: &str) -> String {
        match self.get_color_mode() {
            ColorMode::Color => {
                let colored_text = match color {
                    "red" => text.red(),
                    "green" => text.green(),
                    "yellow" => text.yellow(),
                    "blue" => text.blue(),
                    "magenta" => text.magenta(),
                    "cyan" => text.cyan(),
                    "white" => text.white(),
                    "bright_red" => text.bright_red(),
                    "bright_green" => text.bright_green(),
                    "bright_yellow" => text.bright_yellow(),
                    "bright_blue" => text.bright_blue(),
                    "bright_magenta" => text.bright_magenta(),
                    "bright_cyan" => text.bright_cyan(),
                    "bright_white" => text.bright_white(),
                    _ => text.normal(),
                };

                match style {
                    "bold" => colored_text.bold().to_string(),
                    "dimmed" => colored_text.dimmed().to_string(),
                    _ => colored_text.to_string(),
                }
            }
            ColorMode::Grayscale => match style {
                "bold" => text.bold().to_string(),
                "dimmed" => text.dimmed().to_string(),
                _ => text.normal().to_string(),
            },
        }
    }

    pub async fn show_user_activity(
        &self,
        profile: &UserProfile,
        limit: Option<usize>,
        vertical: bool,
        width: u32,
    ) {
        // Use the new activity header method
        self.print_activity_header(&profile.username);

        let entries_to_show: Vec<_> = if let Some(limit) = limit {
            profile.entries.iter().take(limit).collect()
        } else {
            profile.entries.iter().collect()
        };

        if vertical {
            for entry in entries_to_show.iter() {
                self.display_entry_with_tmdb_lookup(entry, width).await;
            }
        } else {
            self.display_entries_horizontal_grid_tmdb(&entries_to_show, width)
                .await;
        }
    }

    pub fn show_summary(&self, summary: &ViewingSummary) {
        self.print_header(&format!(
            "📊 {} - {} Summary",
            summary.username, summary.year
        ));

        println!(
            "  🎬 Total Movies: {}",
            summary.total_movies.to_string().cyan().bold()
        );
        println!(
            "  📝 Total Reviews: {}",
            summary.total_reviews.to_string().cyan().bold()
        );

        if let Some(avg) = summary.average_rating {
            let stars = self.rating_to_stars(avg);
            println!(
                "  ⭐ Average Rating: {} ({})",
                stars,
                avg.to_string().yellow()
            );
        }

        println!();
        self.print_footer();
    }

    pub fn print_header(&self, title: &str) {
        let width = if let Some((Width(w), Height(_))) = terminal_size() {
            w as usize
        } else {
            80
        };

        let border = "═".repeat(width);
        println!("{}", border.bright_cyan());
        println!("{}", title.bright_white().bold());
        println!("{}", border.bright_cyan());
        println!();
    }

    pub fn print_footer(&self) {
        println!();
        let width = if let Some((Width(w), Height(_))) = terminal_size() {
            w as usize
        } else {
            80
        };
        let border = "─".repeat(width);
        println!("{}", border.dimmed());
    }

    pub fn print_activity_header(&self, username: &str) {
        self.print_header(&format!("{} Activity", username));
    }

    pub fn print_error(&self, message: &str) {
        eprintln!("{} {}", "✗".red().bold(), message.red());
    }

    pub fn print_warning(&self, message: &str) {
        println!("{} {}", "⚠".yellow().bold(), message.yellow());
    }

    pub fn print_success(&self, message: &str) {
        println!("{} {}", "✓".green().bold(), message.green());
    }

    pub fn print_info(&self, message: &str) {
        println!("{} {}", "ℹ".blue().bold(), message.blue());
    }

    pub fn print_minimal_logo(&self) {
        println!("{}", "lbxd".bright_cyan().bold());
    }

    pub async fn print_loading_animation(&self, message: &str, duration_ms: u64) {
        print!("{} ", message.dimmed());
        let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let mut ticker = interval(Duration::from_millis(50));
        let start = std::time::Instant::now();

        while start.elapsed().as_millis() < duration_ms as u128 {
            for frame in &frames {
                print!("\r{} {} ", message.dimmed(), frame.cyan());
                ticker.tick().await;
                if start.elapsed().as_millis() >= duration_ms as u128 {
                    break;
                }
            }
        }
        print!("\r{}\r", " ".repeat(message.len() + 3));
    }

    fn rating_to_stars(&self, rating: f32) -> String {
        let full_stars = rating.floor() as usize;
        let has_half = rating - rating.floor() >= 0.5;
        let mut stars = "★".repeat(full_stars).yellow().to_string();
        if has_half {
            stars.push_str(&"☆".yellow().to_string());
        }
        let empty_stars = 5 - full_stars - if has_half { 1 } else { 0 };
        stars.push_str(&"☆".repeat(empty_stars).dimmed().to_string());
        stars
    }

    pub async fn show_profile_stats(&self, stats: &ProfileStats) {
        self.print_header(&format!("📊 Profile Overview - {}", stats.username));

        // Basic stats
        println!(
            "  {} Films watched",
            stats.total_films.to_string().cyan().bold()
        );
        println!(
            "  {} Films this year",
            stats.films_this_year.to_string().cyan().bold()
        );
        println!(
            "  {} Lists created",
            stats.lists_count.to_string().cyan().bold()
        );
        println!(
            "  {} Following | {} Followers",
            stats.following_count.to_string().cyan().bold(),
            stats.followers_count.to_string().cyan().bold()
        );

        // Display favorite films if available
        if !stats.favorite_films.is_empty() {
            println!();
            println!("{}", "Favorite Films:".bright_white().bold());
            for film in stats.favorite_films.iter().take(4) {
                println!("  • {}", film.title.cyan());
            }
        }

        self.print_footer();
    }

    pub async fn show_tmdb_movie(&self, movie: &TMDBMovie, width: u32) {
        self.print_header(&format!("🎬 {}", movie.title));

        // Display movie details with poster
        self.display_movie_with_poster(
            &movie.title,
            movie.get_year(),
            movie.get_full_poster_url(),
            Some(movie.vote_average),
            movie.release_date.as_ref(),
            movie.overview.as_ref(),
            None,
            None,
            None,
            width,
        )
        .await;

        println!();
        TMDBClient::print_tmdb_attribution();
    }

    // Unified function to display a movie with poster and metadata
    #[allow(clippy::too_many_arguments, clippy::needless_borrow)]
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
        width: u32,
    ) {
        // Always use viu for image display
        if let Some(ref url) = poster_url {
            // Check if viu is available
            if ViuViewer::is_available() {
                self.print_loading_animation("Loading poster...", 300).await;
                let use_pixelated = self.get_display_mode();
                if let Err(_) = self
                    .viu_viewer
                    .display_image_url(&url, width, use_pixelated)
                    .await
                {
                    self.print_warning("Failed to display image with viu");
                }
            } else {
                self.print_warning(&ViuViewer::get_installation_instructions());
            }
        }

        // Display movie metadata
        println!();
        println!("{}", title.bright_white().bold());
        if let Some(y) = year {
            println!("Year: {}", y.to_string().cyan());
        }

        // Display ratings
        if let Some(rating) = tmdb_rating {
            let stars = self.rating_to_stars(rating / 2.0);
            println!("TMDB: {} ({}/10)", stars, rating.to_string().yellow());
        }

        if let Some(rating) = user_rating {
            let stars = self.rating_to_stars(rating);
            println!("Your Rating: {}", stars);
        }

        // Display dates
        if let Some(date) = release_date {
            println!("Released: {}", date.dimmed());
        }

        if let Some(date) = watched_date {
            println!("Watched: {}", date.format("%B %d, %Y").to_string().dimmed());
        }

        // Display overview
        if let Some(text) = overview {
            println!();
            println!("{}", "Synopsis:".bright_white());
            let wrapped = self.wrap_text(text, 80);
            for line in wrapped {
                println!("  {}", line.dimmed());
            }
        }

        // Display review
        if let Some(text) = review {
            println!();
            println!("{}", "Review:".bright_white());
            let wrapped = self.wrap_text(text, 80);
            for line in wrapped {
                println!("  {}", line);
            }
        }
    }

    fn wrap_text(&self, text: &str, width: usize) -> Vec<String> {
        let words = text.split_whitespace();
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in words {
            if current_line.len() + word.len() + 1 > width {
                if !current_line.is_empty() {
                    lines.push(current_line.clone());
                    current_line.clear();
                }
            }
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }

    async fn display_entry_with_tmdb_lookup(&self, entry: &UserEntry, width: u32) {
        // Clean the title for better TMDB search results
        let cleaned_title = self.clean_title_for_search(&entry.movie.title);

        // Search TMDB for the movie using year as URL parameter
        match self
            .tmdb_client
            .search_movie_with_year(&cleaned_title, entry.movie.year)
            .await
        {
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
                    width,
                )
                .await;
            }
            Ok(None) => {
                // Try searching without year if first search failed
                if entry.movie.year.is_some() {
                    match self
                        .tmdb_client
                        .search_movie_with_year(&cleaned_title, None)
                        .await
                    {
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
                                width,
                            )
                            .await;
                        }
                        Ok(None) | Err(_) => {
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
                                width,
                            )
                            .await;
                        }
                    }
                } else {
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
                        width,
                    )
                    .await;
                }
            }
            Err(_) => {
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
                    width,
                )
                .await;
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
    async fn display_entries_horizontal_grid_tmdb(&self, entries: &[&UserEntry], width: u32) {
        if entries.is_empty() {
            return;
        }

        let term_width = if let Some((Width(w), Height(_))) = terminal_size() {
            w as usize
        } else {
            80 // fallback width
        };

        // Calculate how many posters can fit horizontally
        let poster_width = width as usize + 2; // Add padding
        let posters_per_row = (term_width / poster_width).max(1);

        // Process entries in chunks
        for chunk in entries.chunks(posters_per_row) {
            self.print_poster_row_tmdb(chunk, width).await;
            println!(); // Space between rows
        }
    }

    async fn print_poster_row_tmdb(&self, entries: &[&UserEntry], width: u32) {
        // Use viu to display posters side by side if possible
        // For now, display them vertically since viu doesn't support side-by-side easily
        for entry in entries {
            self.display_entry_with_tmdb_lookup(entry, width).await;
            println!();
        }
    }

    pub fn show_search_results(&self, results: Vec<UserEntry>) {
        if results.is_empty() {
            self.print_warning("No matching movies found");
            return;
        }

        self.print_header(&format!("🔍 Found {} matches", results.len()));

        for entry in results {
            println!("{}", entry.movie.title.bright_white().bold());
            if let Some(year) = entry.movie.year {
                print!(" ({})", year.to_string().dimmed());
            }
            if let Some(rating) = entry.rating {
                print!(" - {}", self.rating_to_stars(rating));
            }
            println!();

            if let Some(date) = entry.watched_date {
                println!(
                    "  Watched: {}",
                    date.format("%B %d, %Y").to_string().dimmed()
                );
            }

            if let Some(review) = &entry.review {
                let preview = if review.len() > 100 {
                    format!("{}...", &review[..100])
                } else {
                    review.clone()
                };
                println!("  Review: {}", preview.italic());
            }
            println!();
        }

        self.print_footer();
    }

    pub async fn search_with_poster(&self, results: Vec<UserEntry>, width: u32) {
        if results.is_empty() {
            self.print_warning("No matching movies found");
            return;
        }

        self.print_header(&format!("🔍 Found {} matches", results.len()));

        for result in results.iter() {
            self.display_entry_with_tmdb_lookup(result, width).await;
            println!();
        }

        self.print_footer();
    }
}
