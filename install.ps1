# lbxd Windows Installation Script
# PowerShell script to install lbxd and all dependencies on Windows

param(
    [Parameter(Mandatory=$false)]
    [string]$Version = "latest",
    
    [Parameter(Mandatory=$false)]
    [string]$InstallDir = "$env:LOCALAPPDATA\lbxd",
    
    [Parameter(Mandatory=$false)]
    [switch]$AddToPath = $true
)

# Colors for output
$Red = "`e[31m"
$Green = "`e[32m"
$Yellow = "`e[33m"
$Blue = "`e[34m"
$Magenta = "`e[35m"
$Cyan = "`e[36m"
$Reset = "`e[0m"

function Write-ColoredOutput {
    param([string]$Message, [string]$Color)
    Write-Host "${Color}${Message}${Reset}"
}

function Test-Command {
    param([string]$Command)
    try {
        Get-Command $Command -ErrorAction Stop | Out-Null
        return $true
    } catch {
        return $false
    }
}

function Install-Python {
    Write-ColoredOutput "🐍 Installing Python..." $Blue
    
    if (Test-Command "python") {
        $pythonVersion = & python --version 2>&1
        if ($pythonVersion -match "Python (\d+)\.(\d+)") {
            $majorVersion = [int]$matches[1]
            $minorVersion = [int]$matches[2]
            if ($majorVersion -ge 3 -and $minorVersion -ge 8) {
                Write-ColoredOutput "✅ Python $($matches[0]) is already installed" $Green
                return
            }
        }
    }
    
    # Try to install Python via winget
    if (Test-Command "winget") {
        Write-ColoredOutput "Installing Python via winget..." $Yellow
        winget install Python.Python.3.12 --silent --accept-package-agreements --accept-source-agreements
    } elseif (Test-Command "choco") {
        Write-ColoredOutput "Installing Python via Chocolatey..." $Yellow
        choco install python3 -y
    } else {
        Write-ColoredOutput "Please install Python 3.8+ manually from https://python.org/downloads/" $Red
        Write-ColoredOutput "After installing Python, run this script again." $Yellow
        exit 1
    }
    
    # Refresh PATH
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
}

function Install-PythonDependencies {
    Write-ColoredOutput "📦 Installing Python dependencies..." $Blue
    
    # Install letterboxdpy
    try {
        & python -m pip install --user letterboxdpy
        Write-ColoredOutput "✅ letterboxdpy installed successfully" $Green
    } catch {
        Write-ColoredOutput "❌ Failed to install letterboxdpy: $($_.Exception.Message)" $Red
        exit 1
    }
}

function Install-Viu {
    Write-ColoredOutput "🖼️  Installing viu for enhanced image display..." $Blue
    
    if (Test-Command "viu") {
        Write-ColoredOutput "✅ viu is already installed" $Green
        return
    }
    
    # Try different methods to install viu
    if (Test-Command "cargo") {
        Write-ColoredOutput "Installing viu via Cargo..." $Yellow
        cargo install viu
    } elseif (Test-Command "scoop") {
        Write-ColoredOutput "Installing viu via Scoop..." $Yellow
        scoop install viu
    } elseif (Test-Command "choco") {
        Write-ColoredOutput "Installing viu via Chocolatey..." $Yellow
        choco install viu -y
    } else {
        Write-ColoredOutput "⚠️  Could not install viu. lbxd will use ASCII art mode." $Yellow
    }
}

function Get-LatestRelease {
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/Pranav-Karra-3301/lbxd/releases/latest"
        return $response.tag_name
    } catch {
        Write-ColoredOutput "❌ Failed to fetch latest release information" $Red
        return $null
    }
}

function Download-LbxdBinary {
    Write-ColoredOutput "📥 Downloading lbxd binary..." $Blue
    
    if ($Version -eq "latest") {
        $Version = Get-LatestRelease
        if (-not $Version) {
            Write-ColoredOutput "❌ Could not determine latest version" $Red
            exit 1
        }
    }
    
    # Remove 'v' prefix if present
    $CleanVersion = $Version -replace '^v', ''
    
    # Determine architecture
    $arch = if ([System.Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
    $downloadUrl = "https://github.com/Pranav-Karra-3301/lbxd/releases/download/v${CleanVersion}/lbxd-windows-${arch}.exe.zip"
    
    $tempFile = Join-Path $env:TEMP "lbxd-windows-${arch}.exe.zip"
    
    try {
        Write-ColoredOutput "Downloading from: $downloadUrl" $Yellow
        Invoke-WebRequest -Uri $downloadUrl -OutFile $tempFile -UseBasicParsing
        Write-ColoredOutput "✅ Download completed" $Green
    } catch {
        Write-ColoredOutput "❌ Failed to download lbxd binary: $($_.Exception.Message)" $Red
        exit 1
    }
    
    return $tempFile
}

function Install-LbxdBinary {
    param([string]$ZipFile)
    
    Write-ColoredOutput "📦 Installing lbxd binary..." $Blue
    
    # Create installation directory
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }
    
    # Extract the binary
    try {
        Expand-Archive -Path $ZipFile -DestinationPath $InstallDir -Force
        $binaryPath = Join-Path $InstallDir "lbxd.exe"
        
        if (Test-Path $binaryPath) {
            Write-ColoredOutput "✅ lbxd installed to: $binaryPath" $Green
        } else {
            Write-ColoredOutput "❌ Binary not found after extraction" $Red
            exit 1
        }
    } catch {
        Write-ColoredOutput "❌ Failed to extract binary: $($_.Exception.Message)" $Red
        exit 1
    }
    
    # Clean up
    Remove-Item $ZipFile -Force -ErrorAction SilentlyContinue
    
    return $binaryPath
}

function Add-ToPath {
    param([string]$Directory)
    
    if (-not $AddToPath) {
        return
    }
    
    Write-ColoredOutput "🔧 Adding lbxd to PATH..." $Blue
    
    # Get current user PATH
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    
    # Check if already in PATH
    if ($currentPath -split ';' -contains $Directory) {
        Write-ColoredOutput "✅ $Directory is already in PATH" $Green
        return
    }
    
    # Add to PATH
    $newPath = if ($currentPath) { "$currentPath;$Directory" } else { $Directory }
    
    try {
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        $env:Path += ";$Directory"
        Write-ColoredOutput "✅ Added $Directory to PATH" $Green
        Write-ColoredOutput "ℹ️  You may need to restart your terminal for PATH changes to take effect" $Yellow
    } catch {
        Write-ColoredOutput "❌ Failed to add to PATH: $($_.Exception.Message)" $Red
        Write-ColoredOutput "You can manually add $Directory to your PATH" $Yellow
    }
}

function Test-Installation {
    Write-ColoredOutput "🧪 Testing installation..." $Blue
    
    try {
        $version = & "$InstallDir\lbxd.exe" --version 2>&1
        Write-ColoredOutput "✅ lbxd is working: $version" $Green
    } catch {
        Write-ColoredOutput "❌ lbxd test failed: $($_.Exception.Message)" $Red
        return $false
    }
    
    # Test Python dependency
    try {
        & python -c "import letterboxdpy; print('letterboxdpy is available')" 2>&1 | Out-Null
        Write-ColoredOutput "✅ letterboxdpy is working" $Green
    } catch {
        Write-ColoredOutput "⚠️  letterboxdpy test failed" $Yellow
    }
    
    # Test viu (optional)
    if (Test-Command "viu") {
        Write-ColoredOutput "✅ viu is available for enhanced image display" $Green
    } else {
        Write-ColoredOutput "ℹ️  viu not available - will use ASCII art mode" $Cyan
    }
    
    return $true
}

# Main installation process
Write-ColoredOutput "🚀 lbxd Windows Installer" $Magenta
Write-ColoredOutput "=========================" $Magenta

# Check for required tools
if (-not (Test-Command "powershell")) {
    Write-ColoredOutput "❌ PowerShell is required" $Red
    exit 1
}

# Install dependencies
Install-Python
Install-PythonDependencies
Install-Viu

# Download and install lbxd
$zipFile = Download-LbxdBinary
$binaryPath = Install-LbxdBinary -ZipFile $zipFile

# Add to PATH
Add-ToPath -Directory $InstallDir

# Test installation
if (Test-Installation) {
    Write-ColoredOutput "" ""
    Write-ColoredOutput "🎉 Installation completed successfully!" $Green
    Write-ColoredOutput "" ""
    Write-ColoredOutput "Usage:" $Cyan
    Write-ColoredOutput "  lbxd --help                    # Show help" $Cyan
    Write-ColoredOutput "  lbxd recent username           # Show recent films" $Cyan
    Write-ColoredOutput "  lbxd profile username          # Show user profile" $Cyan
    Write-ColoredOutput "" ""
    
    if ($AddToPath) {
        Write-ColoredOutput "lbxd has been added to your PATH and should be available in new terminal sessions." $Yellow
    } else {
        Write-ColoredOutput "Binary installed to: $binaryPath" $Yellow
        Write-ColoredOutput "Add $InstallDir to your PATH to use 'lbxd' command globally." $Yellow
    }
} else {
    Write-ColoredOutput "❌ Installation verification failed" $Red
    exit 1
}
