# Prepare Release

Prepare lbxd for a new version release.

## Required Information

- New version number (X.Y.Z)
- Type: major / minor / patch
- Summary of changes

## Pre-Release Checklist

1. **Run full test suite**
   ```bash
   cargo test --all
   ```

2. **Run linting**
   ```bash
   cargo fmt -- --check
   cargo clippy -- -D warnings
   ```

3. **Build release binary**
   ```bash
   cargo build --release
   ```

## Release Steps

1. **Update Cargo.toml**
   - Change `version = "X.Y.Z"`

2. **Update CHANGELOG.md**
   ```markdown
   ## [X.Y.Z] - YYYY-MM-DD

   ### Added
   - ...

   ### Changed
   - ...

   ### Fixed
   - ...
   ```

3. **Create commit**
   ```bash
   git add Cargo.toml CHANGELOG.md
   git commit -m "chore: bump version to X.Y.Z"
   ```

4. **Create tag**
   ```bash
   git tag vX.Y.Z
   ```

5. **Push**
   ```bash
   git push origin main
   git push origin vX.Y.Z
   ```

## Post-Release

CI/CD will automatically:
- Build binaries for all platforms
- Create GitHub release
- Upload artifacts

## Do NOT

- Skip tests before release
- Release with clippy warnings
- Forget to update CHANGELOG
- Push incomplete releases
