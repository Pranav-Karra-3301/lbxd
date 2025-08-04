# Configuration

lbxd uses a configuration file to store user preferences and settings. This page covers all available configuration options.

## Configuration File Location

The configuration file is automatically created in the following locations:

- **Linux/macOS**: `~/.config/lbxd/config.toml`
- **Windows**: `%APPDATA%\lbxd\config.toml`

## Configuration Options

### Display Settings

```toml
[display]
# Number of items to show per page
page_size = 20

# Enable colored output
colored = true

# Show ASCII art headers
ascii_art = true

# Terminal width for formatting (0 = auto-detect)
width = 0
```

### Cache Settings

```toml
[cache]
# Enable caching
enabled = true

# Cache directory (empty = use default)
directory = ""

# Cache expiration in hours
ttl = 24

# Maximum cache size in MB
max_size = 100
```

### API Settings

```toml
[api]
# TMDB API key (optional, uses built-in default if not provided)
tmdb_key = ""

# OMDB API key (optional, uses built-in default if not provided)  
omdb_key = ""

# Request timeout in seconds
timeout = 30

# Maximum concurrent requests
max_concurrent = 5

# Rate limiting (requests per second)
rate_limit = 2
```

### Output Settings

```toml
[output]
# Default export format (json, markdown, csv)
default_format = "json"

# Include metadata in exports
include_metadata = true

# Date format for exports
date_format = "%Y-%m-%d"
```

### TUI Settings

```toml
[tui]
# Enable mouse support
mouse = true

# Refresh interval in seconds
refresh_interval = 300

# Theme (dark, light, auto)
theme = "dark"

# Show help panel
show_help = true
```

## Environment Variables

You can override configuration values using environment variables:

- `LBXD_CONFIG_FILE`: Path to custom configuration file
- `LBXD_CACHE_DIR`: Custom cache directory
- `TMDB_API_KEY`: TMDB API key (overrides default)
- `OMDB_API_KEY`: OMDB API key (overrides default) 
- `LBXD_LOG_LEVEL`: Logging level (error, warn, info, debug, trace)

### API Key Configuration

lbxd comes with default API keys for TMDB and OMDB services, so it works out of the box. However, you can use your own API keys if needed:

**For TMDB:**
```bash
export TMDB_API_KEY="your_tmdb_api_key_here"
```

**For OMDB:**
```bash
export OMDB_API_KEY="your_omdb_api_key_here"
```

**To make environment variables persistent, add them to your shell profile:**

For bash (`~/.bashrc` or `~/.bash_profile`):
```bash
# lbxd API configuration
export TMDB_API_KEY="your_tmdb_api_key_here"
export OMDB_API_KEY="your_omdb_api_key_here"
```

For zsh (`~/.zshrc`):
```bash
# lbxd API configuration  
export TMDB_API_KEY="your_tmdb_api_key_here"
export OMDB_API_KEY="your_omdb_api_key_here"
```

For fish (`~/.config/fish/config.fish`):
```fish
# lbxd API configuration
set -gx TMDB_API_KEY "your_tmdb_api_key_here"
set -gx OMDB_API_KEY "your_omdb_api_key_here"
```

After adding to your shell profile, restart your terminal or run `source ~/.bashrc` (or equivalent).

## Managing Configuration

### Initialize Configuration

Create a new configuration file with default values:

```bash
lbxd config init
```

### View Current Configuration

Display all current settings:

```bash
lbxd config show
```

### Update Settings

Set individual configuration values:

```bash
# Set page size
lbxd config set display.page_size 50

# Enable caching
lbxd config set cache.enabled true

# Set TMDB API key
lbxd config set api.tmdb_key "your_tmdb_api_key_here"

# Set OMDB API key  
lbxd config set api.omdb_key "your_omdb_api_key_here"
```

### Reset Configuration

Reset to default values:

```bash
lbxd config reset
```

## Example Configuration

Here's a complete example configuration file:

```toml
[display]
page_size = 25
colored = true
ascii_art = true
width = 0

[cache]
enabled = true
directory = ""
ttl = 48
max_size = 200

[api]
tmdb_key = "your_tmdb_api_key"
omdb_key = "your_omdb_api_key"
timeout = 45
max_concurrent = 3
rate_limit = 1

[output]
default_format = "json"
include_metadata = true
date_format = "%B %d, %Y"

[tui]
mouse = true
refresh_interval = 600
theme = "dark"
show_help = true
```

## Troubleshooting

### Configuration File Not Found

If the configuration file is missing or corrupted:

```bash
# Recreate with defaults
lbxd config init --force
```

### Permission Issues

If you encounter permission errors:

```bash
# Check file permissions
ls -la ~/.config/lbxd/

# Fix permissions if needed
chmod 644 ~/.config/lbxd/config.toml
```

### Invalid Configuration

To validate your configuration:

```bash
lbxd config validate
```
