$ErrorActionPreference = 'Stop'

$packageName = 'lbxd'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64 = 'https://github.com/Pranav-Karra-3301/lbxd/releases/download/v3.0.0/lbxd-windows-x86_64.exe.zip'

# IMPORTANT: SHA256_PLACEHOLDER must be replaced with actual hash before publishing.
# Calculate with: Get-FileHash -Algorithm SHA256 lbxd-windows-x86_64.exe.zip
# Package parameters
$packageArgs = @{
  packageName    = $packageName
  unzipLocation  = $toolsDir
  url64bit       = $url64
  checksum64     = 'SHA256_PLACEHOLDER'  # TODO: Replace with actual SHA256 hash
  checksumType64 = 'sha256'
}

Write-Host "Installing lbxd v3.0.0 (pure Rust - no Python required!)..." -ForegroundColor Green

# Download and extract the binary
Install-ChocolateyZipPackage @packageArgs

# Optionally install viu for enhanced terminal image display
Write-Host ""
Write-Host "viu is recommended for terminal image display." -ForegroundColor Yellow
Write-Host "Install with: cargo install viu" -ForegroundColor Yellow

Write-Host ""
Write-Host "lbxd v3.0.0 installation complete!" -ForegroundColor Green
Write-Host "Try: lbxd --help" -ForegroundColor Cyan
Write-Host "Quick start: lbxd recent username" -ForegroundColor Cyan
