use anyhow::Result;
use artem::{config::ConfigBuilder, convert};
use reqwest;
use std::num::NonZeroU32;
use tokio::time::{timeout, Duration};
use image::{imageops::FilterType, DynamicImage};
use terminal_size::{Width, Height, terminal_size};
use colored::*;

pub struct AsciiConverter {
    client: reqwest::Client,
}

#[derive(Debug, Clone)]
pub enum QualityPreset {
    Low,    // Small terminals, simple characters
    Medium, // Standard quality
    High,   // Large terminals, detailed characters  
    Ultra,  // Maximum detail and smoothness
}

#[derive(Debug, Clone)]
pub struct PosterConfig {
    pub width: u32,
    pub height: u32,
    pub quality: QualityPreset,
    pub enhance_contrast: bool,
    pub normalize_brightness: bool,
}

impl AsciiConverter {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();
            
        Self { client }
    }

    pub async fn convert_poster_to_ascii(&self, poster_url: &str, width: u32) -> Result<String> {
        let image_data = self.fetch_image(poster_url).await?;
        let ascii_art = self.image_to_ascii(&image_data, width)?;
        Ok(ascii_art)
    }

    async fn fetch_image(&self, url: &str) -> Result<Vec<u8>> {
        let response = timeout(Duration::from_secs(5), self.client.get(url).send()).await??;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to fetch image: HTTP {}", response.status()));
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    fn image_to_ascii(&self, image_data: &[u8], width: u32) -> Result<String> {
        let img = image::load_from_memory(image_data)
            .map_err(|e| anyhow::anyhow!("Failed to load image: {}", e))?;
        
        // Determine quality based on terminal size and width
        let term_width = if let Some((Width(w), Height(_))) = terminal_size() {
            w as u32
        } else {
            80
        };
        
        let quality = match (term_width, width) {
            (w, _) if w < 60 => QualityPreset::Low,
            (w, size) if w < 100 || size < 25 => QualityPreset::Medium,
            (w, size) if w < 140 || size < 35 => QualityPreset::High,
            _ => QualityPreset::Ultra,
        };
        
        let config = PosterConfig {
            width,
            height: (width as f32 * 1.5) as u32, // Movie poster aspect ratio
            quality,
            enhance_contrast: true,
            normalize_brightness: true,
        };
        
        self.convert_with_config(img, &config)
    }
    
    fn convert_with_config(&self, mut img: DynamicImage, config: &PosterConfig) -> Result<String> {
        // Preprocess image for better ASCII conversion
        if config.enhance_contrast {
            img = img.adjust_contrast(15.0);
        }
        
        if config.normalize_brightness {
            img = img.brighten(10);
        }
        
        // Resize with high-quality filtering
        let img = img.resize_exact(config.width, config.height, FilterType::Lanczos3);
        
        let target_size = NonZeroU32::new(config.width)
            .unwrap_or(NonZeroU32::new(30).unwrap());
            
        let characters = match config.quality {
            QualityPreset::Low => " .-+*#".to_string(),
            QualityPreset::Medium => " .:-=+*#%@".to_string(),
            QualityPreset::High => " ░▒▓█".to_string(),
            QualityPreset::Ultra => " ·∘∙•⦁⦿●█".to_string(),
        };
        
        let artem_config = ConfigBuilder::new()
            .target_size(target_size)
            .characters(characters)
            .color(false)
            .invert(false)
            .hysteresis(true) // Better edge detection
            .build();

        let ascii_art = convert(img, &artem_config);
        Ok(ascii_art)
    }

    pub fn create_letterboxd_logo() -> String {
        format!("{}{}{}", 
            "✽".color("#ff8000"),  // Orange
            "✽".color("#00d735"),  // Green  
            "✽".color("#40bcf4")   // Blue
        )
    }

    pub fn create_minimal_header() -> String {
        format!("{} lbxd", Self::create_letterboxd_logo())
    }
    
    pub fn create_activity_header(username: &str) -> String {
        format!("{} Activity", username.white().bold())
    }

    pub fn get_fallback_poster_ascii(width: u32) -> String {
        let height = (width as f32 * 0.6) as u32;
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
        let height = (width as f32 * 1.5) as u32; // Proper movie poster ratio
        let mut result = String::new();
        
        // Letterboxd colors: orange (#ff8000), green (#00d735), blue (#40bcf4)
        let colors = ["#ff8000", "#00d735", "#40bcf4"];
        
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
                    result.push_str(&char.color(colors[color_idx]).to_string());
                } else if (y == height / 4 && x >= width / 3 && x < 2 * width / 3) ||
                         (y == height / 2 && x >= width / 4 && x < 3 * width / 4) ||
                         (y == 3 * height / 4 && x >= width / 3 && x < 2 * width / 3) {
                    // Central pattern with ✽ symbols
                    result.push_str(&"✽".color(colors[color_idx]).bold().to_string());
                } else if (x + y) % 8 == 0 {
                    // Subtle background pattern
                    result.push_str(&"·".color(colors[color_idx]).dimmed().to_string());
                } else {
                    result.push(' ');
                }
            }
            result.push('\n');
        }
        
        result
    }
    
    pub fn create_gradient_border(width: usize, style: &str) -> String {
        let colors = ["#ff8000", "#00d735", "#40bcf4"];
        let mut result = String::new();
        
        for i in 0..width {
            let color_idx = (i * 3 / width.max(1)) % 3;
            result.push_str(&style.color(colors[color_idx]).to_string());
        }
        
        result
    }
    
    pub fn get_dynamic_poster_size(terminal_width: u32) -> (u32, u32) {
        let width = match terminal_width {
            0..=60 => 20,
            61..=100 => 25, 
            101..=140 => 30,
            141..=180 => 35,
            _ => 40,
        };
        
        let height = (width as f32 * 1.5) as u32; // Movie poster aspect ratio
        (width, height)
    }
}