$ErrorActionPreference = 'Stop'

$packageName = 'lbxd'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64 = 'https://github.com/Pranav-Karra-3301/lbxd/releases/download/v2.2.0/lbxd-windows-x86_64.exe.zip'

# Package parameters
$packageArgs = @{
  packageName    = $packageName
  unzipLocation  = $toolsDir
  url64bit       = $url64
  checksum64     = 'CHECKSUM_PLACEHOLDER'
  checksumType64 = 'sha256'
}

Write-Host "Installing lbxd..." -ForegroundColor Green

# Download and extract the binary
Install-ChocolateyZipPackage @packageArgs

# Install Python 3 if not present
if (-not (Get-Command python -ErrorAction SilentlyContinue) -and -not (Get-Command python3 -ErrorAction SilentlyContinue)) {
    Write-Host "Python 3 not found. Installing..." -ForegroundColor Yellow
    choco install python3 -y
}

# Install Python dependencies
Write-Host "Installing Python dependencies..." -ForegroundColor Green
try {
    if (Get-Command python -ErrorAction SilentlyContinue) {
        python -m pip install --user letterboxdpy
    } elseif (Get-Command python3 -ErrorAction SilentlyContinue) {
        python3 -m pip install --user letterboxdpy
    }
    Write-Host "Python dependencies installed successfully" -ForegroundColor Green
} catch {
    Write-Warning "Failed to install Python dependencies. You may need to install letterboxdpy manually: pip install letterboxdpy"
}

# Install viu for enhanced image display (optional)
Write-Host "Installing viu for enhanced terminal image display..." -ForegroundColor Green
try {
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        cargo install viu
        Write-Host "viu installed successfully" -ForegroundColor Green
    } elseif (Get-Command scoop -ErrorAction SilentlyContinue) {
        scoop install viu
        Write-Host "viu installed via scoop" -ForegroundColor Green
    } else {
        Write-Warning "Could not install viu. Install Rust/Cargo or Scoop to enable enhanced image display."
        Write-Warning "lbxd will work with ASCII art mode by default."
    }
} catch {
    Write-Warning "Failed to install viu. lbxd will use ASCII art mode by default."
}

Write-Host ""
Write-Host "🎬 lbxd installation complete!" -ForegroundColor Green
Write-Host "Try: lbxd --help" -ForegroundColor Cyan
Write-Host "Quick start: lbxd recent username" -ForegroundColor Cyan