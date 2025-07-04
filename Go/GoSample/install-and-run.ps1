# PowerShell script to install Go and run the demo

Write-Host "=== Go Installation and Demo Runner ===" -ForegroundColor Green
Write-Host ""

# Check if Go is already installed
try {
    $goVersion = & go version 2>$null
    if ($goVersion) {
        Write-Host "✅ Go is already installed: $goVersion" -ForegroundColor Green
        Write-Host ""
        Write-Host "Running the Go features demo..." -ForegroundColor Yellow
        & go run main.go
        exit 0
    }
}
catch {
    Write-Host "❌ Go is not installed or not in PATH" -ForegroundColor Red
}

Write-Host ""
Write-Host "🔧 Installing Go using winget..." -ForegroundColor Yellow

# Try to install Go using winget
try {
    & winget install GoLang.Go
    Write-Host "✅ Go installation completed!" -ForegroundColor Green
    Write-Host ""
    Write-Host "⚠️  Please restart your terminal/PowerShell and run this script again." -ForegroundColor Yellow
    Write-Host "   Or manually run: go run main.go" -ForegroundColor Yellow
}
catch {
    Write-Host "❌ Failed to install Go using winget" -ForegroundColor Red
    Write-Host ""
    Write-Host "📋 Manual installation options:" -ForegroundColor Cyan
    Write-Host "1. Download from: https://golang.org/dl/" -ForegroundColor White
    Write-Host "2. Use Chocolatey: choco install golang" -ForegroundColor White
    Write-Host "3. Use Scoop: scoop install go" -ForegroundColor White
    Write-Host ""
    Write-Host "After installation, restart terminal and run: go run main.go" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Press any key to exit..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
