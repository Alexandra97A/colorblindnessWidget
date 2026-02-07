# Release Packaging Script
# This script creates a ready-to-distribute ZIP file with all necessary components

param(
    [string]$Version = "0.1.0"
)

$ErrorActionPreference = "Stop"

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host " Color Blindness Widget - Release Packager" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""

$ReleaseName = "colorblind-widget-v$Version-windows"
$ReleaseDir = ".\$ReleaseName"
$ZipFile = "$ReleaseName.zip"

# Check if executable exists
$ExePath = ".\target\release\colorblind-widget.exe"
if (-not (Test-Path $ExePath)) {
    Write-Host "ERROR: Executable not found at $ExePath" -ForegroundColor Red
    Write-Host "Please build the release version first:" -ForegroundColor Yellow
    Write-Host "  cargo build --release" -ForegroundColor Yellow
    exit 1
}

Write-Host "[1/6] Checking files..." -ForegroundColor Green
$RequiredFiles = @(
    "install.bat",
    "uninstall.bat",
    "RELEASE_README.txt"
)

foreach ($file in $RequiredFiles) {
    if (-not (Test-Path $file)) {
        Write-Host "  WARNING: $file not found" -ForegroundColor Yellow
    } else {
        Write-Host "  Found: $file" -ForegroundColor Gray
    }
}

# Create release directory
Write-Host ""
Write-Host "[2/6] Creating release directory..." -ForegroundColor Green
if (Test-Path $ReleaseDir) {
    Remove-Item -Recurse -Force $ReleaseDir
}
New-Item -ItemType Directory -Path $ReleaseDir | Out-Null
Write-Host "  Created: $ReleaseDir" -ForegroundColor Gray

# Copy executable
Write-Host ""
Write-Host "[3/6] Copying executable..." -ForegroundColor Green
Copy-Item $ExePath -Destination "$ReleaseDir\colorblind-widget.exe"
$ExeSize = (Get-Item "$ReleaseDir\colorblind-widget.exe").Length / 1MB
Write-Host "  Size: $([math]::Round($ExeSize, 2)) MB" -ForegroundColor Gray

# Copy installer scripts
Write-Host ""
Write-Host "[4/6] Copying installer scripts..." -ForegroundColor Green
Copy-Item "install.bat" -Destination "$ReleaseDir\install.bat"
Copy-Item "uninstall.bat" -Destination "$ReleaseDir\uninstall.bat"
Write-Host "  Copied: install.bat, uninstall.bat" -ForegroundColor Gray

# Copy README
Write-Host ""
Write-Host "[5/6] Copying README..." -ForegroundColor Green
Copy-Item "RELEASE_README.txt" -Destination "$ReleaseDir\README.txt"
Write-Host "  Copied: README.txt" -ForegroundColor Gray

# Copy icon if it exists
if (Test-Path "icon.ico") {
    Copy-Item "icon.ico" -Destination "$ReleaseDir\icon.ico"
    Write-Host "  Copied: icon.ico" -ForegroundColor Gray
}

# Create ZIP file
Write-Host ""
Write-Host "[6/6] Creating ZIP file..." -ForegroundColor Green
if (Test-Path $ZipFile) {
    Remove-Item $ZipFile
}
Compress-Archive -Path $ReleaseDir -DestinationPath $ZipFile
$ZipSize = (Get-Item $ZipFile).Length / 1MB
Write-Host "  Created: $ZipFile" -ForegroundColor Gray
Write-Host "  Size: $([math]::Round($ZipSize, 2)) MB" -ForegroundColor Gray

# Summary
Write-Host ""
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host " Release Package Created!" -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Release ZIP: $ZipFile" -ForegroundColor Green
Write-Host "Contents:" -ForegroundColor White
Get-ChildItem $ReleaseDir | ForEach-Object {
    $size = $_.Length / 1KB
    Write-Host "  - $($_.Name) ($([math]::Round($size, 1)) KB)" -ForegroundColor Gray
}

Write-Host ""
Write-Host "Next Steps:" -ForegroundColor Yellow
Write-Host "  1. Test the installation:" -ForegroundColor White
Write-Host "     - Extract $ZipFile to a test folder" -ForegroundColor Gray
Write-Host "     - Run install.bat" -ForegroundColor Gray
Write-Host "     - Verify the application works" -ForegroundColor Gray
Write-Host ""
Write-Host "  2. Upload to GitHub Releases:" -ForegroundColor White
Write-Host "     - Go to your repository on GitHub" -ForegroundColor Gray
Write-Host "     - Click 'Releases' > 'Create a new release'" -ForegroundColor Gray
Write-Host "     - Tag: v$Version" -ForegroundColor Gray
Write-Host "     - Upload $ZipFile" -ForegroundColor Gray
Write-Host ""
Write-Host "  3. Share the download link!" -ForegroundColor White
Write-Host ""

# Offer to test
$test = Read-Host "Would you like to test the installer now? (Y/N)"
if ($test -eq "Y" -or $test -eq "y") {
    Write-Host ""
    Write-Host "Extracting to test folder..." -ForegroundColor Yellow
    $TestDir = ".\test-installation"
    if (Test-Path $TestDir) {
        Remove-Item -Recurse -Force $TestDir
    }
    Expand-Archive -Path $ZipFile -DestinationPath $TestDir
    Set-Location "$TestDir\$ReleaseName"
    Write-Host "Opening test folder..." -ForegroundColor Yellow
    Start-Process explorer.exe -ArgumentList (Get-Location).Path
    Write-Host "You can now run install.bat from the opened folder" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "Release package is ready! Upload to GitHub when ready." -ForegroundColor Green
}

Write-Host ""
Write-Host "Done! 🚀" -ForegroundColor Cyan
