use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lbxd")]
#[command(about = "A beautiful command-line tool for Letterboxd")]
#[command(long_about = "LBXD - Letterboxd in your terminal\n\nA beautiful CLI tool featuring:\n★ Real movie poster display with viu\n◆ Colorful ASCII art mode with Unicode blocks\n▲ TMDB integration for reliable data\n● Dynamic terminal adaptation\n◉ Smooth loading animations\n\nDeveloped by https://pranavkarra.me")]
#[command(version)]
pub struct Cli {
    #[arg(long, help = "Reconfigure settings through interactive setup")]
    pub reconfig: bool,
    #[arg(help = "Show profile stats for username (or use subcommands)")]
    pub username: Option<String>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "★ Show recent activity for a user")]
    Recent {
        #[arg(help = "Letterboxd username")]
        username: String,
        #[arg(short, long, help = "Number of entries to show", default_value = "3")]
        limit: Option<usize>,
        #[arg(short, long, help = "Filter by date (YYYY-MM-DD)")]
        date: Option<String>,
        #[arg(short, long, help = "Show only rated films")]
        rated: bool,
        #[arg(short = 'w', long, help = "Show only reviewed films")]
        reviewed: bool,
        #[arg(short = 'v', long, help = "Display in vertical layout")]
        vertical: bool,
        #[arg(long, help = "Use ASCII art instead of viu (terminal image viewer)")]
        ascii: bool,
        #[arg(long, help = "Width in characters (30-120)", value_parser = clap::value_parser!(u32).range(30..=120), default_value = "45")]
        width: u32,
    },
    #[command(about = "◆ Search for specific titles in user history")]
    Search {
        #[arg(help = "Letterboxd username")]
        username: String,
        #[arg(help = "Movie title to search for")]
        title: String,
        #[arg(long, help = "Use ASCII art instead of viu (terminal image viewer)")]
        ascii: bool,
        #[arg(long, help = "Width in characters (30-120)", value_parser = clap::value_parser!(u32).range(30..=120), default_value = "45")]
        width: u32,
    },
    #[command(about = "▲ Compare multiple users (coming soon)")]
    Compare {
        #[arg(help = "Letterboxd usernames", num_args = 2..)]
        usernames: Vec<String>,
    },
    #[command(about = "● Export data to JSON/Markdown")]
    Export {
        #[arg(help = "Letterboxd username")]
        username: String,
        #[arg(short, long, help = "Output format", value_enum)]
        format: ExportFormat,
        #[arg(short, long, help = "Output file path")]
        output: String,
    },
    #[command(about = "◉ Generate viewing summary (coming soon)")]
    Summary {
        #[arg(help = "Letterboxd username")]
        username: String,
        #[arg(short, long, help = "Year for summary")]
        year: Option<i32>,
    },
    #[command(about = "✽ Search for movies using TMDB database")]
    Movie {
        #[arg(help = "Movie title to search for")]
        title: String,
        #[arg(long, help = "Use ASCII art instead of viu (terminal image viewer)")]
        ascii: bool,
        #[arg(short, long, help = "Width in characters (30-120)", value_parser = clap::value_parser!(u32).range(30..=120), default_value = "45")]
        width: u32,
    },
    #[command(about = "⚙ Manage user configuration settings")]
    Config {
        #[command(subcommand)]
        config_command: ConfigCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    #[command(about = "ℹ Show current saved username")]
    Whoami,
    #[command(about = "✍ Set username for 'me' alias")]
    SetUser {
        #[arg(help = "Username to save")]
        username: String,
    },
    #[command(about = "⚙ Show all configuration settings")]
    Show,
    #[command(about = "🎨 Switch between color and grayscale mode")]
    SwitchColor {
        #[arg(help = "Color mode (color/grayscale)", value_enum)]
        mode: ColorModeArg,
    },
    #[command(about = "🖼 Switch between pixelated and full resolution posters")]
    SetMode {
        #[arg(help = "Display mode (pixelated/full)", value_enum)]
        mode: DisplayModeArg,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ColorModeArg {
    Color,
    Grayscale,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum DisplayModeArg {
    Pixelated,
    Full,
}

#[derive(clap::ValueEnum, Clone)]
pub enum ExportFormat {
    Json,
    Markdown,
}