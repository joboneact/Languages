// Package main - Entry point package for executable programs
// In Go, the main package is special - it creates an executable binary
// Other packages are used as libraries
package main

// Import statements - Go's way of including external packages
import (
	"context"   // Context package for cancellation, timeouts, and request-scoped values
	"fmt"       // Format package for formatted I/O operations (Printf, Sprintf, etc.)
	"math/rand" // Random number generation package
	"runtime"   // Runtime package for interacting with Go's runtime system
	"sync"      // Synchronization primitives (WaitGroup, Mutex, etc.)
	"time"      // Time and duration operations
)

// 1. STRUCTS AND INTERFACES
// Go uses structs instead of classes and interfaces for polymorphism
// This demonstrates Go's composition over inheritance philosophy

// Define a struct - Go's way of creating custom types
// struct = keyword to define a structure (composite type)
// Fields with capital letters are exported (public), lowercase are private
type Person struct {
	Name    string // exported field (public) - accessible from other packages
	Age     int    // exported field (public) - int is Go's integer type
	private string // unexported field (private) - only accessible within this package
}

// Method on struct - receiver function
// func = keyword to define a function
// (p *Person) = pointer receiver - allows modification of the struct
// *Person = pointer to Person struct (asterisk means "pointer to")
// string = return type
func (p *Person) Greet() string {
	// fmt.Sprintf = formatted string printing (like printf but returns string)
	// %s = string placeholder, %d = integer placeholder
	return fmt.Sprintf("Hello, I'm %s and I'm %d years old", p.Name, p.Age)
}

// Interface definition - describes behavior, not implementation
// interface = keyword to define an interface (contract)
// Interfaces in Go are satisfied implicitly - no "implements" keyword needed
type Speaker interface {
	Greet() string // Method signature - any type with this method implements Speaker
}

// Any type that has a Greet() method implements Speaker automatically
// This is Go's implicit interface satisfaction - duck typing with compile-time checking

// 2. MULTIPLE RETURN VALUES AND ERROR HANDLING
// Go functions can return multiple values, commonly used for error handling
// Go philosophy: explicit error handling instead of exceptions

// Function with multiple return values
// func = function keyword
// (a, b float64) = parameters with types (float64 = 64-bit floating point)
// (float64, error) = return types (tuple of value and error)
func divide(a, b float64) (float64, error) {
	// if = conditional statement keyword
	if b == 0 {
		// Go uses explicit error handling instead of exceptions
		// fmt.Errorf = creates a formatted error
		// return = keyword to return values from function
		return 0, fmt.Errorf("division by zero")
	}
	// nil = Go's zero value for pointers, interfaces, slices, maps, channels, and functions
	// Similar to null in other languages but type-safe
	return a / b, nil
}

// Named return values - can be useful but sometimes overused
// (result string, count int, err error) = named return parameters
// These are pre-declared variables that can be used in the function body
func processData(input string) (result string, count int, err error) {
	// if with condition - Go's simple conditional
	if input == "" {
		// err is already declared in the function signature
		err = fmt.Errorf("input cannot be empty")
		return // naked return - returns the named values (result="", count=0, err=error)
	}
	// Assignment to named return values
	result = "processed: " + input // string concatenation with +
	count = len(input)             // len() = built-in function to get length
	return                         // naked return - returns the named values
}

// 3. GOROUTINES AND CHANNELS - Go's concurrency model
// Goroutines are lightweight threads managed by Go runtime
// Much cheaper than OS threads - can have millions of goroutines

// Worker function for demonstrating goroutines
// id int = worker identifier
// jobs <-chan int = receive-only channel (arrow points left = receive)
// results chan<- int = send-only channel (arrow points right = send)
// wg *sync.WaitGroup = pointer to WaitGroup for synchronization
func worker(id int, jobs <-chan int, results chan<- int, wg *sync.WaitGroup) {
	// defer = keyword to delay execution until function returns
	// Ensures wg.Done() is called even if function panics
	defer wg.Done() // Signal completion when function exits
	
	// for-range loop over channel
	// range = keyword to iterate over collections (slices, maps, channels)
	// job := range jobs = receive values from jobs channel until closed
	for job := range jobs {
		// fmt.Printf = formatted print to stdout
		// %d = decimal integer placeholder
		fmt.Printf("Worker %d processing job %d\n", id, job)
		
		// Simulate work with random sleep
		// time.Sleep = pause execution for specified duration
		// time.Duration = type for representing time intervals
		// rand.Intn(1000) = random integer between 0 and 999
		time.Sleep(time.Duration(rand.Intn(1000)) * time.Millisecond)
		
		// Send result to results channel
		// <- = channel send operator
		results <- job * 2
	}
}

// 4. CHANNELS - Go's way of communication between goroutines
// "Don't communicate by sharing memory; share memory by communicating"
// Channels are Go's implementation of CSP (Communicating Sequential Processes)

func demonstrateChannels() {
	// fmt.Println = print line with newline
	fmt.Println("\n=== CHANNEL DEMONSTRATION ===")
	
	// Unbuffered channel - synchronous communication
	// make = built-in function to create channels, slices, and maps
	// chan string = channel type that carries string values
	ch := make(chan string)
	
	// go = keyword to start a goroutine (concurrent execution)
	// func() = anonymous function (lambda/closure)
	go func() {
		// <- = channel send operator (blocking operation for unbuffered channels)
		ch <- "Hello from goroutine!"
	}()
	
	// := = short variable declaration (type inferred)
	// <-ch = channel receive operator (blocking until value received)
	message := <-ch
	fmt.Println("Received:", message)
	
	// Buffered channel - asynchronous up to buffer size
	// make(chan int, 3) = buffered channel with capacity of 3
	bufferedCh := make(chan int, 3)
	
	// These sends don't block because buffer has space
	bufferedCh <- 1
	bufferedCh <- 2
	bufferedCh <- 3
	// bufferedCh <- 4 // This would block since buffer is full
	
	// Receive all values from buffered channel
	// Multiple assignment from channel receives
	fmt.Println("Buffered values:", <-bufferedCh, <-bufferedCh, <-bufferedCh)
	
	// Channel direction - can restrict channels to send-only or receive-only
	// chan<- string = send-only channel type
	// <-chan string = receive-only channel type
	sendOnly := make(chan<- string)   // can only send
	receiveOnly := make(<-chan string) // can only receive
	
	// Close channels to avoid goroutine leaks in this demo
	// close() = built-in function to close a channel
	close(sendOnly)
	// receiveOnly is already closed (zero value)
	_ = receiveOnly // _ = blank identifier to ignore unused variable
}

// 5. SELECT STATEMENT - multiplexing on channels
// select = keyword for non-blocking channel operations
// Similar to switch but works with channels
func demonstrateSelect() {
	fmt.Println("\n=== SELECT STATEMENT DEMONSTRATION ===")
	
	// Create two channels for demonstration
	ch1 := make(chan string)
	ch2 := make(chan string)
	
	// Start first goroutine - sends to ch1 after 100ms
	go func() {
		// time.Sleep = pause execution for specified duration
		time.Sleep(100 * time.Millisecond)
		ch1 <- "from ch1"
	}()
	
	// Start second goroutine - sends to ch2 after 200ms
	go func() {
		time.Sleep(200 * time.Millisecond)
		ch2 <- "from ch2"
	}()
	
	// Select allows you to wait on multiple channel operations
	// for = loop keyword (C-style for loop)
	// i := 0 = initialization (short variable declaration)
	// i < 2 = condition
	// i++ = increment (i = i + 1)
	for i := 0; i < 2; i++ {
		// select = non-blocking channel multiplexing
		select {
		// case = individual channel operation
		// msg1 := <-ch1 = receive from ch1 and assign to msg1
		case msg1 := <-ch1:
			fmt.Println("Received", msg1)
		case msg2 := <-ch2:
			fmt.Println("Received", msg2)
		// time.After = returns a channel that receives after timeout
		case <-time.After(300 * time.Millisecond):
			fmt.Println("Timeout!")
		}
	}
}

// 6. CONTEXT - for cancellation and timeouts
// Context carries deadlines, cancellation signals, and request-scoped values
// Essential for controlling goroutine lifecycles and preventing resource leaks
func demonstrateContext() {
	fmt.Println("\n=== CONTEXT DEMONSTRATION ===")
	
	// Context with timeout
	// context.WithTimeout = creates a context that cancels after timeout
	// context.Background() = returns an empty context (root context)
	// cancel = function to manually cancel the context
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	
	// defer = ensures cancel is called when function returns
	// IMPORTANT: Always defer cancel to prevent context leaks
	defer cancel() // Always defer cancel to prevent context leaks
	
	// Start a goroutine that may take longer than timeout
	go func() {
		// select = choose between multiple channel operations
		select {
		// This case would complete after 3 seconds
		case <-time.After(3 * time.Second):
			fmt.Println("Work completed")
		// This case triggers when context is cancelled (timeout or manual)
		// ctx.Done() = returns a channel that closes when context is done
		case <-ctx.Done():
			// ctx.Err() = returns the error that caused context cancellation
			fmt.Println("Work cancelled:", ctx.Err())
		}
	}()
	
	// Sleep longer than context timeout to see cancellation
	time.Sleep(2500 * time.Millisecond)
}

// 7. DEFER STATEMENT - ensures function calls happen when function exits
// defer = keyword to delay execution until surrounding function returns
// Commonly used for cleanup operations (closing files, unlocking mutexes, etc.)
func demonstrateDefer() {
	fmt.Println("\n=== DEFER DEMONSTRATION ===")
	
	// Defer calls are executed in LIFO order (Last In, First Out)
	// Like a stack - last deferred call executes first
	defer fmt.Println("First defer")  // Executes third
	defer fmt.Println("Second defer") // Executes second  
	defer fmt.Println("Third defer")  // Executes first
	
	// Regular statement executes immediately
	fmt.Println("Regular statement")
	
	// Defer is commonly used for cleanup
	// := = short variable declaration (var file string = "example.txt")
	file := "example.txt" // Simulating file handling
	
	// defer with anonymous function
	// func() = anonymous function (closure)
	defer func() {
		// This cleanup code runs when demonstrateDefer() returns
		fmt.Printf("Cleaning up %s\n", file)
	}()
	
	// Defer with loop - common gotcha demonstration
	fmt.Println("Defer in loop (potential issue):")
	
	// for i := 0; i < 3; i++ = C-style for loop
	for i := 0; i < 3; i++ {
		// CORRECT WAY: Pass the loop variable as parameter
		// defer func(val int) = anonymous function with parameter
		defer func(val int) {
			fmt.Printf("Deferred value: %d\n", val)
		}(i) // (i) = pass current value of i as argument
		// This captures the VALUE of i at each iteration
	}
	
	// COUNTER EXAMPLE - what NOT to do:
	// for i := 0; i < 3; i++ {
	//     defer func() {
	//         fmt.Printf("Wrong way: %d\n", i) // This would print 3, 3, 3
	//     }()
	// }
	// The above captures the VARIABLE i, not its value
	// When deferred functions execute, loop has finished and i=3
}

// 8. PANIC AND RECOVER - Go's exception handling mechanism
// panic = built-in function to trigger a runtime panic (like throwing an exception)
// recover = built-in function to regain control after a panic (like catching an exception)
// Unlike exceptions, panics should be rare and used for truly exceptional situations
func demonstratePanicRecover() {
	fmt.Println("\n=== PANIC AND RECOVER DEMONSTRATION ===")
	
	// Set up panic recovery using defer
	// defer = ensures this function runs when demonstratePanicRecover() exits
	defer func() {
		// recover() = built-in function to catch panics
		// if r := recover(); r != nil = if a panic occurred
		if r := recover(); r != nil {
			// r = the value passed to panic()
			// %v = general value placeholder (works with any type)
			fmt.Printf("Recovered from panic: %v\n", r)
		}
	}()
	
	fmt.Println("About to panic...")
	
	// panic() = built-in function to trigger a panic
	// Stops normal execution and starts unwinding the call stack
	panic("Something went wrong!")
	
	// This line will never execute because panic stops execution
	fmt.Println("This won't be printed")
}

// 9. SLICES AND MAPS - Go's dynamic data structures
// Slices are dynamic arrays, Maps are key-value pairs (hash tables)
// These are reference types (unlike arrays which are value types)
func demonstrateSlicesAndMaps() {
	fmt.Println("\n=== SLICES AND MAPS DEMONSTRATION ===")
	
	// Slices - dynamic arrays
	// var slice []int = declare a nil slice (no underlying array)
	var slice []int // nil slice (length=0, capacity=0, underlying array=nil)
	
	// append() = built-in function to add elements to slice
	// Returns a new slice (may have different underlying array)
	slice = append(slice, 1, 2, 3, 4, 5)
	
	// fmt.Printf = formatted print
	// %v = general value (prints the slice contents)
	// %d = decimal integer
	// len() = built-in function to get length
	// cap() = built-in function to get capacity
	fmt.Printf("Slice: %v, Length: %d, Capacity: %d\n", slice, len(slice), cap(slice))
	
	// Slice operations - creating sub-slices
	// slice[1:4] = slice from index 1 to 3 (4 is exclusive)
	// This creates a new slice header but shares underlying array
	subSlice := slice[1:4] // [2, 3, 4]
	fmt.Printf("Sub-slice: %v\n", subSlice)
	
	// COUNTER EXAMPLE - slice gotcha
	// Modifying sub-slice affects original slice since they share underlying array
	// This is a common source of bugs in Go programs
	subSlice[0] = 99 // Changes slice[1] from 2 to 99
	fmt.Printf("After modifying sub-slice, original slice: %v\n", slice)
	
	// Maps - key-value pairs (hash tables)
	// make(map[string]int) = create an empty map with string keys and int values
	// map[KeyType]ValueType = map type syntax
	userAges := make(map[string]int)
	
	// Map assignment syntax
	userAges["Alice"] = 30
	userAges["Bob"] = 25
	
	// Check if key exists - the "comma ok" idiom
	// if age, exists := userAges["Alice"]; exists
	// age = value, exists = boolean indicating if key was found
	if age, exists := userAges["Alice"]; exists {
		fmt.Printf("Alice's age: %d\n", age)
	}
	
	// delete() = built-in function to remove key from map
	delete(userAges, "Bob")
	fmt.Printf("Map after deleting Bob: %v\n", userAges)
}

// 10. POINTERS - memory addresses (but safer than C/C++)
// Go has pointers but no pointer arithmetic (safer than C/C++)
// Used for efficiency and to allow functions to modify values
func demonstratePointers() {
	fmt.Println("\n=== POINTERS DEMONSTRATION ===")
	
	// := = short variable declaration (var x int = 42)
	x := 42
	
	// &x = address-of operator (gets memory address of x)
	// p := &x = p is a pointer to x
	// *int = pointer to int type
	p := &x // p is a pointer to x
	
	// Print various pointer-related values
	fmt.Printf("Value of x: %d\n", x)    // Direct value
	fmt.Printf("Address of x: %p\n", &x) // %p = pointer address format
	fmt.Printf("Value of p: %p\n", p)    // p contains the address
	
	// *p = dereference operator (gets value at address)
	// "Follow the pointer" to get the actual value
	fmt.Printf("Value pointed to by p: %d\n", *p)
	
	// Modify through pointer
	// *p = 100 means "set the value at address p to 100"
	*p = 100
	fmt.Printf("After modifying through pointer, x = %d\n", x)
	
	// Go doesn't have pointer arithmetic (safer than C/C++)
	// This prevents many common security vulnerabilities
	// p++ // This would cause a compile error
	// p + 1 // This would also cause a compile error
}

// 11. WORKER POOL PATTERN - common concurrency pattern
// Demonstrates how to limit concurrency and process work efficiently
// Pattern: Fixed number of workers processing jobs from a queue
func demonstrateWorkerPool() {
	fmt.Println("\n=== WORKER POOL DEMONSTRATION ===")
	
	// const = keyword for compile-time constants
	// These values cannot be changed after declaration
	const numWorkers = 3 // Number of worker goroutines
	const numJobs = 9    // Number of jobs to process
	
	// Create buffered channels for job distribution
	// make(chan int, numJobs) = buffered channel with capacity numJobs
	// Buffered channels don't block sends until buffer is full
	jobs := make(chan int, numJobs)       // Jobs to be processed
	results := make(chan int, numJobs)    // Results from workers
	
	// var wg sync.WaitGroup = declare a WaitGroup for synchronization
	// WaitGroup is used to wait for multiple goroutines to complete
	var wg sync.WaitGroup
	
	// Start worker goroutines
	// for w := 1; w <= numWorkers; w++ = traditional for loop
	for w := 1; w <= numWorkers; w++ {
		// wg.Add(1) = increment WaitGroup counter
		// Must be called before starting goroutine
		wg.Add(1)
		
		// go = keyword to start goroutine
		// worker() = function defined earlier that processes jobs
		go worker(w, jobs, results, &wg) // &wg = address of WaitGroup
	}
	
	// Send jobs to workers
	// for j := 1; j <= numJobs; j++ = loop to send all jobs
	for j := 1; j <= numJobs; j++ {
		jobs <- j // Send job number to jobs channel
	}
	// close(jobs) = close channel to signal no more jobs
	// Workers will exit their range loop when channel is closed
	close(jobs)
	
	// Wait for all workers to complete and close results channel
	// Start anonymous goroutine to avoid blocking
	go func() {
		// wg.Wait() = block until WaitGroup counter reaches zero
		// This happens when all workers call wg.Done()
		wg.Wait()
		// close(results) = close results channel when all work is done
		close(results)
	}()
	
	// Collect results from workers
	// for result := range results = receive all results until channel closed
	for result := range results {
		fmt.Printf("Result: %d\n", result)
	}
}

// 12. INTERFACES AND TYPE ASSERTIONS
// Demonstrates Go's interface system and runtime type checking
// Interfaces are satisfied implicitly - no explicit "implements" declaration needed
func demonstrateInterfaces() {
	fmt.Println("\n=== INTERFACES DEMONSTRATION ===")
	
	// var s Speaker = declare variable of interface type
	// Interface variables can hold any type that implements the interface
	var s Speaker
	
	// &Person{...} = create pointer to Person struct
	// & = address-of operator, creates pointer to the struct literal
	person := &Person{Name: "John", Age: 30}
	
	// s = person = assign Person pointer to Speaker interface
	// This works because Person has a Greet() method (implicit interface satisfaction)
	s = person // Person implements Speaker interface
	
	// s.Greet() = call method through interface
	// Go uses dynamic dispatch to call the correct method
	fmt.Println(s.Greet())
	
	// Type assertion - checking and extracting concrete type from interface
	// if p, ok := s.(*Person); ok = "comma ok" idiom
	// p = the concrete value if assertion succeeds
	// ok = boolean indicating if assertion succeeded
	// *Person = pointer to Person type
	if p, ok := s.(*Person); ok {
		fmt.Printf("Successfully asserted to Person: %s\n", p.Name)
	}
	
	// Empty interface - can hold any type
	// interface{} = empty interface type (no methods required)
	// Similar to Object in Java or void* in C, but type-safe
	var anything interface{}
	
	// anything can hold any type
	anything = 42
	// %v = general value format, %T = type format
	fmt.Printf("anything = %v (type: %T)\n", anything, anything)
	
	// Reassign to different type
	anything = "hello"
	fmt.Printf("anything = %v (type: %T)\n", anything, anything)
}

// 13. VARIADIC FUNCTIONS - functions with variable number of arguments
// Variadic functions can accept zero or more arguments of a specified type
// The ... syntax creates a slice from the arguments

// sum function with variadic parameter
// nums ...int = variadic parameter (zero or more int arguments)
// ...int = slice of int created from the arguments
func sum(nums ...int) int {
	// total := 0 = short variable declaration with initialization
	total := 0
	
	// for _, num := range nums = for-range loop over slice
	// _ = blank identifier (ignores the index)
	// num = current element value
	// range = keyword to iterate over collections
	for _, num := range nums {
		total += num // total = total + num
	}
	
	return total
}

func demonstrateVariadicFunctions() {
	fmt.Println("\n=== VARIADIC FUNCTIONS DEMONSTRATION ===")
	
	// Call variadic function with different numbers of arguments
	// sum(1, 2, 3) = pass 3 individual arguments
	fmt.Printf("sum(1, 2, 3) = %d\n", sum(1, 2, 3))
	
	// sum(1, 2, 3, 4, 5) = pass 5 individual arguments
	fmt.Printf("sum(1, 2, 3, 4, 5) = %d\n", sum(1, 2, 3, 4, 5))
	
	// Spread slice into variadic function
	// []int{1, 2, 3, 4, 5} = slice literal
	numbers := []int{1, 2, 3, 4, 5}
	
	// sum(numbers...) = spread operator
	// ... = unpacks slice elements as individual arguments
	// Equivalent to sum(1, 2, 3, 4, 5)
	fmt.Printf("sum(numbers...) = %d\n", sum(numbers...))
}

// 14. CLOSURES - functions that capture variables from their scope
// Closures are anonymous functions that can access variables from outer scope
// They "close over" variables, hence the name "closure"
func demonstrateClosures() {
	fmt.Println("\n=== CLOSURES DEMONSTRATION ===")
	
	// Create a closure that captures a variable from outer scope
	// counter := 0 = variable in outer scope
	counter := 0
	
	// increment := func() int = assign anonymous function to variable
	// func() int = function type (no parameters, returns int)
	increment := func() int {
		// counter++ = increment counter from outer scope
		// The function "closes over" the counter variable
		counter++
		return counter
	}
	
	// Call the closure multiple times
	// Each call modifies the same counter variable
	fmt.Printf("First call: %d\n", increment())  // counter becomes 1
	fmt.Printf("Second call: %d\n", increment()) // counter becomes 2
	fmt.Printf("Third call: %d\n", increment()) // counter becomes 3
	
	// Factory function returning closures
	// multiplier := func(factor int) func(int) int = function that returns function
	// func(int) int = function type (takes int, returns int)
	multiplier := func(factor int) func(int) int {
		// Return anonymous function that captures 'factor'
		// return func(x int) int = return function literal
		return func(x int) int {
			// x * factor = multiply parameter by captured factor
			return x * factor
		}
	}
	
	// Create specialized functions using the factory
	// double := multiplier(2) = creates closure with factor=2
	double := multiplier(2)
	// triple := multiplier(3) = creates closure with factor=3
	triple := multiplier(3)
	
	// Each closure has its own copy of the captured variable
	fmt.Printf("double(5) = %d\n", double(5)) // 5 * 2 = 10
	fmt.Printf("triple(5) = %d\n", triple(5)) // 5 * 3 = 15
}

// 15. MAIN FUNCTION - entry point
// func main() = special function that serves as program entry point
// Must be in package main, takes no parameters, returns no values
func main() {
	// Program header with system information
	fmt.Println("=== GO LANGUAGE FEATURES DEMONSTRATION ===")
	
	// runtime.Version() = returns Go version string
	// %s = string format placeholder
	fmt.Printf("Go version: %s\n", runtime.Version())
	
	// runtime.NumCPU() = returns number of logical CPUs
	// %d = decimal integer format placeholder
	fmt.Printf("Number of CPUs: %d\n", runtime.NumCPU())
	
	// runtime.NumGoroutine() = returns number of active goroutines
	// At program start, only main goroutine exists
	fmt.Printf("Number of Goroutines: %d\n", runtime.NumGoroutine())
	
	// Demonstrate basic struct usage
	// Person{Name: "Alice", Age: 25} = struct literal initialization
	person := Person{Name: "Alice", Age: 25}
	
	// fmt.Printf with %+v = detailed struct format (shows field names and values)
	// \n = newline character
	fmt.Printf("\nPerson: %+v\n", person)
	
	// person.Greet() = method call on struct
	fmt.Println(person.Greet())
	
	// Demonstrate error handling with multiple return values
	// result, err := divide(10, 2) = multiple assignment from function
	result, err := divide(10, 2)
	
	// if err != nil = Go's idiomatic error checking
	// nil = zero value for pointers, interfaces, slices, maps, channels, functions
	if err != nil {
		// %v = general value format (works with any type)
		fmt.Printf("Error: %v\n", err)
	} else {
		// %.2f = floating point with 2 decimal places
		fmt.Printf("10 / 2 = %.2f\n", result)
	}
	
	// Demonstrate error case - division by zero
	// _ = blank identifier (ignores the result value)
	// err = reuse err variable (no := needed)
	_, err = divide(10, 0)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	}
	
	// Demonstrate named returns
	// text, count, err := processData("hello") = three return values
	text, count, err := processData("hello")
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		// %s = string format, %d = decimal integer format
		fmt.Printf("Processed: %s, Count: %d\n", text, count)
	}
	
	// Run all demonstration functions
	// These function calls execute sequentially (synchronously)
	demonstrateChannels()           // Channel communication
	demonstrateSelect()             // Select statement multiplexing
	demonstrateContext()            // Context timeout handling
	demonstrateDefer()              // Defer statement execution
	demonstratePanicRecover()       // Panic and recover mechanism
	demonstrateSlicesAndMaps()      // Slice and map operations
	demonstratePointers()           // Pointer manipulation
	demonstrateWorkerPool()         // Concurrent worker pool
	demonstrateInterfaces()         // Interface usage and type assertions
	demonstrateVariadicFunctions()  // Variable argument functions
	demonstrateClosures()           // Closure examples
	
	// Program completion message
	fmt.Println("\n=== PROGRAM COMPLETED ===")
}
