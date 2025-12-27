# Releasing

## Version Bump

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with release notes
3. Commit: `git commit -am "X.Y.Z"`

## Create Release

```bash
# Tag the release
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin vX.Y.Z
```

The GitHub Actions workflow automatically:
- Builds binaries for all platforms (Linux x86/ARM64, macOS Intel/Apple Silicon, Windows)
- Generates SHA256 checksums
- Creates a GitHub release with all assets

## Post-Release

- Verify binaries on the [releases page](https://github.com/Pranav-Karra-3301/lbxd/releases)
- Update Homebrew formula if needed ([homebrew-lbxd](https://github.com/Pranav-Karra-3301/homebrew-lbxd))

## Versioning

We use [Semantic Versioning](https://semver.org/):
- **Major**: Breaking changes
- **Minor**: New features (backwards compatible)
- **Patch**: Bug fixes
