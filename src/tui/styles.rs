use ratatui::{
    style::{Color, Modifier, Style},
    widgets::BorderType,
};

pub struct AppStyles {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
    pub error_color: Color,
    pub text_color: Color,
    pub dim_text_color: Color,
    pub success_color: Color,
    pub warning_color: Color,
    pub letterboxd_green: Color,
    pub letterboxd_orange: Color,
    pub letterboxd_blue: Color,
}

impl Default for AppStyles {
    fn default() -> Self {
        Self::new()
    }
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
            success_color: Color::Rgb(0, 215, 53), // Same as Letterboxd green
            warning_color: Color::Rgb(255, 128, 0), // Same as Letterboxd orange
            letterboxd_green: Color::Rgb(0, 215, 53),
            letterboxd_orange: Color::Rgb(255, 128, 0),
            letterboxd_blue: Color::Rgb(64, 188, 244),
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

    pub fn header_style(&self) -> Style {
        Style::default()
            .fg(self.primary_color)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
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
            r if r >= 4.0 => self.primary_color,   // Green for high ratings
            r if r >= 3.0 => self.secondary_color, // Orange for medium ratings
            r if r >= 2.0 => Color::Yellow,        // Yellow for low-medium ratings
            _ => self.error_color,                 // Red for low ratings
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

    // Emoji and Unicode helpers
    pub fn star_rating(&self, rating: f32) -> String {
        let full_stars = rating as usize;
        let half_star = rating % 1.0 >= 0.5;
        let mut result = String::new();

        for i in 0..5 {
            if i < full_stars {
                result.push('★');
            } else if i == full_stars && half_star {
                result.push('☆'); // Half star (could be ★ with different styling)
            } else {
                result.push('☆');
            }
        }
        result
    }

    // Statistics styling
    pub fn stats_title_style(&self) -> Style {
        Style::default()
            .fg(self.letterboxd_green)
            .add_modifier(Modifier::BOLD)
    }

    pub fn stats_value_style(&self) -> Style {
        Style::default()
            .fg(self.text_color)
            .add_modifier(Modifier::BOLD)
    }

    pub fn stats_label_style(&self) -> Style {
        Style::default().fg(self.dim_text_color)
    }

    pub fn genre_emoji_style(&self) -> Style {
        Style::default().fg(self.letterboxd_orange)
    }

    pub fn progress_complete_style(&self) -> Style {
        Style::default().fg(self.success_color)
    }

    pub fn progress_partial_style(&self) -> Style {
        Style::default().fg(self.warning_color)
    }

    // Chart and visualization styles
    pub fn chart_bar_style(&self, percentage: f32) -> Style {
        let color = if percentage >= 75.0 {
            self.letterboxd_green
        } else if percentage >= 50.0 {
            self.letterboxd_orange
        } else if percentage >= 25.0 {
            Color::Yellow
        } else {
            self.dim_text_color
        };
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    }

    pub fn analytics_header_style(&self) -> Style {
        Style::default()
            .fg(self.letterboxd_blue)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    }

    // Enhanced movie grid styles
    pub fn genre_tag_style(&self, genre: &str) -> Style {
        let color = match genre.to_lowercase().as_str() {
            "action" | "thriller" => Color::Red,
            "comedy" => Color::Yellow,
            "drama" => self.letterboxd_blue,
            "horror" => Color::Magenta,
            "romance" => Color::LightRed,
            "sci-fi" | "science fiction" => Color::Cyan,
            "documentary" => self.letterboxd_green,
            _ => self.dim_text_color,
        };
        Style::default().fg(color)
    }

    pub fn decade_style(&self, decade: &str) -> Style {
        match decade {
            "2020s" => Style::default().fg(self.letterboxd_green),
            "2010s" => Style::default().fg(self.letterboxd_orange),
            "2000s" => Style::default().fg(self.letterboxd_blue),
            "1990s" => Style::default().fg(Color::Yellow),
            "1980s" => Style::default().fg(Color::Magenta),
            _ => Style::default().fg(self.dim_text_color),
        }
    }

    // Icon and emoji color styling
    pub fn emoji_style(&self) -> Style {
        Style::default().add_modifier(Modifier::BOLD)
    }

    pub fn highlight_style(&self) -> Style {
        Style::default()
            .fg(self.letterboxd_green)
            .bg(Color::Rgb(20, 20, 20))
            .add_modifier(Modifier::BOLD)
    }

    // Gradient colors for advanced visualizations
    pub fn rating_gradient_color(&self, rating: f32) -> Color {
        match rating {
            r if r >= 4.5 => Color::Rgb(0, 255, 100), // Bright green
            r if r >= 4.0 => Color::Rgb(0, 215, 53),  // Letterboxd green
            r if r >= 3.5 => Color::Rgb(100, 255, 0), // Yellow-green
            r if r >= 3.0 => Color::Rgb(255, 200, 0), // Yellow
            r if r >= 2.5 => Color::Rgb(255, 128, 0), // Orange
            r if r >= 2.0 => Color::Rgb(255, 100, 0), // Red-orange
            _ => Color::Rgb(220, 50, 47),             // Red
        }
    }

    pub fn viewing_time_color(&self, hours: f32) -> Color {
        match hours {
            h if h >= 100.0 => self.letterboxd_green,
            h if h >= 50.0 => self.letterboxd_orange,
            h if h >= 20.0 => Color::Yellow,
            _ => self.dim_text_color,
        }
    }
}
