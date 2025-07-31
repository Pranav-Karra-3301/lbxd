use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::profile::{ComprehensiveProfile, LoadingProgress};
use super::{MovieGrid, ProgressBar, AppStyles};

#[derive(Debug, Clone)]
pub enum AppState {
    Loading,
    Loaded,
    Error(String),
}

pub struct App {
    pub username: String,
    pub state: AppState,
    pub profile: Option<ComprehensiveProfile>,
    pub progress: Option<LoadingProgress>,
    pub movie_grid: MovieGrid,
    pub styles: AppStyles,
    pub selected_tab: usize, // 0: Movies, 1: Lists, 2: Stats
}

impl App {
    pub fn new(username: String) -> Self {
        Self {
            username,
            state: AppState::Loading,
            profile: None,
            progress: None,
            movie_grid: MovieGrid::new(),
            styles: AppStyles::new(),
            selected_tab: 0,
        }
    }

    pub fn update_progress(&mut self, progress: LoadingProgress) {
        self.progress = Some(progress);
    }

    pub fn set_profile(&mut self, profile: ComprehensiveProfile) {
        self.movie_grid.set_movies(profile.all_movies.clone());
        self.profile = Some(profile);
        self.state = AppState::Loaded;
        self.progress = None;
    }

    pub fn set_error(&mut self, error: String) {
        self.state = AppState::Error(error);
        self.progress = None;
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match &self.state {
            AppState::Loaded => {
                match key.code {
                    crossterm::event::KeyCode::Tab => {
                        self.selected_tab = (self.selected_tab + 1) % 3;
                    }
                    crossterm::event::KeyCode::BackTab => {
                        self.selected_tab = if self.selected_tab == 0 { 2 } else { self.selected_tab - 1 };
                    }
                    _ => {
                        if self.selected_tab == 0 {
                            self.movie_grid.handle_key(key);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub fn render(&mut self, f: &mut Frame) {
        let size = f.size();
        
        match &self.state {
            AppState::Loading => self.render_loading(f, size),
            AppState::Loaded => self.render_main(f, size),
            AppState::Error(error) => self.render_error(f, size, error),
        }
    }

    fn render_loading(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(format!(" Loading Profile: {} ", self.username))
            .borders(Borders::ALL)
            .border_style(self.styles.border_style());

        f.render_widget(block, area);

        if let Some(ref progress) = self.progress {
            let inner = area.inner(&ratatui::layout::Margin {
                vertical: 2,
                horizontal: 2,
            });

            let progress_bar = ProgressBar::new(progress.clone());
            progress_bar.render(f, inner, &self.styles);
        }
    }

    fn render_main(&mut self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // Tabs
                Constraint::Min(10),   // Content
                Constraint::Length(1), // Status bar
            ])
            .split(area);

        // Render header
        self.render_header(f, chunks[0]);
        
        // Render tabs
        self.render_tabs(f, chunks[1]);
        
        // Render content based on selected tab
        match self.selected_tab {
            0 => self.movie_grid.render(f, chunks[2], &self.styles),
            1 => self.render_lists(f, chunks[2]),
            2 => self.render_stats(f, chunks[2]),
            _ => {}
        }
        
        // Render status bar
        self.render_status_bar(f, chunks[3]);
    }

    fn render_header(&self, f: &mut Frame, area: Rect) {
        if let Some(ref profile) = self.profile {
            let title = format!(" {} (@{}) ", profile.name, profile.username);
            let block = Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(self.styles.header_border_style());
            
            f.render_widget(block, area);
        }
    }

    fn render_tabs(&self, f: &mut Frame, area: Rect) {
        let tabs = ["Movies", "Lists", "Stats"];
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(area);

        for (i, tab) in tabs.iter().enumerate() {
            let style = if i == self.selected_tab {
                self.styles.selected_tab_style()
            } else {
                self.styles.tab_style()
            };
            
            let block = Block::default()
                .title(format!(" {} ", tab))
                .borders(Borders::ALL)
                .border_style(style);
            
            f.render_widget(block, chunks[i]);
        }
    }

    fn render_lists(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Lists ")
            .borders(Borders::ALL)
            .border_style(self.styles.border_style());
        
        let paragraph = Paragraph::new("Lists functionality coming soon...")
            .block(block)
            .style(self.styles.text_style());
        
        f.render_widget(paragraph, area);
    }

    fn render_stats(&self, f: &mut Frame, area: Rect) {
        if let Some(ref profile) = self.profile {
            let stats_text = format!(
                "Total Films: {}\nFilms This Year: {}\nLists: {}\nFollowing: {}\nFollowers: {}",
                profile.total_films,
                profile.films_this_year,
                profile.lists_count,
                profile.following_count,
                profile.followers_count
            );
            
            let block = Block::default()
                .title(" Statistics ")
                .borders(Borders::ALL)
                .border_style(self.styles.border_style());
            
            let paragraph = Paragraph::new(stats_text)
                .block(block)
                .style(self.styles.text_style());
            
            f.render_widget(paragraph, area);
        }
    }

    fn render_status_bar(&self, f: &mut Frame, area: Rect) {
        let help_text = "Tab: Switch tabs | ↑↓: Navigate | q/Esc: Quit";
        let paragraph = Paragraph::new(help_text)
            .style(self.styles.status_bar_style());
        
        f.render_widget(paragraph, area);
    }

    fn render_error(&self, f: &mut Frame, area: Rect, error: &str) {
        let block = Block::default()
            .title(" Error ")
            .borders(Borders::ALL)
            .border_style(self.styles.error_border_style());

        let paragraph = Paragraph::new(error.to_string())
            .block(block)
            .style(self.styles.error_text_style());

        f.render_widget(paragraph, area);
    }
}