use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::profile::{ComprehensiveProfile, LoadingProgress};
use super::{MovieGrid, ProgressBar, AppStyles};

#[derive(Debug, Clone)]
pub enum AppState {
    Loading,
    Loaded,
    Error(String),
    Search,
}

pub struct App {
    pub username: String,
    pub state: AppState,
    pub profile: Option<ComprehensiveProfile>,
    pub progress: Option<LoadingProgress>,
    pub movie_grid: MovieGrid,
    pub styles: AppStyles,
    pub selected_tab: usize, // 0: Movies, 1: Watchlist, 2: Statistics, 3: Analytics, 4: Directors
    pub watchlist_grid: MovieGrid,
    pub search_query: String,
    pub search_results: Vec<crate::omdb::OMDBSearchMovie>,
    pub search_selected: usize,
}

impl App {
    pub fn new(username: String) -> Self {
        Self {
            username,
            state: AppState::Loading,
            profile: None,
            progress: None,
            movie_grid: MovieGrid::new(),
            watchlist_grid: MovieGrid::new(),
            styles: AppStyles::new(),
            selected_tab: 0,
            search_query: String::new(),
            search_results: Vec::new(),
            search_selected: 0,
        }
    }

    pub fn update_progress(&mut self, progress: LoadingProgress) {
        self.progress = Some(progress);
    }

    pub fn set_profile(&mut self, profile: ComprehensiveProfile) {
        self.movie_grid.set_movies(profile.all_movies.clone());
        
        // Convert watchlist DetailedMovies to UserMovieEntry for the grid
        let watchlist_entries: Vec<crate::profile::UserMovieEntry> = profile.watchlist.iter()
            .map(|movie| crate::profile::UserMovieEntry {
                movie: movie.clone(),
                user_rating: None,
                review: None,
                watched_date: None,
                liked: false,
                rewatched: false,
                tags: Vec::new(),
            })
            .collect();
        
        self.watchlist_grid.set_movies(watchlist_entries);
        self.profile = Some(profile);
        self.state = AppState::Loaded;
        self.progress = None;
    }

    pub fn set_error(&mut self, error: String) {
        self.state = AppState::Error(error);
        self.progress = None;
    }

    pub fn set_search_results(&mut self, results: Vec<crate::omdb::OMDBSearchMovie>) {
        self.search_results = results;
        self.search_selected = 0;
    }

    pub fn get_search_query(&self) -> &str {
        &self.search_query
    }

    pub fn get_selected_search_result(&self) -> Option<&crate::omdb::OMDBSearchMovie> {
        self.search_results.get(self.search_selected)
    }

    pub fn is_in_search_mode(&self) -> bool {
        matches!(self.state, AppState::Search)
    }

    pub fn should_perform_search(&self) -> bool {
        matches!(self.state, AppState::Search) && !self.search_query.is_empty()
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match &self.state {
            AppState::Loaded => {
                match key.code {
                    crossterm::event::KeyCode::Tab => {
                        self.selected_tab = (self.selected_tab + 1) % 5;
                    }
                    crossterm::event::KeyCode::BackTab => {
                        self.selected_tab = if self.selected_tab == 0 { 4 } else { self.selected_tab - 1 };
                    }
                    crossterm::event::KeyCode::Char('1') => self.selected_tab = 0,
                    crossterm::event::KeyCode::Char('2') => self.selected_tab = 1,
                    crossterm::event::KeyCode::Char('3') => self.selected_tab = 2,
                    crossterm::event::KeyCode::Char('4') => self.selected_tab = 3,
                    crossterm::event::KeyCode::Char('5') => self.selected_tab = 4,
                    crossterm::event::KeyCode::Char('/') => {
                        self.state = AppState::Search;
                        self.search_query.clear();
                        self.search_results.clear();
                        self.search_selected = 0;
                    }
                    _ => {
                        if self.selected_tab == 0 {
                            self.movie_grid.handle_key(key);
                        } else if self.selected_tab == 1 {
                            self.watchlist_grid.handle_key(key);
                        }
                    }
                }
            }
            AppState::Search => {
                match key.code {
                    crossterm::event::KeyCode::Esc => {
                        self.state = AppState::Loaded;
                    }
                    crossterm::event::KeyCode::Enter => {
                        // This will be handled asynchronously in the main loop
                    }
                    crossterm::event::KeyCode::Up => {
                        if !self.search_results.is_empty() {
                            self.search_selected = if self.search_selected == 0 {
                                self.search_results.len() - 1
                            } else {
                                self.search_selected - 1
                            };
                        }
                    }
                    crossterm::event::KeyCode::Down => {
                        if !self.search_results.is_empty() {
                            self.search_selected = (self.search_selected + 1) % self.search_results.len();
                        }
                    }
                    crossterm::event::KeyCode::Backspace => {
                        self.search_query.pop();
                    }
                    crossterm::event::KeyCode::Char(c) => {
                        self.search_query.push(c);
                    }
                    _ => {}
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
            AppState::Search => self.render_search(f, size),
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
            1 => self.watchlist_grid.render(f, chunks[2], &self.styles),
            2 => self.render_statistics(f, chunks[2]),
            3 => self.render_analytics(f, chunks[2]),
            4 => self.render_directors(f, chunks[2]),
            _ => {}
        }
        
        // Render status bar
        self.render_status_bar(f, chunks[3]);
    }

    fn render_header(&self, f: &mut Frame, area: Rect) {
        if let Some(ref profile) = self.profile {
            let title = if let Some(ref bio) = profile.bio {
                format!(" {} (@{}) - {} ", profile.name, profile.username, bio)
            } else {
                format!(" {} (@{}) ", profile.name, profile.username)
            };
            
            let block = Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(self.styles.header_border_style());
            
            f.render_widget(block, area);
        }
    }

    fn render_tabs(&self, f: &mut Frame, area: Rect) {
        let tabs = ["🎬 Movies", "📝 Watchlist", "📊 Statistics", "📈 Analytics", "🎭 Directors"];
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
                Constraint::Ratio(1, 5),
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



    fn render_status_bar(&self, f: &mut Frame, area: Rect) {
        let help_text = "1-5: Switch tabs | Tab/Shift+Tab: Navigate | ↑↓: Browse | s: Sort | /: Search | q/Esc: Quit";
        let paragraph = Paragraph::new(help_text)
            .style(self.styles.status_bar_style());
        
        f.render_widget(paragraph, area);
    }

    fn render_statistics(&self, f: &mut Frame, area: Rect) {
        if let Some(ref profile) = self.profile {
            if let Some(ref enhanced_stats) = profile.enhanced_stats {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(8),  // Basic stats
                        Constraint::Length(12), // Genre breakdown
                        Constraint::Min(10),    // Rating distribution
                    ])
                    .split(area);

                self.render_basic_stats(f, chunks[0], &enhanced_stats.basic_stats);
                self.render_genre_breakdown(f, chunks[1], &enhanced_stats.genre_breakdown);
                self.render_rating_distribution(f, chunks[2], &enhanced_stats.rating_distribution);
            } else {
                let block = Block::default()
                    .title(" 📊 Statistics ")
                    .borders(Borders::ALL)
                    .border_style(self.styles.border_style());
                
                let paragraph = Paragraph::new("Loading enhanced statistics...")
                    .block(block)
                    .style(self.styles.dim_text_style());
                
                f.render_widget(paragraph, area);
            }
        }
    }

    fn render_basic_stats(&self, f: &mut Frame, area: Rect, stats: &crate::profile::UserStatistics) {
        let block = Block::default()
            .title(" 📊 Overview ")
            .borders(Borders::ALL)
            .border_style(self.styles.stats_title_style())
            .border_type(self.styles.border_type());

        let viewing_hours_str = if stats.total_viewing_time_hours >= 1.0 {
            format!("{:.1}h", stats.total_viewing_time_hours)
        } else {
            format!("{:.0}m", stats.total_viewing_time_hours * 60.0)
        };

        let avg_length_str = if stats.average_film_length >= 60.0 {
            let hours = (stats.average_film_length / 60.0) as u32;
            let mins = (stats.average_film_length % 60.0) as u32;
            format!("{}h {}m", hours, mins)
        } else {
            format!("{:.0}m", stats.average_film_length)
        };

        let stats_text = format!(
            "🎬 Total Viewing Time: {}\n⏱️  Average Film Length: {}\n📊 Average Rating: {:.1}/5\n🎭 Unique Directors: {}\n🎪 Unique Genres: {}",
            viewing_hours_str,
            avg_length_str,
            stats.average_rating,
            stats.unique_directors_count,
            stats.unique_genres_count
        );

        let paragraph = Paragraph::new(stats_text)
            .block(block)
            .style(self.styles.stats_value_style());

        f.render_widget(paragraph, area);
    }

    fn render_genre_breakdown(&self, f: &mut Frame, area: Rect, genres: &[crate::profile::GenreStats]) {
        let block = Block::default()
            .title(" 🎪 Top Genres ")
            .borders(Borders::ALL)
            .border_style(self.styles.stats_title_style())
            .border_type(self.styles.border_type());

        let mut genre_lines = Vec::new();
        for (_i, genre) in genres.iter().take(8).enumerate() {
            let bar_length = ((genre.percentage / 100.0) * 20.0) as usize;
            let bar = "█".repeat(bar_length) + &"░".repeat(20 - bar_length);
            
            let line = format!(
                "{}  {} {:.1}% {} {:.1}",
                genre.emoji,
                genre.name,
                genre.percentage,
                bar,
                genre.average_rating
            );
            genre_lines.push(line);
        }

        let paragraph = Paragraph::new(genre_lines.join("\n"))
            .block(block)
            .style(self.styles.text_style());

        f.render_widget(paragraph, area);
    }

    fn render_rating_distribution(&self, f: &mut Frame, area: Rect, distribution: &[crate::profile::RatingDistribution]) {
        let block = Block::default()
            .title(" 📊 Rating Distribution ")
            .borders(Borders::ALL)
            .border_style(self.styles.stats_title_style())
            .border_type(self.styles.border_type());

        if distribution.is_empty() {
            let paragraph = Paragraph::new("No personal ratings available\n(Rating data requires logged-in Letterboxd scraping)")
                .block(block)
                .style(self.styles.dim_text_style());

            f.render_widget(paragraph, area);
            return;
        }

        let mut rating_lines = Vec::new();
        for rating_data in distribution {
            let bar_length = ((rating_data.percentage / 100.0) * 25.0) as usize;
            let bar = "█".repeat(bar_length) + &"░".repeat(25 - bar_length);
            
            let line = format!(
                "{:.1}★ {} ({}) {:.1}%",
                rating_data.rating,
                bar,
                rating_data.count,
                rating_data.percentage
            );
            rating_lines.push(line);
        }

        let paragraph = Paragraph::new(rating_lines.join("\n"))
            .block(block)
            .style(self.styles.text_style());

        f.render_widget(paragraph, area);
    }

    fn render_analytics(&self, f: &mut Frame, area: Rect) {
        if let Some(ref profile) = self.profile {
            if let Some(ref enhanced_stats) = profile.enhanced_stats {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(12), // Yearly breakdown
                        Constraint::Min(10),    // Viewing patterns
                    ])
                    .split(area);

                self.render_yearly_breakdown(f, chunks[0], &enhanced_stats.yearly_breakdown);
                self.render_viewing_patterns(f, chunks[1], &enhanced_stats.viewing_patterns);
            } else {
                let block = Block::default()
                    .title(" 📈 Analytics ")
                    .borders(Borders::ALL)
                    .border_style(self.styles.border_style());
                
                let paragraph = Paragraph::new("Loading analytics data...")
                    .block(block)
                    .style(self.styles.dim_text_style());
                
                f.render_widget(paragraph, area);
            }
        }
    }

    fn render_yearly_breakdown(&self, f: &mut Frame, area: Rect, yearly_data: &[crate::profile::YearlyBreakdown]) {
        let block = Block::default()
            .title(" 📅 Yearly Breakdown ")
            .borders(Borders::ALL)
            .border_style(self.styles.analytics_header_style())
            .border_type(self.styles.border_type());

        let mut year_lines = Vec::new();
        for year_data in yearly_data.iter().take(8) {
            let runtime_hours = year_data.total_runtime as f32 / 60.0;
            let line = format!(
                "{} 🎬 {} films  ⏱️ {:.1}h  ⭐ {:.1}",
                year_data.year,
                year_data.film_count,
                runtime_hours,
                year_data.average_rating
            );
            year_lines.push(line);
        }

        let paragraph = Paragraph::new(year_lines.join("\n"))
            .block(block)
            .style(self.styles.text_style());

        f.render_widget(paragraph, area);
    }

    fn render_viewing_patterns(&self, f: &mut Frame, area: Rect, patterns: &[crate::profile::ViewingPattern]) {
        let block = Block::default()
            .title(" 📊 Monthly Viewing Patterns ")
            .borders(Borders::ALL)
            .border_style(self.styles.analytics_header_style())
            .border_type(self.styles.border_type());

        let month_names = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun",
            "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
        ];

        let mut pattern_lines = Vec::new();
        for pattern in patterns {
            let month_name = month_names.get((pattern.month - 1) as usize).unwrap_or(&"???");
            let bar_length = ((pattern.films_watched as f32 / 20.0) * 15.0) as usize;
            let bar = "█".repeat(bar_length.min(15)) + &"░".repeat(15 - bar_length.min(15));
            
            let line = format!(
                "{} {} {} films",
                month_name,
                bar,
                pattern.films_watched
            );
            pattern_lines.push(line);
        }

        let paragraph = Paragraph::new(pattern_lines.join("\n"))
            .block(block)
            .style(self.styles.text_style());

        f.render_widget(paragraph, area);
    }

    fn render_directors(&self, f: &mut Frame, area: Rect) {
        if let Some(ref profile) = self.profile {
            if let Some(ref enhanced_stats) = profile.enhanced_stats {
                let block = Block::default()
                    .title(" 🎭 Top Directors ")
                    .borders(Borders::ALL)
                    .border_style(self.styles.analytics_header_style())
                    .border_type(self.styles.border_type());

                let mut director_lines = Vec::new();
                for (i, director) in enhanced_stats.director_stats.iter().take(15).enumerate() {
                    let rank_emoji = match i {
                        0 => "🥇",
                        1 => "🥈", 
                        2 => "🥉",
                        _ => "🎬",
                    };

                    let favorite_info = if let Some(ref fav) = director.favorite_film {
                        format!(" (Fav: {})", fav)
                    } else {
                        String::new()
                    };

                    let line = format!(
                        "{} {} {} films ⭐ {:.1}{}",
                        rank_emoji,
                        director.name,
                        director.film_count,
                        director.average_rating,
                        favorite_info
                    );
                    director_lines.push(line);
                }

                let paragraph = Paragraph::new(director_lines.join("\n"))
                    .block(block)
                    .style(self.styles.text_style());

                f.render_widget(paragraph, area);
            } else {
                let block = Block::default()
                    .title(" 🎭 Directors ")
                    .borders(Borders::ALL)
                    .border_style(self.styles.border_style());
                
                let paragraph = Paragraph::new("Loading director data...")
                    .block(block)
                    .style(self.styles.dim_text_style());
                
                f.render_widget(paragraph, area);
            }
        }
    }


    fn render_search(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Search input
                Constraint::Min(5),    // Search results
                Constraint::Length(1), // Help text
            ])
            .split(area);

        // Search input
        let search_block = Block::default()
            .title(" Movie Search ")
            .borders(Borders::ALL)
            .border_style(self.styles.header_border_style());

        let search_text = format!("🔍 {}", self.search_query);
        let search_paragraph = Paragraph::new(search_text)
            .block(search_block)
            .style(self.styles.text_style());

        f.render_widget(search_paragraph, chunks[0]);

        // Search results
        let results_block = Block::default()
            .title(" Search Results ")
            .borders(Borders::ALL)
            .border_style(self.styles.border_style());

        if self.search_results.is_empty() {
            let message = if self.search_query.is_empty() {
                "Type to search for movies..."
            } else {
                "No results found"
            };
            
            let paragraph = Paragraph::new(message)
                .block(results_block)
                .style(self.styles.dim_text_style());

            f.render_widget(paragraph, chunks[1]);
        } else {
            let items: Vec<ListItem> = self.search_results
                .iter()
                .enumerate()
                .map(|(i, movie)| {
                    let line = format!("{} ({})", movie.title, movie.year);
                    let style = if i == self.search_selected {
                        self.styles.selected_item_style()
                    } else {
                        self.styles.text_style()
                    };
                    ListItem::new(line).style(style)
                })
                .collect();

            let list = List::new(items)
                .block(results_block)
                .highlight_style(self.styles.selected_item_style())
                .highlight_symbol("▶ ");

            let mut list_state = ListState::default();
            list_state.select(Some(self.search_selected));

            f.render_stateful_widget(list, chunks[1], &mut list_state);
        }

        // Help text
        let help_text = "Type to search | ↑↓: Navigate | Enter: Select | Esc: Cancel";
        let help_paragraph = Paragraph::new(help_text)
            .style(self.styles.status_bar_style());

        f.render_widget(help_paragraph, chunks[2]);
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