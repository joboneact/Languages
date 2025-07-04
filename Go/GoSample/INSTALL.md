# Go Installation and Setup Guide

## Problem
Go is not installed on your system. The error "go: The term 'go' is not recognized" indicates that the Go compiler is not available.

## Solution: Install Go

### Option 1: Download from Official Website
1. Go to https://golang.org/dl/
2. Download the Windows installer (.msi file)
3. Run the installer
4. Restart your terminal/PowerShell
5. Verify installation: `go version`

### Option 2: Using Chocolatey (if installed)
```powershell
choco install golang
```

### Option 3: Using Scoop (if installed)
```powershell
scoop install go
```

### Option 4: Using Winget
```powershell
winget install GoLang.Go
```

## After Installation
1. Open a new PowerShell/Command Prompt window
2. Navigate to your project directory:
   ```powershell
   cd "c:\Proj\Languages\Go"
   ```
3. Run the program:
   ```powershell
   go run main.go
   ```

## Alternative: Online Go Playground
If you can't install Go locally, you can run the code online:
1. Go to https://play.golang.org/
2. Copy and paste the main.go content
3. Click "Run"

Note: The online playground has limitations with goroutines and may not show all concurrent behavior properly.

## Expected Output
When you run the program, you should see:
- Go version and system information
- Person struct demonstration
- Error handling examples
- Channel communication
- Select statement multiplexing
- Context timeout handling
- Defer execution order
- Panic recovery
- Slice and map operations
- Pointer manipulation
- Worker pool with concurrent processing
- Interface demonstrations
- Variadic functions
- Closures

## Troubleshooting
If Go is installed but not recognized:
1. Check if Go is in your PATH environment variable
2. Restart your terminal
3. Try using the full path to Go executable

## Quick Test
After installation, test with:
```powershell
go version
go env GOPATH
go env GOROOT
```
