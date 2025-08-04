# API Key Configuration

lbxd integrates with The Movie Database (TMDB) and Open Movie Database (OMDB) to provide rich movie information. The application comes with built-in default API keys that work out of the box, so you can start using lbxd immediately without any setup.

## No Setup Required

lbxd includes default API keys for both services:
- **TMDB**: For movie search, posters, and metadata
- **OMDB**: For additional ratings and detailed information

You can use lbxd immediately after installation without configuring any API keys.

## Using Your Own API Keys (Optional)

If you prefer to use your own API keys or encounter rate limiting, you can easily override the defaults.

### Method 1: Environment Variables (Recommended)

Set environment variables to override the default API keys:

```bash
# Set your TMDB API key
export TMDB_API_KEY="your_tmdb_api_key_here"

# Set your OMDB API key  
export OMDB_API_KEY="your_omdb_api_key_here"
```

### Method 2: Shell Profile (Persistent)

To make the API keys persistent across terminal sessions, add them to your shell profile:

**For Bash:**
```bash
# Add to ~/.bashrc or ~/.bash_profile
export TMDB_API_KEY="your_tmdb_api_key_here"
export OMDB_API_KEY="your_omdb_api_key_here"
```

**For Zsh:**
```bash
# Add to ~/.zshrc
export TMDB_API_KEY="your_tmdb_api_key_here"  
export OMDB_API_KEY="your_omdb_api_key_here"
```

**For Fish:**
```fish
# Add to ~/.config/fish/config.fish
set -gx TMDB_API_KEY "your_tmdb_api_key_here"
set -gx OMDB_API_KEY "your_omdb_api_key_here"
```

After editing your shell profile, restart your terminal or run:
```bash
source ~/.bashrc  # or ~/.zshrc, etc.
```

### Method 3: Per-Session Override

For temporary use in a single terminal session:

```bash
# Run lbxd with custom API keys for this session only
TMDB_API_KEY="your_key" OMDB_API_KEY="your_key" lbxd recent username
```

## Getting Your Own API Keys

### TMDB API Key

1. Visit [The Movie Database](https://www.themoviedb.org/)
2. Create a free account
3. Go to Settings → API
4. Request an API key
5. Use the "API Key (v3 auth)" value

### OMDB API Key

1. Visit [OMDB API](http://www.omdbapi.com/apikey.aspx)
2. Choose a plan (free tier available)
3. Provide your email address
4. Check your email for the API key

## Verifying Your Configuration

To check if your custom API keys are being used:

```bash
# The application will use your custom keys if set
lbxd movie "Inception"
```

If you encounter API errors, check:
1. Your API keys are valid
2. You haven't exceeded rate limits
3. Your keys have the correct permissions

## Default vs Custom Keys

| Aspect | Default Keys | Your Keys |
|--------|-------------|-----------|
| Setup Required | ✅ None | ❌ Account creation needed |
| Rate Limits | Shared across all users | Personal limits |
| Cost | ✅ Free | May have costs depending on usage |
| Reliability | May hit limits during peak usage | More reliable for heavy usage |

## Troubleshooting

### Common Issues

**API Rate Limit Exceeded:**
- Switch to your own API keys for higher limits
- Wait before making more requests
- Consider upgrading to paid API tiers

**Invalid API Key Error:**
- Verify your API key is correct
- Check that environment variables are properly set
- Ensure your API key has the required permissions

**Network Errors:**
- Check your internet connection
- Verify API endpoints are accessible
- Try again after a brief wait

### Debugging

To debug API key issues, you can check environment variables:

```bash
# Check if your custom API keys are set
echo $TMDB_API_KEY
echo $OMDB_API_KEY
```

## Security Notes

- Never commit API keys to version control
- Use environment variables or shell profiles instead of hardcoding
- Keep your API keys private and secure
- Regenerate keys if they're accidentally exposed

## Migration from Xcode Keychain

If you were previously using Xcode keychain storage (not applicable to this project, but for reference):

1. Remove any keychain entries for lbxd API keys
2. Set environment variables as described above
3. lbxd will automatically use the environment variables

The application no longer requires Xcode or keychain access and works on all platforms uniformly.
