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
        let scraper = crate::profile::ProfileScraper::new();
        scraper.scrape_comprehensive_profile(&username_clone, Some(progress_tx)).await
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
    
    // Now run the UI loop
    loop {
        terminal.draw(|f| app.render(f))?;

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
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => app.handle_key(key),
                }
            }
        }
    }

    Ok(())
}