# Rust Features Comprehensive Demo

This project demonstrates Rust's top features including concurrency, functional programming, pattern matching, and more. It also includes examples of common compile errors and how to fix them.

## 🚀 Quick Start

### Prerequisites
- Rust installed (https://rustup.rs/)
- Cargo package manager (comes with Rust)

### Running the Demo

```bash
# Navigate to the project directory
cd rust-features-demo

# Run the main demonstration
cargo run

# Run tests
cargo test

# Run with release optimizations
cargo run --release
```

## 📚 What's Included

### 1. **Algebraic Data Types & Pattern Matching**
- Custom enums with associated data
- Advanced pattern matching with guards
- Destructuring tuples and structs
- Option and Result types

### 2. **Generics & Type Inference**
- Generic structs and functions
- Trait bounds and where clauses
- Type inference examples
- Associated types

### 3. **Functional Programming**
- Higher-order functions
- Closures and function pointers
- Iterator patterns
- Immutable data structures

### 4. **Concurrency**
- Thread-based concurrency with Arc<Mutex<T>>
- Async/await programming
- Parallel processing with Rayon
- Channel communication

### 5. **Error Handling**
- Result type for recoverable errors
- Option type for nullable values
- Custom error types
- Error propagation with `?` operator

### 6. **Memory Safety**
- Ownership and borrowing
- Lifetimes
- Smart pointers (Arc, Rc, Box)
- Zero-cost abstractions

### 7. **LLM Integration**
- Example API client structure
- Async HTTP requests
- JSON serialization/deserialization
- Error handling for network operations

## 🔧 Project Structure

```
rust-features-demo/
├── Cargo.toml              # Dependencies and project metadata
├── src/
│   ├── main.rs            # Main demonstration program
│   └── common_errors.rs   # Common compile errors and fixes
└── README.md              # This file
```

## 📦 Dependencies

- **tokio**: Async runtime for concurrent programming
- **serde**: Serialization/deserialization framework
- **serde_json**: JSON support for serde
- **reqwest**: HTTP client for API calls
- **rayon**: Data parallelism library
- **futures**: Async utilities

## 🎯 Key Features Demonstrated

### Ownership & Borrowing
```rust
fn demonstrate_ownership() {
    let s1 = String::from("hello");
    let s2 = &s1;  // Borrow, not move
    println!("s1: {}, s2: {}", s1, s2);  // Both valid
}
```

### Pattern Matching
```rust
match shape {
    Shape::Circle { radius } => format!("○ Circle (r={})", radius),
    Shape::Rectangle { width, height } => format!("▭ Rectangle ({}x{})", width, height),
    Shape::Triangle { base, height } => format!("△ Triangle (b={}, h={})", base, height),
}
```

### Concurrency
```rust
// Thread-safe counter
let counter = Arc::new(Mutex::new(0));

// Async tasks
let tasks = vec![
    async_task(1, 100),
    async_task(2, 200),
    async_task(3, 150),
];
let results = futures::future::join_all(tasks).await;
```

### Functional Programming
```rust
let result: Vec<_> = numbers
    .iter()
    .filter(|&x| x % 2 == 0)      // Keep even numbers
    .map(|x| x * x)               // Square them
    .collect();
```

## 🚨 Common Compile Errors

The `common_errors.rs` file contains examples of frequent Rust compile errors:

1. **Borrowing Errors**: Mutable/immutable borrow conflicts
2. **Use After Move**: Using values after they've been moved
3. **Lifetime Errors**: References outliving their data
4. **Type Mismatches**: Incorrect type assignments
5. **Missing Traits**: Required trait implementations
6. **Mutability Issues**: Trying to mutate immutable variables

## 🌐 LLM Integration Example

The code includes a mock implementation of LLM API integration:

```rust
async fn call_llm_api(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let request = ChatRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a helpful assistant.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
        max_tokens: Some(150),
        temperature: Some(0.7),
    };
    
    // Replace with actual API call using reqwest
    // let response = reqwest::Client::new()
    //     .post("https://api.openai.com/v1/chat/completions")
    //     .header("Authorization", "Bearer YOUR_API_KEY")
    //     .json(&request)
    //     .send()
    //     .await?;
    
    Ok("Simulated LLM response".to_string())
}
```

## 🔍 Best Practices Demonstrated

1. **Use references when you don't need ownership**
2. **Prefer borrowing over cloning**
3. **Use Result for error handling**
4. **Use Option for nullable values**
5. **Exhaustive pattern matching**
6. **Thread-safe shared state with Arc<Mutex<T>>**
7. **Async/await for concurrent operations**

## 🧪 Testing

The project includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_shape_area
```

## 📈 Performance Tips

- Use `cargo run --release` for optimized builds
- Leverage Rust's zero-cost abstractions
- Use parallel iterators with Rayon for CPU-intensive tasks
- Prefer borrowing over cloning for performance
- Use `Box<dyn Error>` for flexible error handling

## 🔗 Learning Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises
- [Rust Async Book](https://rust-lang.github.io/async-book/)

## 🤝 Contributing

This is a learning project. Feel free to:
- Add more examples
- Improve existing code
- Fix any issues
- Add more comprehensive tests

## 📄 License

This project is for educational purposes and is provided as-is.
