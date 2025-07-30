$ErrorActionPreference = 'Stop'

$packageName = 'lbxd'
$url64 = 'https://github.com/Pranav-Karra-3301/lbxd/releases/download/v0.1.0/lbxd-windows-x86_64.exe.zip'

$packageArgs = @{
  packageName    = $packageName
  unzipLocation  = $toolsDir
  url64bit       = $url64
  checksum64     = 'YOUR_CHECKSUM_HERE'
  checksumType64 = 'sha256'
}

Install-ChocolateyZipPackage @packageArgs