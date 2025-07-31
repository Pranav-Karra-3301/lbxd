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
    scraper_handle: tokio::task::JoinHandle<Result<ComprehensiveProfile>>,
) -> Result<()> {
    // First, just wait for the scraper to complete
    match scraper_handle.await {
        Ok(Ok(profile)) => {
            app.set_profile(profile);
        }
        Ok(Err(e)) => {
            app.set_error(format!("Failed to load profile: {}", e));
        }
        Err(e) => {
            app.set_error(format!("Task failed: {}", e));
        }
    }
    
    let omdb_client = crate::omdb::OMDBClient::new();
    let mut last_search_query = String::new();
    
    // Now run the UI loop
    loop {
        terminal.draw(|f| app.render(f))?;

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