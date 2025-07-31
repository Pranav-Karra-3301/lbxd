use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::profile::UserMovieEntry;
use super::AppStyles;

pub struct MovieGrid {
    movies: Vec<UserMovieEntry>,
    state: ListState,
    selected: usize,
    sort_by: SortMode,
}

#[derive(Debug, Clone, Copy)]
pub enum SortMode {
    Date,
    Rating,
    Title,
    Year,
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
        }
    }

    pub fn set_movies(&mut self, mut movies: Vec<UserMovieEntry>) {
        self.sort_movies(&mut movies);
        self.movies = movies;
        self.selected = 0;
        self.state.select(Some(0));
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => self.previous(),
            KeyCode::Down | KeyCode::Char('j') => self.next(),
            KeyCode::PageUp => self.page_up(),
            KeyCode::PageDown => self.page_down(),
            KeyCode::Home | KeyCode::Char('g') => self.go_to_top(),
            KeyCode::End | KeyCode::Char('G') => self.go_to_bottom(),
            KeyCode::Char('s') => self.cycle_sort(),
            _ => {}
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
                movies.sort_by(|a, b| {
                    b.watched_date.cmp(&a.watched_date)
                });
            }
            SortMode::Rating => {
                movies.sort_by(|a, b| {
                    b.user_rating.partial_cmp(&a.user_rating).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            SortMode::Title => {
                movies.sort_by(|a, b| a.movie.title.cmp(&b.movie.title));
            }
            SortMode::Year => {
                movies.sort_by(|a, b| {
                    b.movie.year.cmp(&a.movie.year)
                });
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
        
        let items: Vec<ListItem> = self.movies
            .iter()
            .map(|entry| {
                let title_year = if let Some(year) = entry.movie.year {
                    format!("{} ({})", entry.movie.title, year)
                } else {
                    entry.movie.title.clone()
                };

                let rating_str = if let Some(rating) = entry.user_rating {
                    format!(" ⭐{:.1}", rating)
                } else {
                    String::new()
                };

                let date_str = if let Some(date) = entry.watched_date {
                    format!(" 📅{}", date.format("%Y-%m-%d"))
                } else {
                    String::new()
                };

                let line = format!("{}{}{}", title_year, rating_str, date_str);
                
                let style = if let Some(rating) = entry.user_rating {
                    styles.rating_style(rating)
                } else {
                    styles.text_style()
                };

                ListItem::new(line).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(styles.border_style())
                    .border_type(styles.border_type())
            )
            .highlight_style(styles.selected_item_style())
            .highlight_symbol("▶ ");

        f.render_stateful_widget(list, area, &mut self.state);
    }

    fn render_movie_details(&self, f: &mut Frame, area: Rect, styles: &AppStyles) {
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

            // Watch date
            if let Some(date) = entry.watched_date {
                details.push(format!("Watched: {}", date.format("%B %d, %Y")));
            }

            // Liked
            if entry.liked {
                details.push("❤️ Liked".to_string());
            }

            // Rewatched
            if entry.rewatched {
                details.push("🔄 Rewatched".to_string());
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
}