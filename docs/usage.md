# Usage

lbxd provides several commands to interact with Letterboxd data from your terminal.

## Basic Commands

### View Recent Activity

Display a user's recent movie activity:

```bash
lbxd recent <username>
```

Example:
```bash
lbxd recent johndoe
```

### Interactive TUI Mode

Launch the interactive terminal user interface to browse collections:

```bash
lbxd tui <username>
```

### Search User Activity

Search for specific movies in a user's activity:

```bash
lbxd search <username> <query>
```

Example:
```bash
lbxd search johndoe "pulp fiction"
```

### Movie Database Search

Search TMDB for movie information:

```bash
lbxd movie <title>
```

Example:
```bash
lbxd movie "The Godfather"
```

## Export Options

### Export to JSON

Export user data in JSON format:

```bash
lbxd export <username> --format json --output movies.json
```

### Export to Markdown

Export user data as a formatted Markdown file:

```bash
lbxd export <username> --format markdown --output movies.md
```

## Configuration

### Initialize Configuration

Set up initial configuration:

```bash
lbxd config init
```

### View Current Configuration

Display current settings:

```bash
lbxd config show
```

### Update Configuration

Modify specific configuration values:

```bash
lbxd config set <key> <value>
```

## Advanced Features

### Caching

lbxd automatically caches data for offline access. To clear the cache:

```bash
lbxd cache clear
```

### Batch Operations

Process multiple users or operations:

```bash
lbxd batch --file users.txt --operation recent
```

## Command Line Options

Common options available across commands:

- `--verbose, -v`: Enable verbose output
- `--quiet, -q`: Suppress non-essential output  
- `--config <file>`: Use custom configuration file
- `--no-cache`: Disable caching for this operation
- `--help, -h`: Display help information

## Examples

### Complete Workflow

```bash
# Initialize configuration
lbxd config init

# View recent activity
lbxd recent moviebuff123

# Launch interactive mode
lbxd tui moviebuff123

# Search for specific movies
lbxd search moviebuff123 "christopher nolan"

# Export data
lbxd export moviebuff123 --format json --output backup.json
```
