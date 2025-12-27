# Debug Issue

Help diagnose and fix a bug or unexpected behavior.

## Information Gathering

Ask the user for:
1. What behavior is expected?
2. What is actually happening?
3. Steps to reproduce
4. Any error messages
5. Relevant command/input

## Debug Process

1. **Reproduce**
   - Run the command locally
   - Capture exact error output

2. **Locate**
   - Find relevant code path
   - Identify the failing component

3. **Analyze**
   - Read the code carefully
   - Check error handling
   - Look for edge cases

4. **Fix**
   - Propose minimal fix
   - Explain why it works
   - Consider side effects

5. **Verify**
   - Run tests
   - Test the fix manually
   - Check for regressions

## Debug Tools

```bash
# Enable debug logging
RUST_LOG=debug cargo run -- [args]

# Trace level for verbose output
RUST_LOG=lbxd=trace cargo run -- [args]

# Run with backtrace
RUST_BACKTRACE=1 cargo run -- [args]
```

## Common Issues

### Network Errors
- Check internet connection
- Verify API availability
- Check for rate limiting

### Cache Problems
- Clear cache: `rm -rf ~/.cache/lbxd/`
- Check cache TTL

### Terminal Issues
- Check terminal size
- Verify Unicode support
- Check color support
