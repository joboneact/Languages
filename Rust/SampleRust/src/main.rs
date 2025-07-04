// Comprehensive Rust Features Demo
// This program demonstrates Rust's top features including concurrency, 
// functional programming, pattern matching, and more

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::time::sleep;
use serde::{Deserialize, Serialize};
use rayon::prelude::*;

// ============================================================================
// 1. ALGEBRAIC DATA TYPES & PATTERN MATCHING
// ============================================================================

// Enum with associated data (Sum Type)
#[derive(Debug, Clone, PartialEq)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

// Option and Result types are built-in algebraic data types
#[derive(Debug)]
enum CustomResult<T, E> {
    Ok(T),
    Err(E),
}

// ============================================================================
// 2. GENERICS & TYPE INFERENCE
// ============================================================================

// Generic struct
#[derive(Debug, Clone)]
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
    
    fn get(&self) -> &T {
        &self.value
    }
}

// Generic function with trait bounds
fn process_data<T, U>(data: T, processor: impl Fn(T) -> U) -> U {
    processor(data)
}

// ============================================================================
// 3. TRAITS (INTERFACES) & ABSTRACTIONS
// ============================================================================

trait Drawable {
    fn draw(&self) -> String;
    fn area(&self) -> f64;
    
    // Default implementation
    fn describe(&self) -> String {
        format!("This shape has an area of {:.2}", self.area())
    }
}

impl Drawable for Shape {
    fn draw(&self) -> String {
        match self {
            Shape::Circle { radius } => format!("○ Circle (r={})", radius),
            Shape::Rectangle { width, height } => format!("▭ Rectangle ({}x{})", width, height),
            Shape::Triangle { base, height } => format!("△ Triangle (b={}, h={})", base, height),
        }
    }
    
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}

// ============================================================================
// 4. FUNCTIONAL PROGRAMMING & HIGHER-ORDER FUNCTIONS
// ============================================================================

// Function that takes a closure
fn apply_operation<F>(numbers: Vec<i32>, operation: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    numbers.into_iter().map(operation).collect()
}

// Function that returns a closure
fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

// ============================================================================
// 5. IMMUTABILITY & OWNERSHIP
// ============================================================================

#[derive(Debug, Clone)]
struct ImmutableData {
    values: Vec<i32>,
    metadata: String,
}

impl ImmutableData {
    fn new(values: Vec<i32>, metadata: String) -> Self {
        ImmutableData { values, metadata }
    }
    
    // Method that doesn't mutate self
    fn add_value(&self, value: i32) -> Self {
        let mut new_values = self.values.clone();
        new_values.push(value);
        ImmutableData {
            values: new_values,
            metadata: self.metadata.clone(),
        }
    }
    
    // Method that creates a new instance with updated metadata
    fn update_metadata(&self, new_metadata: String) -> Self {
        ImmutableData {
            values: self.values.clone(),
            metadata: new_metadata,
        }
    }
}

// ============================================================================
// 6. CONCURRENCY & ASYNC PROGRAMMING
// ============================================================================

// Shared state with Arc<Mutex<T>>
#[derive(Debug)]
struct ThreadSafeCounter {
    value: Arc<Mutex<i32>>,
}

impl ThreadSafeCounter {
    fn new() -> Self {
        ThreadSafeCounter {
            value: Arc::new(Mutex::new(0)),
        }
    }
    
    fn increment(&self) {
        let mut count = self.value.lock().unwrap();
        *count += 1;
    }
    
    fn get_value(&self) -> i32 {
        *self.value.lock().unwrap()
    }
}

impl Clone for ThreadSafeCounter {
    fn clone(&self) -> Self {
        ThreadSafeCounter {
            value: Arc::clone(&self.value),
        }
    }
}

// Async function
async fn async_task(id: u64, delay: u64) -> String {
    sleep(Duration::from_millis(delay)).await;
    format!("Task {} completed after {}ms", id, delay)
}

// ============================================================================
// 7. ERROR HANDLING & RESULT TYPES
// ============================================================================

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

// ============================================================================
// 8. PATTERN MATCHING ADVANCED EXAMPLES
// ============================================================================

fn analyze_option(opt: Option<i32>) -> String {
    match opt {
        Some(x) if x > 0 => format!("Positive number: {}", x),
        Some(x) if x < 0 => format!("Negative number: {}", x),
        Some(0) => "Zero".to_string(),
        None => "No value".to_string(),
        _ => "Unexpected case".to_string(),
    }
}

fn destructure_tuple(data: (i32, &str, bool)) -> String {
    match data {
        (x, "rust", true) => format!("Rust enthusiast with number {}", x),
        (x, lang, false) => format!("Not interested in {}, number: {}", lang, x),
        (x, lang, true) => format!("Likes {}, number: {}", lang, x),
    }
}

// ============================================================================
// 9. MACRO DEMONSTRATION
// ============================================================================

macro_rules! debug_print {
    ($($arg:tt)*) => {
        println!("[DEBUG] {}", format!($($arg)*));
    };
}

// ============================================================================
// 10. ITERATORS & FUNCTIONAL STYLE
// ============================================================================

fn demonstrate_iterators() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Functional style processing
    let result: Vec<_> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)      // Keep even numbers
        .map(|x| x * x)               // Square them
        .collect();
    
    println!("Even numbers squared: {:?}", result);
    
    // Reduce operation
    let sum: i32 = numbers.iter().sum();
    println!("Sum of all numbers: {}", sum);
    
    // Find first element matching condition
    let first_large = numbers.iter().find(|&&x| x > 5);
    println!("First number > 5: {:?}", first_large);
}

// ============================================================================
// 11. COMMON SYNTAX & COMPILE ERROR EXAMPLES
// ============================================================================

fn demonstrate_common_patterns() {
    // Pattern: Using if let for Option unwrapping
    let some_value = Some(42);
    if let Some(x) = some_value {
        println!("Value is: {}", x);
    }
    
    // Pattern: Match with default
    let result = match some_value {
        Some(x) => x * 2,
        None => 0,
    };
    println!("Result: {}", result);
    
    // Pattern: Early return with ?
    fn might_fail() -> Result<i32, &'static str> {
        Ok(42)
    }
    
    fn using_question_mark() -> Result<i32, &'static str> {
        let value = might_fail()?;  // Early return if error
        Ok(value * 2)
    }
    
    match using_question_mark() {
        Ok(val) => println!("Success: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

// ============================================================================
// 12. PARALLEL PROCESSING WITH RAYON
// ============================================================================

fn demonstrate_parallelism() {
    let numbers: Vec<i32> = (0..1000).collect();
    
    // Parallel processing
    let sum: i32 = numbers.par_iter().sum();
    println!("Parallel sum: {}", sum);
    
    // Parallel map
    let squares: Vec<i32> = numbers
        .par_iter()
        .map(|x| x * x)
        .collect();
    
    println!("First 10 squares: {:?}", &squares[..10]);
}

// ============================================================================
// 13. LLM INTEGRATION (CHATGPT/CLAUDE API)
// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatChoice {
    message: ChatMessage,
    finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
    usage: Option<serde_json::Value>,
}

// Mock LLM client (replace with actual API calls)
async fn call_llm_api(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    // This would be your actual API call to OpenAI or Anthropic
    // For demo purposes, we'll simulate it
    
    let request = ChatRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a helpful assistant that explains Rust concepts.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
        max_tokens: Some(150),
        temperature: Some(0.7),
    };
    
    // Simulate API call delay
    sleep(Duration::from_millis(100)).await;
    
    // Mock response
    Ok(format!("LLM Response to '{}': This is a simulated response about Rust concepts. In a real implementation, you would use reqwest to call the actual API.", prompt))
}

// Helper function to interact with LLM
async fn ask_llm_about_rust(topic: &str) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = format!("Explain this Rust concept in simple terms: {}", topic);
    call_llm_api(&prompt).await
}

// ============================================================================
// MAIN FUNCTION - DEMONSTRATING ALL FEATURES
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Comprehensive Rust Features Demo 🦀\n");
    
    // 1. Algebraic Data Types & Pattern Matching
    println!("1. 📐 Algebraic Data Types & Pattern Matching");
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 4.0, height: 3.0 },
        Shape::Triangle { base: 6.0, height: 4.0 },
    ];
    
    for shape in &shapes {
        println!("   {} - Area: {:.2}", shape.draw(), shape.area());
    }
    println!();
    
    // 2. Generics & Type Inference
    println!("2. 🧬 Generics & Type Inference");
    let int_container = Container::new(42);
    let string_container = Container::new("Hello, Rust!".to_string());
    println!("   Int container: {:?}", int_container.get());
    println!("   String container: {:?}", string_container.get());
    
    // Type inference in action
    let processed = process_data(10, |x| x * 2 + 1);
    println!("   Processed data: {}", processed);
    println!();
    
    // 3. Functional Programming
    println!("3. 🔄 Functional Programming");
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = apply_operation(numbers.clone(), |x| x * 2);
    println!("   Original: {:?}", numbers);
    println!("   Doubled: {:?}", doubled);
    
    let multiplier = create_multiplier(3);
    let tripled = apply_operation(numbers, multiplier);
    println!("   Tripled: {:?}", tripled);
    println!();
    
    // 4. Immutability
    println!("4. 🔒 Immutability");
    let data = ImmutableData::new(vec![1, 2, 3], "Initial data".to_string());
    println!("   Original: {:?}", data);
    
    let updated_data = data.add_value(4).update_metadata("Updated data".to_string());
    println!("   Updated: {:?}", updated_data);
    println!("   Original unchanged: {:?}", data);
    println!();
    
    // 5. Concurrency - Thread-based
    println!("5. ⚡ Concurrency - Thread-based");
    let counter = ThreadSafeCounter::new();
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter_clone = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                counter_clone.increment();
            }
            println!("   Thread {} finished", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("   Final counter value: {}", counter.get_value());
    println!();
    
    // 6. Async Concurrency
    println!("6. 🌊 Async Concurrency");
    let tasks = vec![
        async_task(1, 100),
        async_task(2, 200),
        async_task(3, 150),
    ];
    
    let results = futures::future::join_all(tasks).await;
    for result in results {
        println!("   {}", result);
    }
    println!();
    
    // 7. Error Handling
    println!("7. ⚠️ Error Handling");
    match safe_divide(10.0, 2.0) {
        Ok(result) => println!("   10 / 2 = {}", result),
        Err(e) => println!("   Error: {:?}", e),
    }
    
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("   10 / 0 = {}", result),
        Err(e) => println!("   Error: {:?}", e),
    }
    
    match safe_sqrt(-4.0) {
        Ok(result) => println!("   sqrt(-4) = {}", result),
        Err(e) => println!("   Error: {:?}", e),
    }
    println!();
    
    // 8. Advanced Pattern Matching
    println!("8. 🎯 Advanced Pattern Matching");
    let options = vec![Some(5), Some(-3), Some(0), None];
    for opt in options {
        println!("   {}", analyze_option(opt));
    }
    
    let tuples = vec![
        (42, "rust", true),
        (13, "python", false),
        (7, "javascript", true),
    ];
    for tuple in tuples {
        println!("   {}", destructure_tuple(tuple));
    }
    println!();
    
    // 9. Macro Usage
    println!("9. 🔧 Macro Usage");
    debug_print!("This is a debug message with value: {}", 42);
    println!();
    
    // 10. Iterators
    println!("10. 🔗 Iterators & Functional Style");
    demonstrate_iterators();
    println!();
    
    // 11. Common Patterns
    println!("11. 📋 Common Patterns");
    demonstrate_common_patterns();
    println!();
    
    // 12. Parallel Processing
    println!("12. 🚀 Parallel Processing");
    demonstrate_parallelism();
    println!();
    
    // 13. LLM Integration
    println!("13. 🤖 LLM Integration");
    let topics = vec!["ownership", "borrowing", "lifetimes"];
    
    for topic in topics {
        match ask_llm_about_rust(topic).await {
            Ok(response) => println!("   Topic '{}': {}", topic, response),
            Err(e) => println!("   Error asking about '{}': {}", topic, e),
        }
    }
    
    println!("\n🎉 Demo completed successfully!");
    Ok(())
}

// ============================================================================
// COMMON COMPILE ERROR EXAMPLES (COMMENTED OUT)
// ============================================================================

/*
// These examples would cause compile errors - uncomment to see them:

fn compile_error_examples() {
    // 1. Borrowing error
    let mut vec = vec![1, 2, 3];
    let first = &vec[0];  // Immutable borrow
    vec.push(4);          // ERROR: Cannot borrow as mutable
    println!("{}", first);
    
    // 2. Use after move
    let s = String::from("hello");
    let s2 = s;           // Move occurs here
    println!("{}", s);    // ERROR: Use after move
    
    // 3. Lifetime error
    fn dangling_reference() -> &str {
        let s = String::from("hello");
        &s  // ERROR: Returns reference to local variable
    }
    
    // 4. Type mismatch
    let x: i32 = "hello";  // ERROR: Type mismatch
    
    // 5. Missing trait implementation
    #[derive(Debug)]
    struct MyStruct;
    let vec = vec![MyStruct, MyStruct];
    vec.sort();  // ERROR: MyStruct doesn't implement Ord
}
*/

// ============================================================================
// ADDITIONAL UTILITY FUNCTIONS
// ============================================================================

// Demonstrate ownership and borrowing
fn demonstrate_ownership() {
    let s1 = String::from("hello");
    let s2 = &s1;  // Borrow, not move
    println!("s1: {}, s2: {}", s1, s2);  // Both valid
    
    let s3 = s1.clone();  // Explicit clone
    println!("s1: {}, s3: {}", s1, s3);  // Both valid
}

// Demonstrate lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Custom iterator
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shape_area() {
        let circle = Shape::Circle { radius: 1.0 };
        assert!((circle.area() - std::f64::consts::PI).abs() < 0.001);
    }
    
    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0).unwrap(), 5.0);
        assert!(matches!(safe_divide(10.0, 0.0), Err(MathError::DivisionByZero)));
    }
    
    #[test]
    fn test_container() {
        let container = Container::new(42);
        assert_eq!(*container.get(), 42);
    }
}
