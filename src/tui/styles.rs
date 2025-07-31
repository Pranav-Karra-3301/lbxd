use ratatui::{
    style::{Color, Style, Modifier},
    widgets::BorderType,
};

pub struct AppStyles {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
    pub error_color: Color,
    pub text_color: Color,
    pub dim_text_color: Color,
}

impl AppStyles {
    pub fn new() -> Self {
        Self {
            primary_color: Color::Rgb(0, 215, 53),    // Letterboxd green
            secondary_color: Color::Rgb(255, 128, 0), // Letterboxd orange
            accent_color: Color::Rgb(64, 188, 244),   // Blue accent
            error_color: Color::Rgb(220, 50, 47),     // Red
            text_color: Color::White,
            dim_text_color: Color::Rgb(128, 128, 128),
        }
    }

    pub fn border_style(&self) -> Style {
        Style::default().fg(self.primary_color)
    }

    pub fn header_border_style(&self) -> Style {
        Style::default()
            .fg(self.primary_color)
            .add_modifier(Modifier::BOLD)
    }

    pub fn selected_tab_style(&self) -> Style {
        Style::default()
            .fg(self.primary_color)
            .add_modifier(Modifier::BOLD)
    }

    pub fn tab_style(&self) -> Style {
        Style::default().fg(self.dim_text_color)
    }

    pub fn text_style(&self) -> Style {
        Style::default().fg(self.text_color)
    }

    pub fn dim_text_style(&self) -> Style {
        Style::default().fg(self.dim_text_color)
    }

    pub fn status_bar_style(&self) -> Style {
        Style::default()
            .fg(self.dim_text_color)
            .bg(Color::Rgb(20, 20, 20))
    }

    pub fn error_border_style(&self) -> Style {
        Style::default().fg(self.error_color)
    }

    pub fn error_text_style(&self) -> Style {
        Style::default().fg(self.error_color)
    }

    pub fn progress_bar_style(&self) -> Style {
        Style::default().fg(self.primary_color)
    }

    pub fn progress_bg_style(&self) -> Style {
        Style::default().fg(self.dim_text_color)
    }

    pub fn movie_title_style(&self) -> Style {
        Style::default()
            .fg(self.text_color)
            .add_modifier(Modifier::BOLD)
    }

    pub fn movie_year_style(&self) -> Style {
        Style::default().fg(self.dim_text_color)
    }

    pub fn rating_style(&self, rating: f32) -> Style {
        let color = match rating {
            r if r >= 4.0 => self.primary_color,      // Green for high ratings
            r if r >= 3.0 => self.secondary_color,    // Orange for medium ratings
            r if r >= 2.0 => Color::Yellow,           // Yellow for low-medium ratings
            _ => self.error_color,                     // Red for low ratings
        };
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    }

    pub fn selected_item_style(&self) -> Style {
        Style::default()
            .bg(Color::Rgb(40, 40, 40))
            .fg(self.text_color)
            .add_modifier(Modifier::BOLD)
    }

    pub fn gradient_colors(&self) -> Vec<Color> {
        vec![
            Color::Rgb(0, 100, 25),    // Dark green
            Color::Rgb(0, 150, 35),    // Medium green
            Color::Rgb(0, 215, 53),    // Letterboxd green
            Color::Rgb(50, 235, 83),   // Light green
            Color::Rgb(100, 255, 133), // Very light green
        ]
    }

    pub fn border_type(&self) -> BorderType {
        BorderType::Rounded
    }
}