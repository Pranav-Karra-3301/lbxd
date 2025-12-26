use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lbxd")]
#[command(about = "A beautiful command-line tool for Letterboxd")]
#[command(
    long_about = "LBXD - Letterboxd in your terminal\n\nA beautiful CLI tool featuring:\n★ Recent activity viewing with rich display\n◆ Movie search and filtering capabilities\n▲ TMDB integration for reliable data\n● Export functionality for data analysis\n◉ Smooth loading animations\n🎭 Interactive TUI for browsing complete collections\n\nDeveloped by https://pranavkarra.me"
)]
#[command(version)]
pub struct Cli {
    #[arg(long, help = "Reconfigure settings through interactive setup")]
    pub reconfig: bool,
    #[arg(help = "Show profile stats for username (or use 'browse' for interactive TUI)")]
    pub username: Option<String>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "★ Show recent activity for a user",
        long_about = "★ Show recent activity for a user\n\nExamples:\n  lbxd recent johndoe\n  lbxd recent me --limit 10\n  lbxd recent johndoe --rated\n  lbxd recent johndoe --date 2024-01-15"
    )]
    Recent {
        #[arg(help = "Letterboxd username (use 'me' for saved username)")]
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
        #[arg(long, help = "Width in characters (30-120)", value_parser = clap::value_parser!(u32).range(30..=120), default_value = "45")]
        width: u32,
    },
    #[command(
        about = "◆ Search for specific titles in user history",
        long_about = "◆ Search for specific titles in user history\n\nExamples:\n  lbxd search johndoe \"blade runner\"\n  lbxd search me \"inception\""
    )]
    Search {
        #[arg(help = "Letterboxd username (use 'me' for saved username)")]
        username: String,
        #[arg(help = "Movie title to search for")]
        title: String,
        #[arg(long, help = "Width in characters (30-120)", value_parser = clap::value_parser!(u32).range(30..=120), default_value = "45")]
        width: u32,
    },
    #[command(
        about = "▲ Compare multiple users' film stats",
        long_about = "▲ Compare multiple users' film stats\n\nCompare viewing statistics between two or more Letterboxd users.\n\nExamples:\n  lbxd compare user1 user2\n  lbxd compare alice bob charlie"
    )]
    Compare {
        #[arg(help = "Letterboxd usernames to compare", num_args = 2..)]
        usernames: Vec<String>,
    },
    #[command(
        about = "● Export data to JSON/Markdown/CSV",
        long_about = "● Export data to JSON/Markdown/CSV\n\nExamples:\n  lbxd export johndoe -f json -o movies.json\n  lbxd export me -f markdown -o report.md\n  lbxd export johndoe -f csv -o data.csv"
    )]
    Export {
        #[arg(help = "Letterboxd username (use 'me' for saved username)")]
        username: String,
        #[arg(short, long, help = "Output format (json, markdown, csv)", value_enum)]
        format: ExportFormat,
        #[arg(short, long, help = "Output file path")]
        output: String,
    },
    #[command(
        about = "◉ Generate viewing summary for a year",
        long_about = "◉ Generate viewing summary for a year\n\nShow statistics and top films for a specific year.\n\nExamples:\n  lbxd summary johndoe\n  lbxd summary me --year 2024"
    )]
    Summary {
        #[arg(help = "Letterboxd username (use 'me' for saved username)")]
        username: String,
        #[arg(short, long, help = "Year for summary (defaults to current year)")]
        year: Option<i32>,
    },
    #[command(
        about = "✽ Search for movies using TMDB database",
        long_about = "✽ Search for movies using TMDB database\n\nSearch The Movie Database for movie information.\n\nExamples:\n  lbxd movie \"The Godfather\"\n  lbxd movie \"dune 2021\"\n  lbxd movie \"Oppenheimer\" --width 60"
    )]
    Movie {
        #[arg(help = "Movie title to search for")]
        title: String,
        #[arg(short, long, help = "Width in characters (30-120)", value_parser = clap::value_parser!(u32).range(30..=120), default_value = "45")]
        width: u32,
    },
    #[command(
        about = "⚙ Manage user configuration settings",
        long_about = "⚙ Manage user configuration settings\n\nSubcommands:\n  whoami       - Show saved username\n  set-user     - Set default username\n  show         - Show all settings\n  switch-color - Toggle color mode\n  set-mode     - Set poster display mode\n  clear-cache  - Clear cached data\n  paths        - Show config file locations"
    )]
    Config {
        #[command(subcommand)]
        config_command: ConfigCommands,
    },
    #[command(
        about = "🎭 Browse user's complete collection with interactive TUI",
        long_about = "🎭 Browse user's complete collection with interactive TUI\n\nKeyboard shortcuts:\n  j/k, ↑/↓   - Navigate\n  g/G        - Go to top/bottom\n  Tab, 1-3   - Switch tabs\n  s          - Cycle sort mode\n  p          - Load movie info\n  /          - Search\n  q, Esc     - Quit\n\nExamples:\n  lbxd browse johndoe\n  lbxd browse me"
    )]
    Browse {
        #[arg(help = "Letterboxd username (use 'me' for saved username)")]
        username: String,
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
    #[command(about = "🗑 Clear cached user data")]
    ClearCache,
    #[command(about = "📁 Show cache and config file locations")]
    Paths,
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
    Csv,
}
