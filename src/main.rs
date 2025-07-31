use clap::Parser;
use lbxd::{
    cli::{Cli, Commands, ConfigCommands, ColorModeArg, DisplayModeArg},
    display::DisplayEngine,
    feed::FeedParser,
    cache::CacheManager,
    export::ExportManager,
    config::{ConfigManager, ColorMode, DisplayMode},
    tmdb::TMDBClient,
    onboarding::OnboardingManager,
    profile::ProfileScraper,
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
                    
                    let profile_scraper = ProfileScraper::new();
                    display.print_loading_animation("Fetching profile stats...", 1000).await;
                    
                    match profile_scraper.scrape_profile(&actual_username).await {
                        Ok(profile_stats) => {
                            display.show_profile_stats(&profile_stats).await;
                        },
                        Err(e) => {
                            display.print_error(&format!("Failed to fetch profile stats: {}", e));
                        }
                    }
                }
                return;
            } else if !cli.reconfig {
                // No command, no username, and no --reconfig, show help
                display.print_error("No command provided. Use --help to see available commands or --reconfig to reconfigure settings.");
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
        Commands::Recent { username, limit, date, rated, reviewed, vertical, ascii, width } => {
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
                        },
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
            display.show_user_activity(&filtered_profile, limit, vertical, ascii, width).await;
        },
        
        Commands::Search { username, title, ascii, width } => {
            let actual_username = resolve_username(&username, &config_manager, &display).await;
            if actual_username.is_none() {
                return;
            }
            let actual_username = actual_username.unwrap();

            display.print_minimal_logo();
            
            match feed_parser.fetch_user_feed(&actual_username).await {
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
                                username: actual_username.clone(),
                                display_name: profile.display_name.clone(),
                                avatar_url: None,
                                rss_url: profile.rss_url.clone(),
                                entries: vec![entry.clone()],
                            }, None, true, ascii, width).await; // Default to vertical for search results
                        }
                    }
                },
                Err(e) => {
                    display.print_error(&format!("Failed to fetch user data: {}", e));
                }
            }
        },
        
        Commands::Compare { usernames: _ } => {
            display.print_minimal_logo();
            display.print_error("Compare feature coming soon!");
        },
        
        Commands::Export { username, format, output } => {
            let actual_username = resolve_username(&username, &config_manager, &display).await;
            if actual_username.is_none() {
                return;
            }
            let actual_username = actual_username.unwrap();

            match feed_parser.fetch_user_feed(&actual_username).await {
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
            display.print_minimal_logo();
            display.print_error("Summary feature coming soon!");
        },

        Commands::Movie { title, ascii, width } => {
            display.print_minimal_logo();

            let tmdb_client = TMDBClient::new();
            display.print_loading_animation("Searching TMDB...", 1000).await;
            
            match tmdb_client.search_movie(&title).await {
                Ok(Some(movie)) => {
                    display.show_tmdb_movie(&movie, ascii, width).await;
                },
                Ok(None) => {
                    display.print_error(&format!("No movies found for '{}'", title));
                },
                Err(e) => {
                    display.print_error(&format!("Failed to search TMDB: {}", e));
                }
            }
        },

        Commands::Config { config_command } => {
            display.print_minimal_logo();

            match config_command {
                ConfigCommands::Whoami => {
                    match config_manager.get_username() {
                        Ok(Some(username)) => {
                            display.print_success(&format!("Current username: {}", username));
                        },
                        Ok(None) => {
                            display.print_warning("No username is currently saved");
                        },
                        Err(e) => {
                            display.print_error(&format!("Failed to read config: {}", e));
                        }
                    }
                },
                ConfigCommands::SetUser { username } => {
                    match config_manager.change_username(username.clone()) {
                        Ok(_) => {
                            display.print_success(&format!("Username set to: {}", username));
                        },
                        Err(e) => {
                            display.print_error(&format!("Failed to save username: {}", e));
                        }
                    }
                },
                ConfigCommands::Show => {
                    match config_manager.get_all_config() {
                        Ok(config) => {
                            display.print_info("Current Configuration:");
                            println!("  Username: {}", config.username.unwrap_or_else(|| "Not set".to_string()));
                            println!("  Color mode: {:?}", config.color_mode);
                            println!("  Display mode: {:?}", config.display_mode);
                        },
                        Err(e) => {
                            display.print_error(&format!("Failed to read config: {}", e));
                        }
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
                        },
                        Err(e) => {
                            display.print_error(&format!("Failed to update color mode: {}", e));
                        }
                    }
                },
                ConfigCommands::SetMode { mode } => {
                    let display_mode = match mode {
                        DisplayModeArg::Pixelated => DisplayMode::Pixelated,
                        DisplayModeArg::Full => DisplayMode::FullResolution,
                    };
                    match config_manager.set_display_mode(display_mode) {
                        Ok(_) => {
                            display.print_success(&format!("Display mode set to: {:?}", mode));
                        },
                        Err(e) => {
                            display.print_error(&format!("Failed to update display mode: {}", e));
                        }
                    }
                },
            }
        },

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

async fn resolve_username(username: &str, config_manager: &ConfigManager, display: &DisplayEngine) -> Option<String> {
    if username == "me" {
        match config_manager.get_username() {
            Ok(Some(saved_username)) => Some(saved_username),
            Ok(None) => {
                display.print_error("No username saved. Please provide a username or run a command with your actual username first.");
                None
            },
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
