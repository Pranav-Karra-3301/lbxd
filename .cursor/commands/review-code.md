# Code Review

Perform a thorough code review of recent changes or specified files.

## Review Checklist

### Safety
- [ ] No `unwrap()` or `expect()` in production paths
- [ ] No hardcoded secrets or API keys
- [ ] Proper error handling with `?` or match
- [ ] Input validation present
- [ ] No `unsafe` blocks (or justified if present)

### Code Quality
- [ ] Follows Rust idioms
- [ ] Functions are single-purpose
- [ ] No unnecessary allocations
- [ ] Async/await used correctly
- [ ] No blocking calls in async context

### Style
- [ ] Naming follows conventions
- [ ] Doc comments on public API
- [ ] No dead code or unused imports
- [ ] Formatting matches cargo fmt

### Testing
- [ ] New code has tests
- [ ] Edge cases covered
- [ ] Error cases tested

## Output Format

For each issue found:
1. **Location**: file:line
2. **Severity**: Error / Warning / Suggestion
3. **Issue**: Description
4. **Fix**: Recommended solution
