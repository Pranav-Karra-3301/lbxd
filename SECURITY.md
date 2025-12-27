# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 3.x     | :white_check_mark: |
| < 3.0   | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability:

1. **Do not** open a public issue
2. Email the maintainer or open a private security advisory on GitHub
3. Include details about the vulnerability and steps to reproduce

You can expect a response within 48 hours. We'll work with you to understand and address the issue.

## Security Measures

- Dependencies are scanned weekly via `cargo audit`
- CI includes security checks on all PRs
- API keys are never logged or cached
