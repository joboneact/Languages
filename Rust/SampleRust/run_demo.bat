@echo off
REM Rust Features Demo Build and Run Script for Windows

echo 🦀 Rust Features Demo
echo =====================

REM Check if Rust is installed
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Rust is not installed. Please install Rust from https://rustup.rs/
    exit /b 1
)

cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Cargo is not installed. Please install Rust from https://rustup.rs/
    exit /b 1
)

echo ✅ Rust version:
rustc --version
echo ✅ Cargo version:
cargo --version
echo.

REM Build the project
echo 🔨 Building project...
cargo build
if %errorlevel% neq 0 (
    echo ❌ Build failed!
    exit /b 1
)
echo ✅ Build successful!
echo.

REM Run tests
echo 🧪 Running tests...
cargo test
if %errorlevel% neq 0 (
    echo ❌ Some tests failed!
) else (
    echo ✅ All tests passed!
)
echo.

REM Run the main program
echo 🚀 Running main program...
echo =========================
cargo run

echo.
echo 🎉 Demo completed!
echo.
echo 📚 Additional commands you can try:
echo    cargo run --release     # Run with optimizations
echo    cargo test              # Run tests only
echo    cargo doc --open        # Generate and open documentation
echo    cargo clippy            # Run linter
echo    cargo fmt               # Format code

pause
