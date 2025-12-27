use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use super::AppStyles;
use crate::profile::UserMovieEntry;

pub struct MovieGrid {
    movies: Vec<UserMovieEntry>,
    state: ListState,
    selected: usize,
    sort_by: SortMode,
    poster_cache: std::collections::HashMap<String, String>, // title -> ascii art
    loading_poster: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum SortMode {
    Date,
    Rating,
    Title,
    Year,
}

#[derive(Debug, Clone)]
pub enum MovieGridAction {
    LoadPoster(String), // movie title
}

impl Default for MovieGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl MovieGrid {
    pub fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));

        Self {
            movies: Vec::new(),
            state,
            selected: 0,
            sort_by: SortMode::Date,
            poster_cache: std::collections::HashMap::new(),
            loading_poster: false,
        }
    }

    pub fn set_movies(&mut self, mut movies: Vec<UserMovieEntry>) {
        self.sort_movies(&mut movies);
        self.movies = movies;
        self.selected = 0;
        self.state.select(Some(0));
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<MovieGridAction> {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.previous();
                None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.next();
                None
            }
            KeyCode::PageUp => {
                self.page_up();
                None
            }
            KeyCode::PageDown => {
                self.page_down();
                None
            }
            KeyCode::Home | KeyCode::Char('g') => {
                self.go_to_top();
                None
            }
            KeyCode::End | KeyCode::Char('G') => {
                self.go_to_bottom();
                None
            }
            KeyCode::Char('s') => {
                self.cycle_sort();
                None
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                if let Some(movie) = self.movies.get(self.selected) {
                    Some(MovieGridAction::LoadPoster(movie.movie.title.clone()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn previous(&mut self) {
        if self.movies.is_empty() {
            return;
        }

        self.selected = if self.selected == 0 {
            self.movies.len() - 1
        } else {
            self.selected - 1
        };
        self.state.select(Some(self.selected));
    }

    fn next(&mut self) {
        if self.movies.is_empty() {
            return;
        }

        self.selected = (self.selected + 1) % self.movies.len();
        self.state.select(Some(self.selected));
    }

    fn page_up(&mut self) {
        if self.movies.is_empty() {
            return;
        }

        self.selected = self.selected.saturating_sub(10);
        self.state.select(Some(self.selected));
    }

    fn page_down(&mut self) {
        if self.movies.is_empty() {
            return;
        }

        self.selected = std::cmp::min(self.selected + 10, self.movies.len() - 1);
        self.state.select(Some(self.selected));
    }

    fn go_to_top(&mut self) {
        if !self.movies.is_empty() {
            self.selected = 0;
            self.state.select(Some(0));
        }
    }

    fn go_to_bottom(&mut self) {
        if !self.movies.is_empty() {
            self.selected = self.movies.len() - 1;
            self.state.select(Some(self.selected));
        }
    }

    fn cycle_sort(&mut self) {
        self.sort_by = match self.sort_by {
            SortMode::Date => SortMode::Rating,
            SortMode::Rating => SortMode::Title,
            SortMode::Title => SortMode::Year,
            SortMode::Year => SortMode::Date,
        };

        let mut movies = self.movies.clone();
        self.sort_movies(&mut movies);
        self.movies = movies;
    }

    fn sort_movies(&self, movies: &mut Vec<UserMovieEntry>) {
        match self.sort_by {
            SortMode::Date => {
                movies.sort_by(|a, b| b.watched_date.cmp(&a.watched_date));
            }
            SortMode::Rating => {
                movies.sort_by(|a, b| {
                    // Prioritize letterboxd_rating over user_rating
                    let a_rating = a.movie.letterboxd_rating.or(a.user_rating);
                    let b_rating = b.movie.letterboxd_rating.or(b.user_rating);
                    b_rating
                        .partial_cmp(&a_rating)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            SortMode::Title => {
                movies.sort_by(|a, b| a.movie.title.cmp(&b.movie.title));
            }
            SortMode::Year => {
                movies.sort_by(|a, b| b.movie.year.cmp(&a.movie.year));
            }
        }
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect, styles: &AppStyles) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Ratio(2, 3), // Movie list
                Constraint::Ratio(1, 3), // Movie details
            ])
            .split(area);

        self.render_movie_list(f, chunks[0], styles);
        self.render_movie_details(f, chunks[1], styles);
    }

    fn render_movie_list(&mut self, f: &mut Frame, area: Rect, styles: &AppStyles) {
        let sort_indicator = match self.sort_by {
            SortMode::Date => "📅 Date",
            SortMode::Rating => "⭐ Rating",
            SortMode::Title => "🎬 Title",
            SortMode::Year => "📆 Year",
        };

        let title = format!(" Movies (Sorted by {}) ", sort_indicator);

        let items: Vec<ListItem> = self
            .movies
            .iter()
            .map(|entry| {
                // Column 1: Title (truncated to fit)
                let title = if entry.movie.title.len() > 33 {
                    format!("{}...", &entry.movie.title[..30])
                } else {
                    entry.movie.title.clone()
                };

                // Column 2: Date watched
                let watched_date = if let Some(date) = entry.watched_date {
                    date.format("%Y-%m-%d").to_string()
                } else {
                    "-".to_string()
                };

                // Column 3: Release year
                let release_year = if let Some(year) = entry.movie.year {
                    year.to_string()
                } else {
                    "-".to_string()
                };

                // Column 4: Letterboxd Rating
                let letterboxd_rating = if let Some(rating) = entry.movie.letterboxd_rating {
                    format!("{:.1}", rating)
                } else {
                    "-".to_string()
                };

                // Column 5: IMDB Rating
                let imdb_rating = if let Some(rating) = entry.movie.imdb_rating {
                    format!("IMDb:{:.1}", rating)
                } else {
                    "-".to_string()
                };

                // Column 6: RT Rating
                let rt_rating = if let Some(rating) = entry.movie.rotten_tomatoes_rating {
                    format!("RT:{}%", rating)
                } else {
                    "-".to_string()
                };

                // Format as columns with consistent spacing
                let line = format!(
                    "{:<35} {:<12} {:<6} {:<8} {:<10} {:<8}",
                    title, watched_date, release_year, letterboxd_rating, imdb_rating, rt_rating
                );

                let style = if let Some(rating) = entry.user_rating {
                    styles.rating_style(rating)
                } else {
                    styles.text_style()
                };

                ListItem::new(line).style(style)
            })
            .collect();

        // Add header row
        let header = format!(
            "{:<35} {:<12} {:<6} {:<8} {:<10} {:<8}",
            "Title", "Watched", "Year", "LB", "IMDb", "RT"
        );

        let mut all_items = vec![ListItem::new(header).style(styles.header_style())];
        all_items.extend(items);

        let list = List::new(all_items)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(styles.border_style())
                    .border_type(styles.border_type()),
            )
            .highlight_style(styles.selected_item_style())
            .highlight_symbol("▶ ");

        // Adjust state to account for header row
        let mut adjusted_state = ListState::default();
        adjusted_state.select(self.state.selected().map(|i| i + 1));

        f.render_stateful_widget(list, area, &mut adjusted_state);
    }

    fn render_movie_details(&self, f: &mut Frame, area: Rect, styles: &AppStyles) {
        // Split the details area to show poster and details side by side
        let detail_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(20), // Poster area
                Constraint::Min(10),    // Details area
            ])
            .split(area);

        self.render_movie_poster(f, detail_chunks[0], styles);
        self.render_movie_info(f, detail_chunks[1], styles);
    }

    fn render_movie_poster(&self, f: &mut Frame, area: Rect, styles: &AppStyles) {
        let block = Block::default()
            .title(" Poster ")
            .borders(Borders::ALL)
            .border_style(styles.border_style())
            .border_type(styles.border_type());

        if let Some(entry) = self.movies.get(self.selected) {
            // Try to get and display poster using viu
            let poster_text = self.get_movie_poster_text(&entry.movie.title, entry.movie.year);

            let paragraph = Paragraph::new(poster_text)
                .block(block)
                .wrap(Wrap { trim: true })
                .style(styles.text_style());

            f.render_widget(paragraph, area);
        } else {
            let paragraph = Paragraph::new("No movie selected")
                .block(block)
                .style(styles.dim_text_style());

            f.render_widget(paragraph, area);
        }
    }

    fn render_movie_info(&self, f: &mut Frame, area: Rect, styles: &AppStyles) {
        let block = Block::default()
            .title(" Details ")
            .borders(Borders::ALL)
            .border_style(styles.border_style())
            .border_type(styles.border_type());

        if let Some(entry) = self.movies.get(self.selected) {
            let mut details = Vec::new();

            // Title and year
            let title_line = if let Some(year) = entry.movie.year {
                format!("{} ({})", entry.movie.title, year)
            } else {
                entry.movie.title.clone()
            };
            details.push(title_line);
            details.push(String::new()); // Empty line

            // Rating
            if let Some(rating) = entry.user_rating {
                details.push(format!("Your Rating: ⭐ {:.1}/5", rating));
            }

            // Letterboxd Rating
            if let Some(rating) = entry.movie.letterboxd_rating {
                details.push(format!("Letterboxd Rating: ⭐ {:.2}/5", rating));
            }

            // OMDB Ratings
            if let Some(rating) = entry.movie.imdb_rating {
                details.push(format!("IMDb Rating: ⭐ {:.1}/10", rating));
            }

            if let Some(rating) = entry.movie.rotten_tomatoes_rating {
                details.push(format!("Rotten Tomatoes: 🍅 {}%", rating));
            }

            if let Some(rating) = entry.movie.metacritic_rating {
                details.push(format!("Metacritic: 📊 {}/100", rating));
            }

            // Director
            if let Some(ref director) = entry.movie.director {
                details.push(format!("Director: {}", director));
            }

            // Genres
            if !entry.movie.genres.is_empty() {
                details.push(format!("Genres: {}", entry.movie.genres.join(", ")));
            }

            // Runtime
            if let Some(runtime) = entry.movie.runtime {
                let hours = runtime / 60;
                let minutes = runtime % 60;
                if hours > 0 {
                    details.push(format!("Runtime: {}h {}m", hours, minutes));
                } else {
                    details.push(format!("Runtime: {}m", minutes));
                }
            }

            // Release date
            if let Some(ref release_date) = entry.movie.release_date {
                details.push(format!("Released: {}", release_date));
            }

            // Watch date
            if let Some(date) = entry.watched_date {
                details.push(format!("Watched: {}", date.format("%B %d, %Y")));
            }

            // Plot/Synopsis (prefer OMDB plot over synopsis)
            let plot_text = entry.movie.plot.as_ref().or(entry.movie.synopsis.as_ref());
            if let Some(plot) = plot_text {
                details.push(String::new()); // Empty line
                details.push("Plot:".to_string());
                details.push(plot.clone());
            }

            // Awards
            if let Some(ref awards) = entry.movie.awards {
                if awards != "N/A" && !awards.is_empty() {
                    details.push(String::new()); // Empty line
                    details.push("Awards:".to_string());
                    details.push(awards.clone());
                }
            }

            // Review
            if let Some(ref review) = entry.review {
                details.push(String::new()); // Empty line
                details.push("Review:".to_string());
                details.push(review.clone());
            }

            let text = details.join("\n");
            let paragraph = Paragraph::new(text)
                .block(block)
                .wrap(Wrap { trim: true })
                .style(styles.text_style());

            f.render_widget(paragraph, area);
        } else {
            let paragraph = Paragraph::new("No movie selected")
                .block(block)
                .style(styles.dim_text_style());

            f.render_widget(paragraph, area);
        }
    }

    fn get_movie_poster_text(&self, title: &str, _year: Option<u16>) -> String {
        // Check cache first
        if let Some(cached_poster) = self.poster_cache.get(title) {
            return cached_poster.clone();
        }

        // Show loading state if poster is being loaded
        if self.loading_poster {
            return format!("🎬 {}\n\n⏳ Loading movie info...", title);
        }

        // Default placeholder
        format!("🎬 {}\n\n💡 Press 'p' to load TMDB info", title)
    }

    pub fn set_poster_cache(&mut self, title: String, ascii_art: String) {
        self.poster_cache.insert(title, ascii_art);
        self.loading_poster = false;
    }

    pub fn set_loading_poster(&mut self, loading: bool) {
        self.loading_poster = loading;
    }
}
