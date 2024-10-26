# Collections and Error Handling Demo

## Overview

This demonstration showcases Rust's collection types and error handling mechanisms through a practical grade management system. It includes examples of:

- **Vectors** for storing sequences of grades
- **Strings** for text manipulation
- **HashMaps** for storing student records
- **Custom error types** and error handling patterns

## Running the Application

### Run the Project

To run the demonstration:

```bash
cargo run
```

You will see output similar to this:

```bash
--- Vector Operations ---
Student grades: [85, 92, 78]
Average grade: 85.00

--- String Operations ---
Word count: 5
Essay: Rust is a systems programming language

--- HashMap Operations ---
HashMap demonstration failed: ParseError("No grades found for student: Charlie")

--- File Operations ---
File contents:
Alice: 85, 92, 78
Bob: 88, 79, 95
```

### Run Tests

To run the test suite:

```bash
cargo test
```

You will see output like this:

```bash
running 2 tests
test tests::test_student_grade_validation ... ok
test tests::test_average_grade ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Code Structure

### 1. Custom Error Type
- Defines `AppError` enum for different error scenarios
- Implements `From` trait for error conversion

### 2. Student Structure
- Manages student data and grades
- Implements grade validation and average calculation
- Uses `Result` and `Option` for error handling

### 3. Collection Demonstrations
- Vector operations with grade storage
- String manipulation with essay text
- HashMap usage with student records
- File I/O operations with error handling

## Key Features

### 1. Error Handling
- Custom error types
- Result and Option usage
- Error propagation with `?` operator
- Graceful error reporting

### 2. Collections
- Vector for sequential data
- String manipulation
- HashMap for key-value storage
- File operations

## Test Coverage
- Grade validation testing
- Average calculation verification
- Error handling verification

The example provides a practical demonstration of Rust's collection types and error handling mechanisms in a real-world scenario.