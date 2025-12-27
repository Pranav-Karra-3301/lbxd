# Lint and Format

Run code quality checks on lbxd.

## Tasks

1. Run `cargo fmt -- --check` to verify formatting
2. Run `cargo clippy -- -D warnings` for linting
3. Report all issues found

## Output Format

For each issue:
- File and line number
- Issue description
- Suggested fix (if clippy provides one)

## Fix Mode

If the user adds "fix" after the command:
1. Run `cargo fmt` to auto-format
2. Apply clippy suggestions where safe
3. Report what was changed
