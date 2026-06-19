# kb installer — Windows
# Build + auto-install en une commande

$ErrorActionPreference = "Stop"

$binary = "target\release\kb.exe"

Write-Host "=== kb Installer ===" -ForegroundColor Cyan

if (-not (Test-Path $binary)) {
    Write-Host "Build release en cours..." -ForegroundColor Yellow
    cargo build --release
    if (-not $?) { Write-Host "Build échoué." -ForegroundColor Red; exit 1 }
}

& $binary install
