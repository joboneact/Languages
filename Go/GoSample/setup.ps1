Write-Host "=== Go Installation and Demo Runner ===" -ForegroundColor Green
Write-Host ""

# Check if Go is installed
$goInstalled = $false
try {
    $goVersion = go version 2>$null
    if ($goVersion) {
        Write-Host "Go is already installed: $goVersion" -ForegroundColor Green
        $goInstalled = $true
    }
}
catch {
    Write-Host "Go is not installed" -ForegroundColor Red
}

if ($goInstalled) {
    Write-Host "Running Go demo..." -ForegroundColor Yellow
    go run main.go
} else {
    Write-Host "Installing Go using winget..." -ForegroundColor Yellow
    try {
        winget install GoLang.Go
        Write-Host "Go installation completed!" -ForegroundColor Green
        Write-Host "Please restart your terminal and run 'go run main.go'" -ForegroundColor Yellow
    }
    catch {
        Write-Host "Failed to install Go automatically" -ForegroundColor Red
        Write-Host "Please install Go manually from https://golang.org/dl/" -ForegroundColor Yellow
    }
}

Read-Host "Press Enter to exit"
