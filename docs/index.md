# lbxd Documentation

Welcome to the lbxd documentation! This guide covers installation, usage, configuration, and API reference for the Letterboxd command-line tool.

```{toctree}
:maxdepth: 2
:caption: "Contents:"

installation
usage
configuration
api-keys
migration
api/index
```

## Overview

**lbxd** is a Rust-based command-line tool that allows you to:

- 🎭 Browse movie collections with an interactive TUI
- 📽️ View user's recent activity, ratings, and reviews  
- 🔍 Search for specific titles in activity history
- 🎬 Access detailed movie information from TMDB
- 📤 Export data to JSON or Markdown formats
- ⚙️ Configure persistent settings and preferences
- 💾 Cache data for offline access
- 🎨 Enjoy beautiful terminal displays with ASCII art

## Quick Start

Install lbxd using Homebrew:

```bash
brew tap pranav-karra-3301/lbxd
brew install lbxd
```

*Formula maintained at: [homebrew-lbxd](https://github.com/Pranav-Karra-3301/homebrew-lbxd)*

Or build from source:

```bash
git clone https://github.com/Pranav-Karra-3301/lbxd.git
cd lbxd
cargo build --release
```

## Indices and tables

* {ref}`genindex`
* {ref}`modindex`
* {ref}`search`
