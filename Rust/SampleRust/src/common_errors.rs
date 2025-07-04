// Common Rust Compile Errors - Examples for Learning
// These examples demonstrate common mistakes and how to fix them

// Note: This file won't compile! It's for educational purposes only.
// Uncomment sections one at a time to see specific errors.

#![allow(unused_variables)]
#![allow(dead_code)]

// ============================================================================
// 1. BORROWING AND OWNERSHIP ERRORS
// ============================================================================

/*
// ERROR: Cannot borrow as mutable while immutable borrow exists
fn borrowing_error_1() {
    let mut vec = vec![1, 2, 3];
    let first = &vec[0];  // Immutable borrow
    vec.push(4);          // ERROR: Cannot borrow as mutable
    println!("{}", first);
}

// FIX: Don't hold the immutable borrow across the mutable operation
fn borrowing_fixed_1() {
    let mut vec = vec![1, 2, 3];
    let first_value = vec[0];  // Copy the value instead of borrowing
    vec.push(4);               // OK now
    println!("{}", first_value);
}
*/

// ============================================================================
// 2. USE AFTER MOVE ERRORS
// ============================================================================

/*
// ERROR: Use after move
fn move_error_1() {
    let s = String::from("hello");
    let s2 = s;           // Move occurs here
    println!("{}", s);    // ERROR: Use after move
}

// FIX: Clone instead of move
fn move_fixed_1() {
    let s = String::from("hello");
    let s2 = s.clone();   // Clone instead of move
    println!("{}", s);    // OK now
}

// FIX: Use references
fn move_fixed_2() {
    let s = String::from("hello");
    let s2 = &s;          // Borrow instead of move
    println!("{}", s);    // OK now
}
*/

// ============================================================================
// 3. LIFETIME ERRORS
// ============================================================================

/*
// ERROR: Returns reference to local variable
fn lifetime_error_1() -> &str {
    let s = String::from("hello");
    &s  // ERROR: Returns reference to local variable that will be dropped
}

// FIX: Return owned value
fn lifetime_fixed_1() -> String {
    let s = String::from("hello");
    s  // Return owned value
}

// FIX: Use static lifetime
fn lifetime_fixed_2() -> &'static str {
    "hello"  // String literal has static lifetime
}
*/

// ============================================================================
// 4. TYPE MISMATCH ERRORS
// ============================================================================

/*
// ERROR: Type mismatch
fn type_error_1() {
    let x: i32 = "hello";  // ERROR: Expected i32, found &str
}

// FIX: Use correct type
fn type_fixed_1() {
    let x: &str = "hello";  // Correct type
}

// FIX: Parse string to number
fn type_fixed_2() {
    let x: i32 = "42".parse().unwrap();  // Parse string to i32
}
*/

// ============================================================================
// 5. MISSING TRAIT IMPLEMENTATION ERRORS
// ============================================================================

/*
// ERROR: Missing trait implementation
#[derive(Debug)]
struct MyStruct {
    value: i32,
}

fn trait_error_1() {
    let vec = vec![MyStruct { value: 1 }, MyStruct { value: 2 }];
    vec.sort();  // ERROR: MyStruct doesn't implement Ord
}

// FIX: Implement required traits
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MyStructFixed {
    value: i32,
}

fn trait_fixed_1() {
    let mut vec = vec![MyStructFixed { value: 2 }, MyStructFixed { value: 1 }];
    vec.sort();  // OK now
}
*/

// ============================================================================
// 6. MUTABILITY ERRORS
// ============================================================================

/*
// ERROR: Cannot mutate immutable variable
fn mutability_error_1() {
    let x = 5;
    x = 6;  // ERROR: Cannot assign to immutable variable
}

// FIX: Make variable mutable
fn mutability_fixed_1() {
    let mut x = 5;
    x = 6;  // OK now
}
*/

// ============================================================================
// 7. PATTERN MATCHING ERRORS
// ============================================================================

/*
// ERROR: Non-exhaustive pattern matching
fn pattern_error_1(opt: Option<i32>) -> i32 {
    match opt {
        Some(x) => x,
        // ERROR: Missing None case
    }
}

// FIX: Handle all cases
fn pattern_fixed_1(opt: Option<i32>) -> i32 {
    match opt {
        Some(x) => x,
        None => 0,  // Handle None case
    }
}
*/

// ============================================================================
// 8. CLOSURE CAPTURE ERRORS
// ============================================================================

/*
// ERROR: Closure captures variable by reference
fn closure_error_1() -> impl Fn() -> i32 {
    let x = 5;
    move || x  // This actually works with 'move'
}

// But this would be an error:
fn closure_error_2() -> impl Fn() -> i32 {
    let x = 5;
    || x  // ERROR: Closure may outlive current function
}

// FIX: Use move keyword
fn closure_fixed_1() -> impl Fn() -> i32 {
    let x = 5;
    move || x  // OK with move
}
*/

// ============================================================================
// 9. ASYNC/AWAIT ERRORS
// ============================================================================

/*
// ERROR: Async function not awaited
async fn async_error_1() {
    let future = async { 42 };
    let result = future;  // ERROR: Not awaited
}

// FIX: Await the future
async fn async_fixed_1() {
    let future = async { 42 };
    let result = future.await;  // OK now
}
*/

// ============================================================================
// 10. THREAD SAFETY ERRORS
// ============================================================================

/*
use std::thread;
use std::rc::Rc;

// ERROR: Rc is not Send
fn thread_error_1() {
    let rc = Rc::new(5);
    thread::spawn(move || {
        println!("{}", rc);  // ERROR: Rc cannot be sent between threads
    });
}

// FIX: Use Arc instead
use std::sync::Arc;

fn thread_fixed_1() {
    let arc = Arc::new(5);
    thread::spawn(move || {
        println!("{}", arc);  // OK now
    });
}
*/

// ============================================================================
// BEST PRACTICES TO AVOID ERRORS
// ============================================================================

// 1. Use references when you don't need ownership
fn use_references(data: &Vec<i32>) -> i32 {
    data.iter().sum()
}

// 2. Use clone sparingly, prefer borrowing
fn prefer_borrowing(s: &str) -> String {
    s.to_uppercase()
}

// 3. Use Result for error handling
fn safe_operation(x: i32) -> Result<i32, &'static str> {
    if x >= 0 {
        Ok(x * 2)
    } else {
        Err("Negative number not allowed")
    }
}

// 4. Use Option for nullable values
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&x| x % 2 == 0).copied()
}

// 5. Use exhaustive pattern matching
fn handle_option(opt: Option<i32>) -> String {
    match opt {
        Some(x) if x > 0 => format!("Positive: {}", x),
        Some(x) if x < 0 => format!("Negative: {}", x),
        Some(0) => "Zero".to_string(),
        None => "No value".to_string(),
    }
}

// ============================================================================
// COMPILER HINTS AND ATTRIBUTES
// ============================================================================

// Suppress unused variable warnings
#[allow(unused_variables)]
fn with_unused_var() {
    let x = 5;  // Won't trigger warning
}

// Mark function as deprecated
#[deprecated(note = "Use new_function instead")]
fn old_function() {}

// Conditional compilation
#[cfg(debug_assertions)]
fn debug_only_function() {
    println!("This only runs in debug mode");
}

// ============================================================================
// TESTING ERROR SCENARIOS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_safe_operation() {
        assert_eq!(safe_operation(5), Ok(10));
        assert!(safe_operation(-1).is_err());
    }
    
    #[test]
    fn test_find_first_even() {
        assert_eq!(find_first_even(&[1, 3, 4, 5]), Some(4));
        assert_eq!(find_first_even(&[1, 3, 5]), None);
    }
    
    #[test]
    #[should_panic]
    fn test_panic_scenario() {
        panic!("This test expects a panic");
    }
}

fn main() {
    println!("This file demonstrates common Rust compile errors.");
    println!("Most code is commented out to prevent compilation errors.");
    println!("Uncomment sections one at a time to see specific error messages.");
}
