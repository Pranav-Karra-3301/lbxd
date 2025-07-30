use crate::models::{UserProfile, ViewingSummary};
use crate::cli::ExportFormat;
use anyhow::Result;
use serde_json;
use std::fs;

pub struct ExportManager;

impl ExportManager {
    pub fn new() -> Self {
        Self
    }

    pub fn export_profile(&self, profile: &UserProfile, format: &ExportFormat, output_path: &str) -> Result<()> {
        match format {
            ExportFormat::Json => self.export_json(profile, output_path),
            ExportFormat::Markdown => self.export_markdown(profile, output_path),
        }
    }

    pub fn export_summary(&self, summary: &ViewingSummary, format: &ExportFormat, output_path: &str) -> Result<()> {
        match format {
            ExportFormat::Json => {
                let content = serde_json::to_string_pretty(summary)?;
                fs::write(output_path, content)?;
                Ok(())
            },
            ExportFormat::Markdown => self.export_summary_markdown(summary, output_path),
        }
    }

    fn export_json(&self, profile: &UserProfile, output_path: &str) -> Result<()> {
        let content = serde_json::to_string_pretty(profile)?;
        fs::write(output_path, content)?;
        Ok(())
    }

    fn export_markdown(&self, profile: &UserProfile, output_path: &str) -> Result<()> {
        let mut content = String::new();
        
        content.push_str(&format!("# {} - Letterboxd Activity\n\n", profile.username));
        
        if let Some(display_name) = &profile.display_name {
            content.push_str(&format!("**Display Name:** {}\n\n", display_name));
        }
        
        content.push_str("## Recent Activity\n\n");
        
        for entry in &profile.entries {
            let title_with_year = if let Some(year) = entry.movie.year {
                format!("{} ({})", entry.movie.title, year)
            } else {
                entry.movie.title.clone()
            };
            
            content.push_str(&format!("### {}\n\n", title_with_year));
            
            if let Some(rating) = entry.rating {
                let stars = "★".repeat(rating as usize);
                let half_star = if rating % 1.0 > 0.0 { "½" } else { "" };
                content.push_str(&format!("**Rating:** {}{} ({:.1}/5)\n\n", stars, half_star, rating));
            }
            
            if entry.liked {
                content.push_str("❤️ **Liked**\n\n");
            }
            
            if let Some(review) = &entry.review {
                content.push_str(&format!("**Review:**\n{}\n\n", review));
            }
            
            if let Some(date) = entry.watched_date {
                content.push_str(&format!("**Date:** {}\n\n", date.format("%B %d, %Y")));
            }
            
            content.push_str(&format!("[View on Letterboxd]({})\n\n", entry.movie.letterboxd_url));
            content.push_str("---\n\n");
        }
        
        fs::write(output_path, content)?;
        Ok(())
    }

    fn export_summary_markdown(&self, summary: &ViewingSummary, output_path: &str) -> Result<()> {
        let mut content = String::new();
        
        content.push_str(&format!("# {} - {} Summary\n\n", summary.username, summary.year));
        
        content.push_str("## Statistics\n\n");
        content.push_str(&format!("- **Total Movies:** {}\n", summary.total_movies));
        content.push_str(&format!("- **Total Reviews:** {}\n", summary.total_reviews));
        
        if let Some(avg) = summary.average_rating {
            content.push_str(&format!("- **Average Rating:** {:.1}/5\n", avg));
        }
        
        content.push_str("\n## Top Rated Movies\n\n");
        for (i, (movie, rating)) in summary.top_movies.iter().enumerate() {
            let title_with_year = if let Some(year) = movie.year {
                format!("{} ({})", movie.title, year)
            } else {
                movie.title.clone()
            };
            content.push_str(&format!("{}. {} - {:.1}★\n", i + 1, title_with_year, rating));
        }
        
        if !summary.favorite_directors.is_empty() {
            content.push_str("\n## Favorite Directors\n\n");
            for (director, count) in &summary.favorite_directors {
                content.push_str(&format!("- {} ({} films)\n", director, count));
            }
        }
        
        fs::write(output_path, content)?;
        Ok(())
    }
}