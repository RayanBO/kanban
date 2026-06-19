# kb installer — Windows
# Usage: powershell -ExecutionPolicy Bypass -File install.ps1

$ErrorActionPreference = "Stop"

$installDir = "$env:LOCALAPPDATA\Programs\kb"
$binary     = "target\release\kb.exe"

Write-Host "=== kb Installer ===" -ForegroundColor Cyan

# Build si binaire absent
if (-not (Test-Path $binary)) {
    Write-Host "Build release en cours..." -ForegroundColor Yellow
    cargo build --release
    if (-not $?) { Write-Host "Build echoue." -ForegroundColor Red; exit 1 }
}

# Creer dossier installation
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

# Copier binaire
Copy-Item $binary "$installDir\kb.exe" -Force
Write-Host "Installe dans : $installDir\kb.exe" -ForegroundColor Green

# Ajouter au PATH utilisateur si absent
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    Write-Host "PATH mis a jour." -ForegroundColor Green
} else {
    Write-Host "PATH deja configure." -ForegroundColor Gray
}

Write-Host ""
Write-Host "Installation terminee !" -ForegroundColor Cyan
Write-Host "Ouvre un nouveau terminal et lance : kb --version" -ForegroundColor White
