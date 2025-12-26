# Usage

lbxd provides several commands to interact with Letterboxd data from your terminal.

## Quick Start

```bash
# Show version and ASCII art
lbxd

# View profile stats for a user
lbxd <username>

# Interactive browsing mode
lbxd browse <username>

# Show recent activity
lbxd recent <username>
```

## Commands

### Profile Stats

Display a user's profile statistics:

```bash
lbxd <username>
```

This shows total films, films this year, lists count, followers/following, and favorite films.

### Interactive TUI Mode

Launch the interactive terminal user interface to browse complete collections:

```bash
lbxd browse <username>
```

**TUI Keyboard Shortcuts:**
- `j/k` or `↑/↓` - Navigate up/down
- `Page Up/Down` - Scroll by page
- `g/G` - Go to top/bottom
- `Tab` or `1-3` - Switch tabs (Movies, Watchlist, Statistics)
- `s` - Cycle sort mode (Date, Rating, Title, Year)
- `p/P` - Load movie poster
- `/` - Open search
- `q` or `Esc` - Quit

### Recent Activity

Show a user's recent movie activity:

```bash
lbxd recent <username> [OPTIONS]
```

**Options:**
- `-l, --limit <N>` - Number of entries to show (default: 3)
- `-d, --date <YYYY-MM-DD>` - Filter by specific date
- `-r, --rated` - Show only rated films
- `-w, --reviewed` - Show only reviewed films
- `-v, --vertical` - Display in vertical layout
- `--width <30-120>` - Width in characters (default: 45)

**Examples:**
```bash
# Show last 10 entries
lbxd recent johndoe --limit 10

# Show only rated films
lbxd recent johndoe --rated

# Show only reviewed films
lbxd recent johndoe --reviewed

# Filter by specific date
lbxd recent johndoe --date 2024-01-15

# Use 'me' alias for saved username
lbxd recent me
```

### Search User Activity

Search for specific movies in a user's watch history:

```bash
lbxd search <username> <title> [OPTIONS]
```

**Options:**
- `--width <30-120>` - Width in characters (default: 45)

**Example:**
```bash
lbxd search johndoe "blade runner"
```

### Movie Database Search

Search TMDB for movie information:

```bash
lbxd movie <title> [OPTIONS]
```

**Options:**
- `--width <30-120>` - Width in characters (default: 45)

**Examples:**
```bash
lbxd movie "The Godfather"
lbxd movie "dune 2021"
```

### Data Export

Export user data to JSON, Markdown, or CSV formats:

```bash
lbxd export <username> --format <json|markdown|csv> --output <file>
```

**Options:**
- `-f, --format <json|markdown|csv>` - Output format (required)
- `-o, --output <file>` - Output file path (required)

**Examples:**
```bash
# Export to JSON
lbxd export johndoe --format json --output movies.json

# Export to Markdown
lbxd export johndoe --format markdown --output report.md

# Export to CSV (for spreadsheets)
lbxd export johndoe --format csv --output data.csv
```

### Compare Users

Compare viewing statistics between multiple users:

```bash
lbxd compare <username1> <username2> [username3...]
```

**Examples:**
```bash
# Compare two users
lbxd compare alice bob

# Compare multiple users
lbxd compare alice bob charlie
```

This shows a comparison table with total films, average ratings, and reviews for each user.

### Viewing Summary

Generate a yearly viewing summary with statistics:

```bash
lbxd summary <username> [--year <year>]
```

**Options:**
- `-y, --year <year>` - Year for summary (defaults to current year)

**Examples:**
```bash
# Current year summary
lbxd summary johndoe

# Summary for specific year
lbxd summary me --year 2024
```

Shows total films, average rating, reviews written, liked films, top rated films, and monthly breakdown.

### Configuration

Manage lbxd settings:

```bash
lbxd config <subcommand>
```

**Subcommands:**

#### Show Current Settings
```bash
lbxd config show
```

#### Set Default Username
```bash
lbxd config set-user <username>
```

#### Check Saved Username
```bash
lbxd config whoami
```

#### Switch Color Mode
```bash
lbxd config switch-color <color|grayscale>
```

#### Set Display Mode
```bash
lbxd config set-mode <pixelated|full>
```

#### Clear Cache
```bash
lbxd config clear-cache
```

Clear all cached user data to fetch fresh data on next request.

#### Show File Paths
```bash
lbxd config paths
```

Shows the locations of config and cache files on your system.

### Reconfigure

Run the interactive setup wizard again:

```bash
lbxd --reconfig
```

## The "me" Alias

Once you've set a default username (during onboarding or via `lbxd config set-user`), you can use `me` as a shortcut:

```bash
lbxd recent me
lbxd browse me
lbxd search me "inception"
lbxd export me --format json --output my-movies.json
```

## Caching

lbxd automatically caches user data in `~/.cache/lbxd/` for faster access. Cache entries expire after 6 hours.

## Configuration Files

- **Config location:** `~/.config/lbxd/config.json`
- **Cache location:** `~/.cache/lbxd/`

## API Keys

lbxd comes with built-in API keys for TMDB and OMDB, so it works out of the box. To use your own keys:

```bash
export TMDB_API_KEY="your_key_here"
export OMDB_API_KEY="your_key_here"
```

## Examples

### Complete Workflow

```bash
# First run - interactive setup
lbxd

# View your profile stats
lbxd me

# Browse your collection interactively
lbxd browse me

# View recent activity
lbxd recent me --limit 10

# Search for specific movies
lbxd search me "christopher nolan"

# Search TMDB database
lbxd movie "Oppenheimer"

# Export your data
lbxd export me --format json --output backup.json

# Change settings
lbxd config show
lbxd config switch-color grayscale
lbxd config set-mode full
```
