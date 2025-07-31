use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;
use tokio::sync::mpsc;

pub mod app;
pub mod grid;
pub mod styles;
pub mod progress;

pub use app::*;
pub use grid::*;
pub use styles::*;
pub use progress::*;

use crate::profile::{ComprehensiveProfile, LoadingProgress};


pub async fn run_tui(username: &str) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new(username.to_string());
    
    // Create channels for progress updates
    let (progress_tx, progress_rx) = mpsc::unbounded_channel::<LoadingProgress>();
    
    // Start data loading in background
    let username_clone = username.to_string();
    let scraper_handle = tokio::spawn(async move {
        match crate::letterboxd_client::LetterboxdClient::new() {
            Ok(client) => client.get_comprehensive_profile(&username_clone, Some(progress_tx)).await,
            Err(e) => Err(e),
        }
    });

    // Run the UI
    let res = run_ui(&mut terminal, &mut app, progress_rx, scraper_handle).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

async fn run_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    mut progress_rx: mpsc::UnboundedReceiver<LoadingProgress>,
    mut scraper_handle: tokio::task::JoinHandle<Result<ComprehensiveProfile>>,
) -> Result<()> {
    // Show loading screen while scraper is running
    let mut scraper_complete = false;
    
    loop {
        // Check if scraper is done
        if !scraper_complete {
            // Poll the scraper handle non-blockingly
            match tokio::time::timeout(tokio::time::Duration::from_millis(10), &mut scraper_handle).await {
                Ok(task_result) => {
                    match task_result {
                        Ok(profile_result) => {
                            match profile_result {
                                Ok(profile) => {
                                    app.set_profile(profile);
                                    scraper_complete = true;
                                    
                                    // Auto-load first movie poster
                                    if let Some(first_movie) = app.get_first_movie_title() {
                                        app.auto_load_first_poster(first_movie);
                                    }
                                }
                                Err(e) => {
                                    app.set_error(format!("Failed to load profile: {}", e));
                                    scraper_complete = true;
                                }
                            }
                        }
                        Err(e) => {
                            app.set_error(format!("Task failed: {}", e));
                            scraper_complete = true;
                        }
                    }
                }
                Err(_) => {
                    // Timeout - scraper still running, continue with loading UI
                }
            }
        }
        
        // Always draw the UI (will show loading screen if not complete)
        terminal.draw(|f| app.render(f))?;
        
        // If loading is complete, continue with main UI loop
        if scraper_complete && !matches!(app.state, crate::tui::AppState::Loading) {
            break;
        }
        
        // Check for progress updates
        while let Ok(progress) = progress_rx.try_recv() {
            app.update_progress(progress);
        }
        
        // Handle basic input during loading (just quit)
        if event::poll(tokio::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    return Ok(());
                }
            }
        }
    }
    
    let omdb_client = crate::omdb::OMDBClient::new();
    let tmdb_client = crate::tmdb::TMDBClient::new();
    let mut last_search_query = String::new();
    
    // Now run the UI loop
    loop {
        terminal.draw(|f| app.render(f))?;

        // Handle poster loading (simplified for development)
        if let Some(title) = app.get_pending_poster_load() {
            app.clear_pending_poster_load();
            
            // Try to get movie details from TMDB and show poster URL
            let title_clone = title.clone();
            if let Ok(Some(movie)) = tmdb_client.search_movie(&title_clone).await {
                if let Some(ref poster_path) = movie.poster_path {
                    let poster_url = tmdb_client.get_poster_url(poster_path);
                    let dev_info = format!("🎬 Poster for {}\n\n[Development Mode]\nPoster URL:\n{}\n\nTMDB ID: {}\nRelease: {}", 
                        title, 
                        poster_url,
                        movie.id,
                        movie.release_date.as_deref().unwrap_or("Unknown")
                    );
                    app.set_poster_result(title, dev_info);
                } else {
                    let fallback = format!("🎬 No poster found for {}\n\n[Development Mode]\nNo poster available on TMDB", title);
                    app.set_poster_result(title, fallback);
                }
            } else {
                let fallback = format!("🎬 Movie not found: {}\n\n[Development Mode]\nTMDB search failed", title);
                app.set_poster_result(title, fallback);
            }
        }


        // Handle search functionality
        if app.should_perform_search() && app.get_search_query() != last_search_query {
            last_search_query = app.get_search_query().to_string();
            if !last_search_query.is_empty() {
                if let Ok(results) = omdb_client.search_movies(&last_search_query, None).await {
                    app.set_search_results(results);
                }
            }
        }

        // Handle events with timeout
        let timeout = tokio::time::Duration::from_millis(100);
        
        // Check for progress updates (in case there are still some in the channel)
        while let Ok(progress) = progress_rx.try_recv() {
            app.update_progress(progress);
        }

        // Handle input events
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        if !app.is_in_search_mode() {
                            break;
                        }
                    }
                    KeyCode::Esc => {
                        if app.is_in_search_mode() {
                            app.handle_key(key);
                        } else {
                            break;
                        }
                    }
                    KeyCode::Enter => {
                        if app.is_in_search_mode() {
                            // Handle movie selection - for now just show details
                            if let Some(_selected_movie) = app.get_selected_search_result() {
                                // Future: Show detailed movie view
                                app.handle_key(crossterm::event::KeyEvent::from(KeyCode::Esc));
                            }
                        } else {
                            app.handle_key(key);
                        }
                    }
                    _ => app.handle_key(key),
                }
            }
        }
    }

    Ok(())
}