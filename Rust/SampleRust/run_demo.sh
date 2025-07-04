#!/bin/bash

# Rust Features Demo Build and Run Script

echo "🦀 Rust Features Demo"
echo "====================="

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust version: $(rustc --version)"
echo "✅ Cargo version: $(cargo --version)"
echo ""

# Build the project
echo "🔨 Building project..."
if cargo build; then
    echo "✅ Build successful!"
else
    echo "❌ Build failed!"
    exit 1
fi

echo ""

# Run tests
echo "🧪 Running tests..."
if cargo test; then
    echo "✅ All tests passed!"
else
    echo "❌ Some tests failed!"
fi

echo ""

# Run the main program
echo "🚀 Running main program..."
echo "========================="
cargo run

echo ""
echo "🎉 Demo completed!"
echo ""
echo "📚 Additional commands you can try:"
echo "   cargo run --release     # Run with optimizations"
echo "   cargo test              # Run tests only"
echo "   cargo doc --open        # Generate and open documentation"
echo "   cargo clippy            # Run linter"
echo "   cargo fmt               # Format code"
