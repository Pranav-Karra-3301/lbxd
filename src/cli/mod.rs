use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lbxd")]
#[command(about = "A beautiful command-line tool for Letterboxd")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Show recent activity for a user")]
    Recent {
        #[arg(help = "Letterboxd username")]
        username: String,
        #[arg(short, long, help = "Number of entries to show")]
        limit: Option<usize>,
        #[arg(short, long, help = "Filter by date (YYYY-MM-DD)")]
        date: Option<String>,
        #[arg(short, long, help = "Show only rated films")]
        rated: bool,
        #[arg(short, long, help = "Show only reviewed films")]
        reviewed: bool,
    },
    #[command(about = "Search for specific titles")]
    Search {
        #[arg(help = "Letterboxd username")]
        username: String,
        #[arg(help = "Movie title to search for")]
        title: String,
    },
    #[command(about = "Compare multiple users")]
    Compare {
        #[arg(help = "Letterboxd usernames", num_args = 2..)]
        usernames: Vec<String>,
    },
    #[command(about = "Export data to file")]
    Export {
        #[arg(help = "Letterboxd username")]
        username: String,
        #[arg(short, long, help = "Output format", value_enum)]
        format: ExportFormat,
        #[arg(short, long, help = "Output file path")]
        output: String,
    },
    #[command(about = "Generate viewing summary")]
    Summary {
        #[arg(help = "Letterboxd username")]
        username: String,
        #[arg(short, long, help = "Year for summary")]
        year: Option<i32>,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum ExportFormat {
    Json,
    Markdown,
}