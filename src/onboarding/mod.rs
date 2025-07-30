use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use crate::config::ConfigManager;

pub struct OnboardingManager {
    config_manager: ConfigManager,
}

impl OnboardingManager {
    pub fn new(config_manager: ConfigManager) -> Self {
        Self { config_manager }
    }

    pub async fn run_interactive_setup(&self) -> Result<()> {
        // Clear screen and show welcome
        self.show_welcome_screen();
        
        // Step 1: Get username
        let username = self.get_username_input()?;
        self.config_manager.set_username(username)?;
        self.show_config_saved();
        
        // Step 2: Test terminal colors
        let supports_colors = self.test_terminal_colors()?;
        self.show_color_test_result(supports_colors);
        
        // Step 3: Get poster preference
        let use_pixelated = self.get_poster_preference()?;
        self.config_manager.set_pixelated_mode(use_pixelated)?;
        
        // Step 4: Setup complete
        self.show_setup_complete().await;
        
        Ok(())
    }

    fn show_welcome_screen(&self) {
        println!("\n{}", "═".repeat(60));
        println!("{}", self.create_letterboxd_ascii_art());
        println!("{}", "═".repeat(60));
        println!("\n{}", "Welcome to Letterboxd in your terminal!".bright_white().bold());
        println!("{}", "Let's set up your preferences...".dimmed());
        println!();
    }

    fn create_letterboxd_ascii_art(&self) -> String {
        format!(
            "    {}  {}  {}
    {}  {}  {}
    {}  {}  {}

            Welcome to lbxd",
            "███".color("#ff8000"), "███".color("#00d735"), "███".color("#40bcf4"),
            "███".color("#ff8000"), "███".color("#00d735"), "███".color("#40bcf4"),
            "▒▒▒".color("#ff8000"), "▒▒▒".color("#00d735"), "▒▒▒".color("#40bcf4")
        )
    }

    fn get_username_input(&self) -> Result<String> {
        loop {
            print!("{} ", "Enter your Letterboxd username:".bright_cyan());
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let username = input.trim().to_string();
            
            if username.is_empty() {
                println!("{}", "Username cannot be empty. Please try again.".red());
                continue;
            }
            
            if username.contains(' ') {
                println!("{}", "Username cannot contain spaces. Please try again.".red());
                continue;
            }
            
            return Ok(username);
        }
    }

    fn show_config_saved(&self) {
        println!("{} {}", "✓".green().bold(), "Config saved".green());
        println!();
    }

    fn test_terminal_colors(&self) -> Result<bool> {
        println!("{}", "Testing terminal color support...".bright_yellow());
        println!();
        
        // Display colored text
        println!("  {} {} {}", "●".color("#ff8000"), "●".color("#00d735"), "●".color("#40bcf4"));
        println!("  {} {} {}", "Orange".color("#ff8000"), "Green".color("#00d735"), "Blue".color("#40bcf4"));
        println!();
        
        loop {
            print!("{} ", "Can you see these colors clearly? (y/n):".bright_cyan());
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let response = input.trim().to_lowercase();
            
            match response.as_str() {
                "y" | "yes" => return Ok(true),
                "n" | "no" => return Ok(false),
                _ => println!("{}", "Please enter 'y' for yes or 'n' for no.".red()),
            }
        }
    }

    fn show_color_test_result(&self, supports_colors: bool) {
        if supports_colors {
            println!("{} {}", "✓".green().bold(), "Great! Your terminal supports colors.".green());
        } else {
            println!("{} {}", "⚠".yellow().bold(), "Terminal will display in grayscale mode.".yellow());
        }
        println!();
    }

    fn get_poster_preference(&self) -> Result<bool> {
        println!("{}", "Choose your poster display preference:".bright_cyan());
        println!("  {} {}", "[p]".bright_green(), "Pixelated terminal-friendly posters (recommended)");
        println!("  {} {}", "[f]".bright_green(), "Full resolution posters");
        println!();
        
        loop {
            print!("{} ", "Your choice (p/f):".bright_cyan());
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let response = input.trim().to_lowercase();
            
            match response.as_str() {
                "p" | "pixelated" => return Ok(true),
                "f" | "full" => return Ok(false),
                _ => println!("{}", "Please enter 'p' for pixelated or 'f' for full resolution.".red()),
            }
        }
    }

    async fn show_setup_complete(&self) {
        println!("{} {}", "✓".green().bold(), "Setup complete!".green().bold());
        println!();
        
        println!("{}", "═".repeat(50));
        println!("{}", "Getting started:".bright_white().bold());
        println!("  {} {}", "•".bright_blue(), "Run 'lbxd --help' to see all available commands");
        println!("  {} {}", "•".bright_blue(), "Try 'lbxd recent me' to see your recent activity");
        println!("  {} {}", "•".bright_blue(), "Use 'lbxd movie \"movie title\"' to search for movies");
        println!("{}", "═".repeat(50));
        println!();
        
        // Show final welcome with ASCII art
        self.show_final_welcome().await;
    }

    async fn show_final_welcome(&self) {
        // Create a simple letterboxd-style welcome
        let welcome_art = format!(
            "{}  Welcome to Letterboxd  {}",
            "✽".color("#ff8000"),
            "✽".color("#40bcf4")
        );
        
        println!("{}", welcome_art);
        
        let subtitle = format!(
            "{}{}{}",
            "in ASCII art! ".color("#00d735"),
            "Enjoy exploring movies ".color("#ff8000"),
            "in your terminal.".color("#40bcf4")
        );
        
        println!("{}", subtitle);
        println!();
    }

    pub fn should_run_onboarding(&self) -> bool {
        self.config_manager.is_first_run()
    }
}