use clap::Parser;
use lbxd::{
    cli::{Cli, Commands},
    display::DisplayEngine,
    feed::FeedParser,
    cache::CacheManager,
    export::ExportManager,
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let display = DisplayEngine::new();
    let feed_parser = FeedParser::new();
    let export_manager = ExportManager::new();
    
    let cache_manager = match CacheManager::new() {
        Ok(cache) => Some(cache),
        Err(_) => {
            display.print_error("Warning: Could not initialize cache");
            None
        }
    };

    match cli.command {
        Commands::Recent { username, limit, date, rated, reviewed } => {
            display.print_ascii_art();
            
            let profile = if let Some(ref cache) = cache_manager {
                if let Some(cached) = cache.get_cached_profile(&username) {
                    cached
                } else {
                    match feed_parser.fetch_user_feed(&username).await {
                        Ok(profile) => {
                            let _ = cache.cache_profile(&profile);
                            profile
                        },
                        Err(e) => {
                            display.print_error(&format!("Failed to fetch user data: {}", e));
                            return;
                        }
                    }
                }
            } else {
                match feed_parser.fetch_user_feed(&username).await {
                    Ok(profile) => profile,
                    Err(e) => {
                        display.print_error(&format!("Failed to fetch user data: {}", e));
                        return;
                    }
                }
            };

            let filtered_profile = filter_entries(profile, date, rated, reviewed);
            display.show_user_activity(&filtered_profile, limit);
        },
        
        Commands::Search { username, title } => {
            display.print_ascii_art();
            
            match feed_parser.fetch_user_feed(&username).await {
                Ok(profile) => {
                    let matching_entries: Vec<_> = profile.entries
                        .iter()
                        .filter(|entry| entry.movie.title.to_lowercase().contains(&title.to_lowercase()))
                        .collect();
                    
                    if matching_entries.is_empty() {
                        display.print_error(&format!("No movies found matching '{}'", title));
                    } else {
                        display.print_success(&format!("Found {} matching entries:", matching_entries.len()));
                        for entry in matching_entries {
                            display.show_user_activity(&lbxd::models::UserProfile {
                                username: username.clone(),
                                display_name: profile.display_name.clone(),
                                avatar_url: None,
                                rss_url: profile.rss_url.clone(),
                                entries: vec![entry.clone()],
                            }, None);
                        }
                    }
                },
                Err(e) => {
                    display.print_error(&format!("Failed to fetch user data: {}", e));
                }
            }
        },
        
        Commands::Compare { usernames: _ } => {
            display.print_ascii_art();
            display.print_error("Compare feature coming soon!");
        },
        
        Commands::Export { username, format, output } => {
            match feed_parser.fetch_user_feed(&username).await {
                Ok(profile) => {
                    match export_manager.export_profile(&profile, &format, &output) {
                        Ok(_) => display.print_success(&format!("Data exported to {}", output)),
                        Err(e) => display.print_error(&format!("Export failed: {}", e)),
                    }
                },
                Err(e) => {
                    display.print_error(&format!("Failed to fetch user data: {}", e));
                }
            }
        },
        
        Commands::Summary { username: _, year: _ } => {
            display.print_ascii_art();
            display.print_error("Summary feature coming soon!");
        },
    }
}

fn filter_entries(
    mut profile: lbxd::models::UserProfile, 
    date_filter: Option<String>, 
    rated_only: bool, 
    reviewed_only: bool
) -> lbxd::models::UserProfile {
    profile.entries.retain(|entry| {
        if rated_only && entry.rating.is_none() {
            return false;
        }
        
        if reviewed_only && entry.review.is_none() {
            return false;
        }
        
        if let Some(ref date_str) = date_filter {
            if let Ok(filter_date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                if let Some(watched_date) = entry.watched_date {
                    let watched_naive = watched_date.date_naive();
                    if watched_naive != filter_date {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        
        true
    });
    
    profile
}
