use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::profile::{LoadingProgress, LoadingStage};
use super::AppStyles;

pub struct ProgressBar {
    progress: LoadingProgress,
}

impl ProgressBar {
    pub fn new(progress: LoadingProgress) -> Self {
        Self { progress }
    }

    pub fn render(&self, f: &mut Frame, area: Rect, styles: &AppStyles) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Stage info
                Constraint::Length(3), // Progress bar
                Constraint::Length(3), // Current action
            ])
            .split(area);

        // Stage info
        let stage_text = format!("Stage: {}", self.stage_name());
        let stage_paragraph = Paragraph::new(stage_text)
            .alignment(Alignment::Center)
            .style(styles.text_style());
        f.render_widget(stage_paragraph, chunks[0]);

        // Progress bar
        let ratio = if self.progress.total > 0 {
            (self.progress.current as f64 / self.progress.total as f64) * 100.0
        } else {
            0.0
        };

        let gauge = Gauge::default()
            .block(Block::default().borders(Borders::ALL).border_style(styles.border_style()))
            .gauge_style(styles.progress_bar_style())
            .ratio(ratio / 100.0)
            .label(format!("{:.1}%", ratio));
        f.render_widget(gauge, chunks[1]);

        // Current action
        let action_text = format!("{} ({}/{})", 
            self.progress.message, 
            self.progress.current, 
            self.progress.total
        );
        let action_paragraph = Paragraph::new(action_text)
            .alignment(Alignment::Center)
            .style(styles.dim_text_style());
        f.render_widget(action_paragraph, chunks[2]);
    }

    fn stage_name(&self) -> &'static str {
        match self.progress.stage {
            LoadingStage::Profile => "Loading Profile",
            LoadingStage::Diary => "Loading Film Diary",
            LoadingStage::Lists => "Loading Lists",
            LoadingStage::MovieDetails => "Loading Movie Details",
            LoadingStage::Complete => "Complete",
        }
    }
}

pub struct LoadingSpinner {
    frames: Vec<&'static str>,
    current_frame: usize,
}

impl LoadingSpinner {
    pub fn new() -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            current_frame: 0,
        }
    }

    pub fn next_frame(&mut self) -> &'static str {
        let frame = self.frames[self.current_frame];
        self.current_frame = (self.current_frame + 1) % self.frames.len();
        frame
    }

    pub fn render(&mut self, f: &mut Frame, area: Rect, styles: &AppStyles, message: &str) {
        let spinner_text = format!("{} {}", self.next_frame(), message);
        let paragraph = Paragraph::new(spinner_text)
            .alignment(Alignment::Center)
            .style(ratatui::style::Style::default().fg(styles.primary_color));
        f.render_widget(paragraph, area);
    }
}