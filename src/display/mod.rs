use crate::models::{UserEntry, UserProfile, ViewingSummary};
use colored::*;

pub struct DisplayEngine;

impl DisplayEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn show_user_activity(&self, profile: &UserProfile, limit: Option<usize>) {
        self.print_header(&format!("📽️  {} Activity", profile.username));
        
        let entries_to_show: Vec<_> = if let Some(limit) = limit {
            profile.entries.iter().take(limit).collect()
        } else {
            profile.entries.iter().collect()
        };

        for (i, entry) in entries_to_show.iter().enumerate() {
            self.print_entry(entry, i == 0);
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
        println!("\n{}", "═".repeat(60).dimmed());
        println!("{}", title.white().bold());
        println!("{}\n", "═".repeat(60).dimmed());
    }

    fn print_entry(&self, entry: &UserEntry, is_first: bool) {
        if !is_first {
            println!("{}", "─".repeat(50).dimmed());
        }

        let title_with_year = if let Some(year) = entry.movie.year {
            format!("{} ({})", entry.movie.title, year)
        } else {
            entry.movie.title.clone()
        };

        println!("  🎬 {}", title_with_year.white().bold());

        if let Some(rating) = entry.rating {
            let stars = "★".repeat(rating as usize);
            let half_star = if rating % 1.0 > 0.0 { "½" } else { "" };
            println!("  ⭐ {}{} ({:.1}/5)", stars.yellow(), half_star.yellow(), rating.to_string().yellow().bold());
        }

        if entry.liked {
            println!("  {} Liked", "♥".red());
        }

        if let Some(review) = &entry.review {
            let truncated = if review.len() > 150 {
                format!("{}...", &review[..150])
            } else {
                review.clone()
            };
            println!("  💭 {}", truncated.white());
        }

        if let Some(date) = entry.watched_date {
            println!("  📅 {}", date.format("%B %d, %Y").to_string().dimmed());
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

    pub fn print_error(&self, message: &str) {
        println!("{} {}", "✗".red().bold(), message.red());
    }

    pub fn print_success(&self, message: &str) {
        println!("{} {}", "✓".green().bold(), message.green());
    }
}