use anyhow::Result;
use colored::*;
use reqwest;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;
use tokio::time::{timeout, Duration};

pub struct AsciiConverter {
    client: reqwest::Client,
}

impl Default for AsciiConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl AsciiConverter {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self { client }
    }

    /// Get the correct Python executable name for the current platform
    fn python_executable() -> &'static str {
        if cfg!(windows) {
            "python"
        } else {
            "python3"
        }
    }

    pub fn detect_terminal_colors() -> bool {
        // Check TERM environment variable
        if let Ok(term) = std::env::var("TERM") {
            if term.contains("256color") || term.contains("color") {
                return true;
            }
        }

        // Check tput colors command
        if let Ok(output) = Command::new("tput").arg("colors").output() {
            if output.status.success() {
                if let Ok(colors_str) = String::from_utf8(output.stdout) {
                    if let Ok(colors) = colors_str.trim().parse::<i32>() {
                        return colors >= 8;
                    }
                }
            }
        }

        // Check for common color-supporting terminals
        if let Ok(term) = std::env::var("TERM") {
            let color_terms = [
                "xterm", "screen", "tmux", "rxvt", "gnome", "konsole", "iterm",
            ];
            return color_terms
                .iter()
                .any(|&color_term| term.to_lowercase().contains(color_term));
        }

        false
    }

    pub async fn convert_poster_to_ascii(
        &self,
        poster_url: &str,
        width: u32,
    ) -> Result<(String, f32)> {
        let image_data = self.fetch_image(poster_url).await?;
        let (ascii_art, aspect_ratio) = self.image_to_ascii_python(&image_data, width)?;
        Ok((ascii_art, aspect_ratio))
    }

    async fn fetch_image(&self, url: &str) -> Result<Vec<u8>> {
        let response = timeout(Duration::from_secs(5), self.client.get(url).send()).await??;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to fetch image: HTTP {}",
                response.status()
            ));
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    fn image_to_ascii_python(&self, image_data: &[u8], width: u32) -> Result<(String, f32)> {
        // Create temporary file for input image
        let mut temp_input = NamedTempFile::new()
            .map_err(|e| anyhow::anyhow!("Failed to create temp input file: {}", e))?;

        std::io::Write::write_all(&mut temp_input, image_data)
            .map_err(|e| anyhow::anyhow!("Failed to write image data: {}", e))?;

        // Create temporary file for output ASCII
        let temp_output = NamedTempFile::new()
            .map_err(|e| anyhow::anyhow!("Failed to create temp output file: {}", e))?;

        // Create temporary file for aspect ratio
        let temp_aspect_ratio = NamedTempFile::new()
            .map_err(|e| anyhow::anyhow!("Failed to create temp aspect ratio file: {}", e))?;

        let input_path = temp_input.path().to_string_lossy();
        let output_path = temp_output.path().to_string_lossy();
        let aspect_ratio_path = temp_aspect_ratio.path().to_string_lossy();

        // Get the Python script path relative to the binary
        let python_script_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/python");
        let script_path = python_script_dir.join("ascii_converter.py");

        // Check if terminal supports colors
        let supports_colors = Self::detect_terminal_colors();

        // Build Python command with reduced scale for better terminal display
        let mut cmd = Command::new(Self::python_executable());
        cmd.arg(&script_path)
            .arg("--input")
            .arg(&*input_path)
            .arg("--output")
            .arg(&*output_path)
            .arg("--aspect_ratio_file")
            .arg(&*aspect_ratio_path)
            .arg("--num_cols")
            .arg(width.to_string())
            .arg("--scale")
            .arg("1") // Reduced from default 2 to 1 for better height
            .arg("--background")
            .arg("black")
            .arg("--mode")
            .arg("blocks") // Use Unicode block characters for better compactness
            .arg("--square"); // Force images to 1:1 aspect ratio for ASCII display

        if supports_colors {
            cmd.arg("--color_output");
        }

        // Execute Python script
        let output = cmd
            .output()
            .map_err(|e| anyhow::anyhow!("Failed to execute Python script: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Python script failed: {}", stderr));
        }

        // Read the generated ASCII art
        let ascii_content = fs::read_to_string(&*output_path)
            .map_err(|e| anyhow::anyhow!("Failed to read ASCII output: {}", e))?;

        // Read the aspect ratio
        let aspect_ratio_str = fs::read_to_string(&*aspect_ratio_path)
            .map_err(|e| anyhow::anyhow!("Failed to read aspect ratio: {}", e))?;
        let aspect_ratio: f32 = aspect_ratio_str
            .trim()
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse aspect ratio: {}", e))?;

        Ok((ascii_content, aspect_ratio))
    }

    pub fn create_letterboxd_logo() -> String {
        format!(
            "{}{}{}",
            "✽".bright_yellow(), // Yellow instead of orange
            "✽".bright_green(),  // Green
            "✽".bright_blue()    // Blue
        )
    }

    pub fn create_minimal_header() -> String {
        format!("{} lbxd", Self::create_letterboxd_logo())
    }

    pub fn create_colored_triple_stars() -> String {
        format!(
            "{}{}{}",
            "✽".bright_yellow(), // Yellow instead of orange
            "✽".bright_green(),  // Green
            "✽".bright_blue()    // Blue
        )
    }

    pub fn create_activity_header(username: &str) -> String {
        format!("{} Activity", username.white().bold())
    }

    pub fn get_fallback_poster_ascii(width: u32) -> String {
        let height = (width as f32 * 1.5) as u32;
        let mut result = String::new();

        for y in 0..height {
            for x in 0..width {
                if y == 0 || y == height - 1 || x == 0 || x == width - 1 {
                    result.push('█');
                } else if y == height / 2 && x == width / 2 {
                    result.push('▢');
                } else {
                    result.push(' ');
                }
            }
            result.push('\n');
        }

        result
    }

    pub fn get_colored_fallback_poster_ascii(width: u32) -> String {
        let height = (width as f32 * 1.5) as u32; // Movie poster aspect ratio (2:3)
        let mut result = String::new();

        // ANSI colors: yellow, green, blue
        let colors = [
            |s: &str| s.bright_yellow(),
            |s: &str| s.bright_green(),
            |s: &str| s.bright_blue(),
        ];

        // Create a more sophisticated pattern
        for y in 0..height {
            for x in 0..width {
                let color_idx = (((x + y) / 3) % 3) as usize;

                if y == 0 || y == height - 1 || x == 0 || x == width - 1 {
                    // Border with gradient effect
                    let char = match (x + y) % 4 {
                        0 => "╭",
                        1 => "─",
                        2 => "╮",
                        _ => "│",
                    };
                    result.push_str(&colors[color_idx](char).to_string());
                } else if (y == height / 4 && x >= width / 3 && x < 2 * width / 3)
                    || (y == height / 2 && x >= width / 4 && x < 3 * width / 4)
                    || (y == 3 * height / 4 && x >= width / 3 && x < 2 * width / 3)
                {
                    // Central pattern with ✽ symbols
                    result.push_str(&colors[color_idx]("✽").bold().to_string());
                } else if (x + y) % 8 == 0 {
                    // Subtle background pattern
                    result.push_str(&colors[color_idx]("·").dimmed().to_string());
                } else {
                    result.push(' ');
                }
            }
            result.push('\n');
        }

        result
    }

    pub fn create_gradient_border(width: usize, style: &str) -> String {
        let colors = [
            |s: &str| s.bright_yellow(),
            |s: &str| s.bright_green(),
            |s: &str| s.bright_blue(),
        ];
        let mut result = String::new();

        for i in 0..width {
            let color_idx = (i * 3 / width.max(1)) % 3;
            result.push_str(&colors[color_idx](style).to_string());
        }

        result
    }

    pub fn get_dynamic_poster_size(terminal_width: u32) -> (u32, u32) {
        // Dramatically increased sizes for much more detailed ASCII art
        let width = match terminal_width {
            0..=80 => 40,    // Minimum 40px even for small terminals
            81..=120 => 60,  // Standard quality
            121..=160 => 70, // High quality
            161..=200 => 80, // Ultra quality
            _ => 90,         // Maximum detail for very large terminals
        };

        let height = (width as f32 * 1.5) as u32; // Movie poster aspect ratio (2:3)
        (width, height)
    }

    pub fn get_optimal_poster_size(width: u32, aspect_ratio: Option<f32>) -> (u32, u32) {
        let height = if let Some(ratio) = aspect_ratio {
            // Use original aspect ratio - Python script handles character compensation
            (width as f32 / ratio) as u32
        } else {
            // Default fallback for movie posters (typical 2:3 ratio means height = width * 1.5)
            (width as f32 * 1.5) as u32
        };
        (width, height)
    }
}
