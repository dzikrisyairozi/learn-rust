# Modules and Crates in Rust

## Overview

This demonstration showcases Rust's module system and crate organization through a practical library management system. It demonstrates:

- **Module organization** with nested structures
- **Public API design** with selective exports
- **Separation of concerns** using modules
- **Dependency management** with external crates

## Project Structure

### 1. Crate Layout
- Binary crate (`main.rs`) for the executable
- Library crate (`lib.rs`) for reusable code
- Organized modules in separate directories

### 2. Module Hierarchy
- `models/` - Data structures
  - `book.rs` - Book-related structures
  - `user.rs` - User-related structures
- `services/` - Business logic
  - `library.rs` - Library management
  - `auth.rs` - Authentication services

## Running the Application

### Run the Project

To run the demonstration:

```bash
cargo run
```

You will see output similar to this:

```bash
Initializing library system...
Found book: Book { 
    id: 1, 
    title: "The Rust Programming Language", 
    author: "Dzikri Syairozi and Lebron James",
    published_date: 2024-01-01T00:00:00Z,
    is_available: true 
}
```

### Run Tests

To run the test suite:

```bash
cargo test
```

## Key Features

### 1. Module Organization
- Clear separation of models and services
- Hierarchical module structure
- Public API design

### 2. Code Reusability
- Library crate for shared code
- Re-exports for convenience
- External dependency usage

### 3. Best Practices
- Selective visibility with `pub`
- Logical grouping of related code
- Clear module boundaries

## Usage Examples

### 1. Importing the Library

```rust
use library_system::{Book, User, Library};
```

### 2. Creating and Using Models

```rust
let book = Book::new(1, "Title", "Author");
let user = User::new(1, "username");
```

### 3. Using Services

```rust
let mut library = Library::new();
library.add_book(book);
library.register_user(user);
```

## Conclusion

This demonstration shows how to:
- Organize code using Rust's module system
- Create reusable library crates
- Manage public APIs effectively
- Structure a real-world Rust application