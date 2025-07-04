# Go Language Features Demo

This program demonstrates the top features of the Go programming language, including its famous concurrency model.

## Features Demonstrated

### 1. **Structs and Methods**
- Custom types using structs
- Method receivers
- Public vs private fields (capitalization)

### 2. **Interfaces**
- Implicit interface satisfaction
- Interface{} (empty interface)
- Type assertions

### 3. **Error Handling**
- Multiple return values
- Explicit error handling (no exceptions)
- Named return values

### 4. **Concurrency (Go's Killer Feature)**
- **Goroutines**: Lightweight threads
- **Channels**: Communication between goroutines
- **Select Statement**: Multiplexing on channels
- **Worker Pool Pattern**: Common concurrency pattern
- **Context**: For cancellation and timeouts

### 5. **Memory Management**
- Pointers (but safer than C/C++)
- No pointer arithmetic
- Automatic garbage collection

### 6. **Control Flow**
- **Defer**: Ensures cleanup code runs
- **Panic/Recover**: Exception-like mechanism
- **Range**: Iterator over collections

### 7. **Data Structures**
- **Slices**: Dynamic arrays
- **Maps**: Key-value pairs
- **Arrays**: Fixed-size sequences

### 8. **Functions**
- **Variadic Functions**: Variable arguments
- **Closures**: Functions that capture variables
- **First-class Functions**: Functions as values

### 9. **Concurrency Patterns**
- Producer-Consumer using channels
- Fan-out/Fan-in patterns
- Timeout handling
- Context cancellation

## Counter Examples Included

The program includes several counter examples showing common mistakes:

1. **Defer in loops**: Capturing loop variables incorrectly
2. **Slice modifications**: How sub-slices affect original slices
3. **Goroutine leaks**: Proper channel closing
4. **Context leaks**: Always defer cancel()

## Key Go Philosophy

> "Don't communicate by sharing memory; share memory by communicating"

This program demonstrates Go's unique approach to concurrency using channels instead of traditional mutex-based synchronization.

## Running the Program

```bash
go run main.go
```

or

```bash
go build
./go-features-demo
```

## What Makes Go Special

1. **Simplicity**: Small language specification
2. **Concurrency**: Built-in goroutines and channels
3. **Fast Compilation**: Compiles to native code quickly
4. **Garbage Collection**: Automatic memory management
5. **Strong Typing**: Static typing with inference
6. **No Inheritance**: Composition over inheritance
7. **Explicit Error Handling**: No hidden exceptions

## Output

The program will demonstrate:
- Basic struct and interface usage
- Error handling patterns
- Channel communication
- Concurrent worker pools
- Context usage for timeouts
- Defer statement execution order
- Panic recovery
- Slice and map operations
- Pointer usage
- Variadic functions
- Closures

Watch the output to see goroutines working concurrently and communicating through channels!
