# Migration Guide: From Xcode Keychain to Environment Variables

## Overview

This migration guide helps you transition from any potential Xcode keychain-based API key storage to the new environment variable approach. 

**Important Note**: The lbxd project never actually used Xcode keychain storage, but this guide is provided for completeness and to clarify the current approach.

## What Changed

### Before (Hypothetical Xcode Approach)
- ❌ Required Xcode installation
- ❌ Complex keychain setup
- ❌ macOS-only functionality
- ❌ Security prompts and permissions

### After (Current Environment Variable Approach)
- ✅ **No setup required** - works immediately with defaults
- ✅ Cross-platform compatibility (Linux, macOS, Windows)
- ✅ Simple environment variable overrides
- ✅ No Xcode dependency
- ✅ No security prompts

## Migration Steps

Since lbxd never used Xcode keychain, there's no actual migration needed. However, if you want to use custom API keys:

### 1. Remove Any Xcode/Keychain References

If you have any shell scripts or configurations that reference keychain access, they can be removed:

```bash
# Remove these types of commands (if they exist)
# security find-generic-password -s "lbxd-tmdb" -w
# security find-generic-password -s "lbxd-omdb" -w
```

### 2. Set Environment Variables (Optional)

If you want to use your own API keys:

```bash
# Add to your shell profile (~/.bashrc, ~/.zshrc, etc.)
export TMDB_API_KEY="your_tmdb_api_key_here"
export OMDB_API_KEY="your_omdb_api_key_here"
```

### 3. Test the Configuration

```bash
# Test with default keys (no setup needed)
lbxd movie "Inception"

# Test with custom keys (if you set them)
lbxd recent username
```

## Key Benefits of the New Approach

### 1. **Zero Setup Required**
```bash
# Install and use immediately
brew install lbxd
lbxd recent username  # Works right away!
```

### 2. **Cross-Platform Compatibility**
- **Linux**: Full support with defaults
- **macOS**: No Xcode requirement
- **Windows**: Native support
- **Docker/CI**: Works in containers

### 3. **Flexible Configuration**
```bash
# Use defaults (recommended)
lbxd movie "Dune"

# Override per-session
TMDB_API_KEY="your_key" lbxd movie "Dune"

# Set permanently
export TMDB_API_KEY="your_key"
```

### 4. **Developer-Friendly**
```bash
# Easy CI/CD integration
env:
  TMDB_API_KEY: ${{ secrets.TMDB_KEY }}
  OMDB_API_KEY: ${{ secrets.OMDB_KEY }}

# Docker compatibility
docker run -e TMDB_API_KEY="$TMDB_KEY" lbxd
```

## FAQ

### Q: Do I need to configure API keys?
**A: No!** lbxd works immediately with built-in default keys.

### Q: Should I get my own API keys?
**A: Only if you encounter rate limits** or want dedicated quota.

### Q: How do I know if my custom keys are being used?
**A: Check environment variables:**
```bash
echo $TMDB_API_KEY
echo $OMDB_API_KEY
```

### Q: What if I want to remove custom keys?
**A: Simply unset the environment variables:**
```bash
unset TMDB_API_KEY
unset OMDB_API_KEY
```

### Q: Is this more secure than keychain?
**A: It's simpler and more transparent.** Environment variables are:
- Visible in your shell configuration
- Easy to manage and rotate
- Compatible with all deployment scenarios
- No system-level permissions required

## Troubleshooting

### Issue: "Command not found" 
**Solution**: lbxd is properly installed and in your PATH.

### Issue: API rate limits
**Solution**: Use your own API keys for higher limits.

### Issue: Environment variables not working
**Solution**: Check that they're properly exported:
```bash
# Verify they're set
env | grep -E "(TMDB|OMDB)_API_KEY"

# Re-source your shell profile
source ~/.bashrc  # or ~/.zshrc
```

## Summary

The new approach is:
- **Simpler**: No complex setup
- **More reliable**: No system dependencies  
- **More portable**: Works everywhere
- **More maintainable**: Clear configuration path

You can start using lbxd immediately without any configuration, and optionally customize it with environment variables when needed.
