#![allow(clippy::uninlined_format_args)]
#![allow(clippy::redundant_pattern_matching)]

use chrono::Datelike;
use clap::Parser;
use colored::Colorize;
use lbxd::{
    cache::CacheManager,
    cli::{Cli, ColorModeArg, Commands, ConfigCommands, DisplayModeArg},
    config::{ColorMode, ConfigManager, DisplayMode},
    display::DisplayEngine,
    export::ExportManager,
    feed::FeedParser,
    letterboxd_client_rust::LetterboxdClient,
    onboarding::OnboardingManager,
    tmdb::TMDBClient,
    tui,
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let display = DisplayEngine::new();
    let feed_parser = FeedParser::new();
    let export_manager = ExportManager::new();

    let mut config_manager = match ConfigManager::new() {
        Ok(config) => config,
        Err(_) => {
            display.print_error("Error: Could not initialize configuration");
            return;
        }
    };

    // Run onboarding for first-time users or when --reconfig is used
    if config_manager.is_first_run() || cli.reconfig {
        let onboarding = OnboardingManager::new(config_manager);
        if let Err(e) = onboarding.run_interactive_setup().await {
            display.print_error(&format!("Setup failed: {}", e));
            return;
        }

        // Reload config manager after onboarding
        config_manager = match ConfigManager::new() {
            Ok(config) => config,
            Err(_) => {
                display.print_error("Error: Could not reload configuration after setup");
                return;
            }
        };

        // If only --reconfig was used (no subcommand), exit after setup
        if cli.reconfig && cli.command.is_none() {
            return;
        }
    }

    let cache_manager = match CacheManager::new() {
        Ok(cache) => Some(cache),
        Err(_) => {
            display.print_error("Warning: Could not initialize cache");
            None
        }
    };

    // Handle case where no command is provided but username is given (profile stats)
    let command = match cli.command {
        Some(cmd) => Some(cmd),
        None => {
            if let Some(username) = cli.username {
                // Show profile stats for the given username
                let actual_username = resolve_username(&username, &config_manager, &display).await;
                if let Some(actual_username) = actual_username {
                    display.print_minimal_logo();

                    match LetterboxdClient::new() {
                        Ok(client) => {
                            display
                                .print_loading_animation("Fetching profile stats...", 1000)
                                .await;

                            match client
                                .get_comprehensive_profile(&actual_username, None)
                                .await
                            {
                                Ok(comprehensive_profile) => {
                                    // Convert to basic profile stats for display
                                    let profile_stats = lbxd::profile::ProfileStats {
                                        name: comprehensive_profile.name,
                                        username: comprehensive_profile.username,
                                        avatar_url: None, // No avatar support
                                        total_films: comprehensive_profile.total_films,
                                        films_this_year: comprehensive_profile.films_this_year,
                                        lists_count: comprehensive_profile.lists_count,
                                        following_count: comprehensive_profile.following_count,
                                        followers_count: comprehensive_profile.followers_count,
                                        favorite_films: comprehensive_profile.favorite_films,
                                    };
                                    display.show_profile_stats(&profile_stats).await;
                                }
                                Err(e) => {
                                    display.print_error(&format!(
                                        "Failed to fetch profile stats: {}",
                                        e
                                    ));
                                }
                            }
                        }
                        Err(e) => {
                            display.print_error(&format!(
                                "Failed to initialize Letterboxd client: {}",
                                e
                            ));
                        }
                    }
                }
                return;
            } else if !cli.reconfig {
                // No command, no username, and no --reconfig, show version info
                show_version_info();
                return;
            } else {
                return;
            }
        }
    };

    // Handle subcommands
    let command = match command {
        Some(cmd) => cmd,
        None => return,
    };

    match command {
        Commands::Recent {
            username,
            limit,
            date,
            rated,
            reviewed,
            vertical,
            width,
        } => {
            let actual_username = resolve_username(&username, &config_manager, &display).await;
            if actual_username.is_none() {
                return;
            }
            let actual_username = actual_username.unwrap();

            display.print_minimal_logo();

            let profile = if let Some(ref cache) = cache_manager {
                if let Some(cached) = cache.get_cached_profile(&actual_username) {
                    cached
                } else {
                    match feed_parser.fetch_user_feed(&actual_username).await {
                        Ok(profile) => {
                            let _ = cache.cache_profile(&profile);
                            profile
                        }
                        Err(e) => {
                            display.print_error(&format!("Failed to fetch user data: {}", e));
                            return;
                        }
                    }
                }
            } else {
                match feed_parser.fetch_user_feed(&actual_username).await {
                    Ok(profile) => profile,
                    Err(e) => {
                        display.print_error(&format!("Failed to fetch user data: {}", e));
                        return;
                    }
                }
            };

            let filtered_profile = filter_entries(profile, date, rated, reviewed);
            display
                .show_user_activity(&filtered_profile, limit, vertical, width)
                .await;
        }

        Commands::Search {
            username,
            title,
            width,
        } => {
            let actual_username = resolve_username(&username, &config_manager, &display).await;
            if actual_username.is_none() {
                return;
            }
            let actual_username = actual_username.unwrap();

            display.print_minimal_logo();

            match feed_parser.fetch_user_feed(&actual_username).await {
                Ok(profile) => {
                    let matching_entries: Vec<_> = profile
                        .entries
                        .iter()
                        .filter(|entry| {
                            entry
                                .movie
                                .title
                                .to_lowercase()
                                .contains(&title.to_lowercase())
                        })
                        .collect();

                    if matching_entries.is_empty() {
                        display.print_error(&format!("No movies found matching '{}'", title));
                    } else {
                        display.print_success(&format!(
                            "Found {} matching entries:",
                            matching_entries.len()
                        ));
                        for entry in matching_entries {
                            display
                                .show_user_activity(
                                    &lbxd::models::UserProfile {
                                        username: actual_username.clone(),
                                        display_name: profile.display_name.clone(),
                                        avatar_url: None,
                                        rss_url: profile.rss_url.clone(),
                                        entries: vec![entry.clone()],
                                    },
                                    None,
                                    true,
                                    width,
                                )
                                .await; // Default to vertical for search results
                        }
                    }
                }
                Err(e) => {
                    display.print_error(&format!("Failed to fetch user data: {}", e));
                }
            }
        }

        Commands::Compare { usernames } => {
            display.print_minimal_logo();

            if usernames.len() < 2 {
                display.print_error("Please provide at least 2 usernames to compare");
                return;
            }

            display
                .print_loading_animation("Fetching user profiles...", 500)
                .await;

            println!();
            println!(
                "{}",
                "═══════════════════════════════════════════════════════════".green()
            );
            println!(
                "{}",
                "                     📊 User Comparison                     ".bright_white()
            );
            println!(
                "{}",
                "═══════════════════════════════════════════════════════════".green()
            );
            println!();

            let mut profiles_data: Vec<(String, usize, f32, usize)> = Vec::new();

            for username in &usernames {
                match feed_parser.fetch_user_feed(username).await {
                    Ok(profile) => {
                        let total_films = profile.entries.len();
                        let rated_films: Vec<_> =
                            profile.entries.iter().filter_map(|e| e.rating).collect();
                        let avg_rating = if !rated_films.is_empty() {
                            rated_films.iter().sum::<f32>() / rated_films.len() as f32
                        } else {
                            0.0
                        };
                        let reviews = profile
                            .entries
                            .iter()
                            .filter(|e| e.review.is_some())
                            .count();

                        profiles_data.push((username.clone(), total_films, avg_rating, reviews));
                    }
                    Err(e) => {
                        display.print_warning(&format!(
                            "Could not fetch data for {}: {}",
                            username, e
                        ));
                    }
                }
            }

            if profiles_data.is_empty() {
                display.print_error("Could not fetch any user data");
                return;
            }

            // Print comparison table
            println!(
                "  {:<20} {:>12} {:>12} {:>12}",
                "Username".bright_cyan(),
                "Films".bright_cyan(),
                "Avg Rating".bright_cyan(),
                "Reviews".bright_cyan()
            );
            println!("  {}", "─".repeat(58));

            for (username, films, avg_rating, reviews) in &profiles_data {
                let rating_str = if *avg_rating > 0.0 {
                    format!("{:.1}★", avg_rating)
                } else {
                    "N/A".to_string()
                };
                println!(
                    "  {:<20} {:>12} {:>12} {:>12}",
                    username.bright_white(),
                    films.to_string().yellow(),
                    rating_str.green(),
                    reviews.to_string().blue()
                );
            }

            println!();

            // Find who has the most films
            if let Some(max_films) = profiles_data.iter().max_by_key(|p| p.1) {
                println!(
                    "  🏆 {} has watched the most films ({})!",
                    max_films.0.bright_yellow(),
                    max_films.1
                );
            }

            // Find highest average rating
            let rated_users: Vec<_> = profiles_data.iter().filter(|p| p.2 > 0.0).collect();
            if let Some(max_rating) = rated_users
                .iter()
                .max_by(|a, b| a.2.partial_cmp(&b.2).unwrap())
            {
                println!(
                    "  ⭐ {} has the highest average rating ({:.1}★)!",
                    max_rating.0.bright_yellow(),
                    max_rating.2
                );
            }

            println!();
        }

        Commands::Export {
            username,
            format,
            output,
        } => {
            let actual_username = resolve_username(&username, &config_manager, &display).await;
            if actual_username.is_none() {
                return;
            }
            let actual_username = actual_username.unwrap();

            match feed_parser.fetch_user_feed(&actual_username).await {
                Ok(profile) => match export_manager.export_profile(&profile, &format, &output) {
                    Ok(_) => display.print_success(&format!("Data exported to {}", output)),
                    Err(e) => display.print_error(&format!("Export failed: {}", e)),
                },
                Err(e) => {
                    display.print_error(&format!("Failed to fetch user data: {}", e));
                }
            }
        }

        Commands::Summary { username, year } => {
            let actual_username = resolve_username(&username, &config_manager, &display).await;
            if actual_username.is_none() {
                return;
            }
            let actual_username = actual_username.unwrap();

            display.print_minimal_logo();
            display
                .print_loading_animation("Generating summary...", 500)
                .await;

            let target_year = year.unwrap_or_else(|| chrono::Utc::now().year());

            match feed_parser.fetch_user_feed(&actual_username).await {
                Ok(profile) => {
                    // Filter entries for the target year
                    let year_entries: Vec<_> = profile
                        .entries
                        .iter()
                        .filter(|e| {
                            e.watched_date
                                .map(|d| d.year() == target_year)
                                .unwrap_or(false)
                        })
                        .collect();

                    println!();
                    println!(
                        "{}",
                        "═══════════════════════════════════════════════════════════".green()
                    );
                    println!(
                        "  {} {} - {} Summary",
                        "📊".bright_white(),
                        actual_username.bright_green(),
                        target_year.to_string().bright_yellow()
                    );
                    println!(
                        "{}",
                        "═══════════════════════════════════════════════════════════".green()
                    );
                    println!();

                    if year_entries.is_empty() {
                        println!("  {} No films found for {}", "ℹ".blue(), target_year);
                        println!();
                        return;
                    }

                    // Calculate stats
                    let total_films = year_entries.len();
                    let rated_films: Vec<_> =
                        year_entries.iter().filter_map(|e| e.rating).collect();
                    let avg_rating = if !rated_films.is_empty() {
                        rated_films.iter().sum::<f32>() / rated_films.len() as f32
                    } else {
                        0.0
                    };
                    let reviews = year_entries.iter().filter(|e| e.review.is_some()).count();
                    let liked = year_entries.iter().filter(|e| e.liked).count();

                    // Stats section
                    println!("  {} {}", "📈".bright_white(), "Statistics".bright_cyan());
                    println!("  {}", "─".repeat(40));
                    println!(
                        "  🎬 Total Films Watched: {}",
                        total_films.to_string().bright_yellow()
                    );
                    if avg_rating > 0.0 {
                        let stars = "★".repeat(avg_rating as usize);
                        let half = if avg_rating % 1.0 >= 0.5 { "½" } else { "" };
                        println!(
                            "  ⭐ Average Rating: {}{} ({:.1}/5)",
                            stars.bright_yellow(),
                            half.bright_yellow(),
                            avg_rating
                        );
                    }
                    println!(
                        "  📝 Reviews Written: {}",
                        reviews.to_string().bright_blue()
                    );
                    println!("  ❤️  Films Liked: {}", liked.to_string().bright_red());

                    // Top rated films
                    let mut top_rated: Vec<_> =
                        year_entries.iter().filter(|e| e.rating.is_some()).collect();
                    top_rated.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap());

                    if !top_rated.is_empty() {
                        println!();
                        println!(
                            "  {} {}",
                            "🏆".bright_white(),
                            "Top Rated Films".bright_cyan()
                        );
                        println!("  {}", "─".repeat(40));
                        for (i, entry) in top_rated.iter().take(5).enumerate() {
                            let year_str = entry
                                .movie
                                .year
                                .map(|y| format!(" ({})", y))
                                .unwrap_or_default();
                            let stars = "★".repeat(entry.rating.unwrap() as usize);
                            println!(
                                "  {}. {}{} - {}",
                                i + 1,
                                entry.movie.title.bright_white(),
                                year_str.dimmed(),
                                stars.bright_yellow()
                            );
                        }
                    }

                    // Monthly breakdown
                    println!();
                    println!(
                        "  {} {}",
                        "📅".bright_white(),
                        "Monthly Breakdown".bright_cyan()
                    );
                    println!("  {}", "─".repeat(40));

                    let mut monthly_counts: std::collections::HashMap<u32, usize> =
                        std::collections::HashMap::new();
                    for entry in &year_entries {
                        if let Some(date) = entry.watched_date {
                            *monthly_counts.entry(date.month()).or_insert(0) += 1;
                        }
                    }

                    let months = [
                        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct",
                        "Nov", "Dec",
                    ];
                    for (i, month) in months.iter().enumerate() {
                        let count = monthly_counts.get(&(i as u32 + 1)).unwrap_or(&0);
                        let bar = "█".repeat(std::cmp::min(*count, 20));
                        if *count > 0 {
                            println!("  {} {:>3} {}", month, count, bar.bright_green());
                        }
                    }

                    println!();
                }
                Err(e) => {
                    display.print_error(&format!("Failed to fetch user data: {}", e));
                }
            }
        }

        Commands::Movie { title, width } => {
            display.print_minimal_logo();

            let tmdb_client = TMDBClient::new();
            display
                .print_loading_animation("Searching TMDB...", 1000)
                .await;

            match tmdb_client.search_movie(&title).await {
                Ok(Some(movie)) => {
                    display.show_tmdb_movie(&movie, width).await;
                }
                Ok(None) => {
                    display.print_error(&format!("No movies found for '{}'", title));
                }
                Err(e) => {
                    display.print_error(&format!("Failed to search TMDB: {}", e));
                }
            }
        }

        Commands::Config { config_command } => {
            display.print_minimal_logo();

            match config_command {
                ConfigCommands::Whoami => match config_manager.get_username() {
                    Ok(Some(username)) => {
                        display.print_success(&format!("Current username: {}", username));
                    }
                    Ok(None) => {
                        display.print_warning("No username is currently saved");
                    }
                    Err(e) => {
                        display.print_error(&format!("Failed to read config: {}", e));
                    }
                },
                ConfigCommands::SetUser { username } => {
                    match config_manager.change_username(username.clone()) {
                        Ok(_) => {
                            display.print_success(&format!("Username set to: {}", username));
                        }
                        Err(e) => {
                            display.print_error(&format!("Failed to save username: {}", e));
                        }
                    }
                }
                ConfigCommands::Show => match config_manager.get_all_config() {
                    Ok(config) => {
                        display.print_info("Current Configuration:");
                        println!(
                            "  Username: {}",
                            config.username.unwrap_or_else(|| "Not set".to_string())
                        );
                        println!("  Color mode: {:?}", config.color_mode);
                        println!("  Display mode: {:?}", config.display_mode);
                    }
                    Err(e) => {
                        display.print_error(&format!("Failed to read config: {}", e));
                    }
                },
                ConfigCommands::SwitchColor { mode } => {
                    let color_mode = match mode {
                        ColorModeArg::Color => ColorMode::Color,
                        ColorModeArg::Grayscale => ColorMode::Grayscale,
                    };
                    match config_manager.set_color_mode(color_mode) {
                        Ok(_) => {
                            display.print_success(&format!("Color mode switched to: {:?}", mode));
                        }
                        Err(e) => {
                            display.print_error(&format!("Failed to update color mode: {}", e));
                        }
                    }
                }
                ConfigCommands::SetMode { mode } => {
                    let display_mode = match mode {
                        DisplayModeArg::Pixelated => DisplayMode::Pixelated,
                        DisplayModeArg::Full => DisplayMode::FullResolution,
                    };
                    match config_manager.set_display_mode(display_mode) {
                        Ok(_) => {
                            display.print_success(&format!("Display mode set to: {:?}", mode));
                        }
                        Err(e) => {
                            display.print_error(&format!("Failed to update display mode: {}", e));
                        }
                    }
                }
                ConfigCommands::ClearCache => {
                    if let Some(ref cache) = cache_manager {
                        match cache.clear_cache() {
                            Ok(_) => {
                                display.print_success("Cache cleared successfully");
                            }
                            Err(e) => {
                                display.print_error(&format!("Failed to clear cache: {}", e));
                            }
                        }
                    } else {
                        display.print_warning("Cache manager not available");
                    }
                }
                ConfigCommands::Paths => {
                    let home_dir = dirs::home_dir()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| "~".to_string());
                    display.print_info("File Locations:");
                    println!("  Config: {}/.config/lbxd/config.json", home_dir);
                    println!("  Cache:  {}/.cache/lbxd/", home_dir);
                }
            }
        }

        Commands::Browse { username } => {
            let actual_username = resolve_username(&username, &config_manager, &display).await;
            if actual_username.is_none() {
                return;
            }
            let actual_username = actual_username.unwrap();

            // Launch TUI
            if let Err(e) = tui::run_tui(&actual_username).await {
                display.print_error(&format!("TUI failed: {}", e));
            }
        }
    }
}

fn filter_entries(
    mut profile: lbxd::models::UserProfile,
    date_filter: Option<String>,
    rated_only: bool,
    reviewed_only: bool,
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

async fn resolve_username(
    username: &str,
    config_manager: &ConfigManager,
    display: &DisplayEngine,
) -> Option<String> {
    if username == "me" {
        match config_manager.get_username() {
            Ok(Some(saved_username)) => Some(saved_username),
            Ok(None) => {
                display.print_error("No username saved. Please provide a username or run a command with your actual username first.");
                None
            }
            Err(_) => {
                display.print_error("Error reading configuration.");
                None
            }
        }
    } else {
        if config_manager.get_username().unwrap_or(None).is_none() {
            if let Err(_) = config_manager.set_username(username.to_string()) {
                display.print_error("Warning: Could not save username to configuration");
            }
        }
        Some(username.to_string())
    }
}

fn show_version_info() {
    let ascii_art = r#"
    ██╗     ██████╗ ██╗  ██╗██████╗ 
    ██║     ██╔══██╗╚██╗██╔╝██╔══██╗
    ██║     ██████╔╝ ╚███╔╝ ██║  ██║
    ██║     ██╔══██╗ ██╔██╗ ██║  ██║
    ███████╗██████╔╝██╔╝ ██╗██████╔╝
    ╚══════╝╚═════╝ ╚═╝  ╚═╝╚═════╝ 
    "#;

    println!("{}", ascii_art);
    println!("    Letterboxd in your terminal");
    println!();
    println!("    Version: {}", env!("CARGO_PKG_VERSION"));
    println!("    Developed by https://pranavkarra.me");
    println!();
    println!("    Use --help to see available commands");
}
