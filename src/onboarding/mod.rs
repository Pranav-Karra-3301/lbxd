use crate::config::{ColorMode, ConfigManager, DisplayMode};
use anyhow::Result;
use colored::*;
use std::io::{self, Write};

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

        // Step 2: Test terminal colors and get preference
        let color_mode = self.test_terminal_colors_advanced()?;
        self.config_manager.set_color_mode(color_mode)?;

        // Step 3: Get poster preference
        let display_mode = self.get_poster_preference()?;
        self.config_manager.set_display_mode(display_mode)?;

        // Step 4: Setup complete
        self.show_setup_complete().await;

        Ok(())
    }

    fn show_welcome_screen(&self) {
        println!("\n{}", "═".repeat(60));
        println!("{}", self.create_letterboxd_ascii_art());
        println!("{}", "═".repeat(60));
        println!(
            "\n{}",
            "Welcome to Letterboxd in your terminal!"
                .bright_white()
                .bold()
        );
        println!("{}", "Let's set up your preferences...".dimmed());
        println!();
    }

    fn create_letterboxd_ascii_art(&self) -> String {
        let art = r#"
██╗     ██████╗ ██╗  ██╗██████╗ 
██║     ██╔══██╗╚██╗██╔╝██╔══██╗
██║     ██████╔╝ ╚███╔╝ ██║  ██║
██║     ██╔══██╗ ██╔██╗ ██║  ██║
███████╗██████╔╝██╔╝ ██╗██████╔╝
╚══════╝╚═════╝ ╚═╝  ╚═╝╚═════╝
"#;

        // Always use colors during onboarding since we're testing them
        format!(
            "{}

            Welcome to lbxd",
            art.bright_yellow()
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
                println!(
                    "{}",
                    "Username cannot contain spaces. Please try again.".red()
                );
                continue;
            }

            return Ok(username);
        }
    }

    fn show_config_saved(&self) {
        println!("{} {}", "✓".green().bold(), "Config saved".green());
        println!();
    }

    fn test_terminal_colors_advanced(&self) -> Result<ColorMode> {
        println!("{}", "Testing terminal color support...".bright_yellow());
        println!();

        // Test different color sets
        println!("{} {}", "Set 1:".white().bold(), "ANSI Colors");
        println!(
            "  {} {} {} {} {} {} {} {}",
            "●".red(),
            "●".green(),
            "●".yellow(),
            "●".blue(),
            "●".magenta(),
            "●".cyan(),
            "●".white(),
            "●".black()
        );

        println!("{} {}", "Set 2:".white().bold(), "Bright Colors");
        println!(
            "  {} {} {} {}",
            "★".bright_red(),
            "★".bright_green(),
            "★".bright_yellow(),
            "★".bright_blue()
        );

        println!("{} {}", "Set 3:".white().bold(), "Theme Colors");
        println!("  {} {} {}", "✽".red(), "✽".green(), "✽".blue());

        println!();

        // First ask which sets work
        let colors_work = loop {
            println!("{}", "Which color sets can you see clearly?".bright_cyan());
            println!("  {} All sets are clearly visible", "[all]".bright_green());
            println!("  {} Some sets are visible", "[some]".bright_yellow());
            println!(
                "  {} No colors visible (black and white only)",
                "[none]".white()
            );

            print!("{} ", "Your choice (all/some/none):".bright_cyan());
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let response = input.trim().to_lowercase();

            match response.as_str() {
                "all" => {
                    println!(
                        "{} {}",
                        "✓".green().bold(),
                        "Great! Your terminal has excellent color support.".green()
                    );
                    break true;
                }
                "some" => {
                    println!(
                        "{} {}",
                        "✓".yellow().bold(),
                        "Good! Your terminal has basic color support.".yellow()
                    );
                    break true;
                }
                "none" => {
                    println!(
                        "{} {}",
                        "ℹ".white().bold(),
                        "No problem! We'll use grayscale mode.".white()
                    );
                    return Ok(ColorMode::Grayscale);
                }
                _ => println!("{}", "Please enter 'all', 'some', or 'none'.".red()),
            }
        };

        // If colors work, ask for preference
        if colors_work {
            println!();
            loop {
                println!("{}", "Color preference:".bright_cyan());
                println!("  {} Use colors (recommended)", "[color]".bright_green());
                println!(
                    "  {} Use grayscale (better compatibility)",
                    "[grayscale]".white()
                );

                print!("{} ", "Your choice (color/grayscale):".bright_cyan());
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let response = input.trim().to_lowercase();

                match response.as_str() {
                    "color" | "c" => {
                        println!("{} {}", "✓".green().bold(), "Color mode selected.".green());
                        return Ok(ColorMode::Color);
                    }
                    "grayscale" | "g" => {
                        println!(
                            "{} {}",
                            "✓".white().bold(),
                            "Grayscale mode selected.".white()
                        );
                        return Ok(ColorMode::Grayscale);
                    }
                    _ => println!("{}", "Please enter 'color' or 'grayscale'.".red()),
                }
            }
        } else {
            Ok(ColorMode::Grayscale)
        }
    }

    fn get_poster_preference(&self) -> Result<DisplayMode> {
        println!();
        println!("{}", "Choose your poster display preference:".bright_cyan());
        println!(
            "  {} {}",
            "[p]".bright_green(),
            "Pixelated terminal-friendly posters (recommended)"
        );
        println!(
            "  {} {}",
            "[f]".bright_green(),
            "Full resolution posters (requires good terminal)"
        );
        println!();

        loop {
            print!("{} ", "Your choice (p/f):".bright_cyan());
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let response = input.trim().to_lowercase();

            match response.as_str() {
                "p" | "pixelated" => {
                    println!(
                        "{} {}",
                        "✓".green().bold(),
                        "Pixelated mode selected.".green()
                    );
                    return Ok(DisplayMode::Pixelated);
                }
                "f" | "full" => {
                    println!(
                        "{} {}",
                        "✓".green().bold(),
                        "Full resolution mode selected.".green()
                    );
                    return Ok(DisplayMode::FullResolution);
                }
                _ => println!(
                    "{}",
                    "Please enter 'p' for pixelated or 'f' for full resolution.".red()
                ),
            }
        }
    }

    async fn show_setup_complete(&self) {
        println!();
        println!(
            "{} {}",
            "✓".green().bold(),
            "Setup complete!".green().bold()
        );
        println!();

        println!("{}", "═".repeat(50));
        println!("{}", "Getting started:".bright_white().bold());
        println!(
            "  {} {}",
            "•".bright_blue(),
            "Run 'lbxd --help' to see all available commands"
        );
        println!(
            "  {} {}",
            "•".bright_blue(),
            "Try 'lbxd recent me' to see your recent activity"
        );
        println!(
            "  {} {}",
            "•".bright_blue(),
            "Use 'lbxd movie \"movie title\"' to search for movies"
        );
        println!(
            "  {} {}",
            "•".bright_blue(),
            "Run 'lbxd --reconfig' to change these settings anytime"
        );
        println!("{}", "═".repeat(50));
        println!();

        // Show final welcome with ASCII art
        self.show_final_welcome().await;
    }

    async fn show_final_welcome(&self) {
        // Create a simple letterboxd-style welcome
        let welcome_art = format!(
            "{}  Welcome to Letterboxd  {}",
            "✽".bright_yellow(),
            "✽".bright_blue()
        );

        println!("{}", welcome_art);

        let subtitle = format!(
            "{}{}{}",
            "in ASCII art! ".bright_green(),
            "Enjoy exploring movies ".bright_yellow(),
            "in your terminal.".bright_blue()
        );

        println!("{}", subtitle);
        println!();
    }

    pub fn should_run_onboarding(&self) -> bool {
        self.config_manager.is_first_run()
    }
}
