# Chapter 6: Collections and Error Handling

### 1. Introduction

In Rust, **collections** are growable data structures and **error handling** ensures your program can gracefully handle unexpected situations. This chapter explores:

- **Vectors**, **Strings**, and **HashMaps** – three common collections you’ll use frequently in Rust.
- The **Option** and **Result** types – how Rust expresses the possibility of *no value* or an *error*.
- Error handling strategies – using `match`, `unwrap`, and the `?` operator, plus best practices for robust error management.

---

### 1. Working with Common Collections

#### 1.1 Vectors

A *vector* (often called `Vec<T>`) is a growable array of elements of the same type.

```rust
fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);
    numbers.push(20);
    numbers.push(30);

    println!("First element: {}", numbers[0]);
    println!("Full vector: {:?}", numbers);
}
```

- You can create a vector using `Vec::new()` or a macro like `vec![1, 2, 3]`.
- Elements can be added with `push` and removed with `pop`.
- Accessing an element out of bounds will cause a **panic** if you use indexing (`numbers[99]`). You can use `numbers.get(99)` which returns an `Option` to handle it more safely.

#### 1.2 Strings

Rust’s **String** type is a growable, UTF-8 encoded text buffer.

```rust
fn main() {
    let mut greeting = String::from("Hello");
    greeting.push_str(", Rust!");
    println!("{}", greeting);
}
```

- Unlike string *slices* (`&str`), a `String` owns the text data and can be modified.
- Methods like `push_str` (add a string slice) or `push` (add a single character) let you change it.

#### 1.3 HashMaps

A **HashMap** stores key-value pairs, letting you look up a value by providing a key.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Score of Blue: {:?}", scores.get("Blue"));
    println!("All scores: {:?}", scores);
}
```

- You can `insert` key-value pairs, and use `get` to retrieve values.
- Keys and values can be any type that implements the `Eq` and `Hash` traits (for keys) and can store data.

---

### 2. Option and Result Types

Rust uses *generic enums* to represent potential absence (`Option`) and possible failure (`Result`).

- **Option** – used when a value might not exist.
  
```rust
fn get_nth_element(vec: &Vec<i32>, index: usize) -> Option<i32> {
    vec.get(index).copied()
}
```

If `index` is out of bounds, `vec.get(index)` returns `None`; otherwise it returns `Some(value)`.

- **Result** – used when a function might fail in a way you want to handle.
  
```rust
use std::fs::File;

fn open_file() -> Result<File, std::io::Error> {
    File::open("hello.txt")
}
```

A `Result` is either `Ok(T)` if successful or `Err(E)` if an error occurred.

---

### 3. Handling Errors

Rust encourages explicit error handling. Three common approaches:

1. **match**  
   Use a `match` to handle each outcome clearly:

```rust
match open_file() {
    Ok(file) => println!("File opened successfully: {:?}", file),
    Err(e) => println!("Failed to open file: {:?}", e),
}
```

2. **unwrap** and **expect**  
   These methods let you access the `Ok` value or **panic** (crash) if it’s an error:

```rust
let file = open_file().unwrap();
```

- `unwrap` is quick but doesn’t provide a custom error message.
- `expect("message")` gives an error message if it panics.

3. **The ? Operator**  
   Propagates errors up the call stack. If the result is `Err`, it returns it immediately from the current function.

```rust
fn open_and_process_file() -> Result<(), std::io::Error> {
    let file = File::open("hello.txt")?;
    // Process file here...
    Ok(())
}
```

---

### 4. Strategies for Robust Error Management in Rust

- Use **`match`** or **`if let`** to handle different outcomes in a *safe, descriptive* manner.
- Only use **`unwrap`** or **`expect`** when you’re *sure* an error indicates a bug or there’s no possible recovery.
- Prefer **`?`** for convenient propagation of errors when they can be handled at a *higher level*.
- Consider custom error types if your application has multiple error scenarios and you want more detailed handling.

---

By mastering **collections** and **error handling**, you’ll be well-equipped to tackle most Rust programming challenges. **Vectors**, **Strings**, and **HashMaps** give you the tools to store and manipulate data, while **Option** and **Result** let you gracefully handle absent data and potential failures.

### Next Up: [Chapter 7: Modules and Crates](../07-modules-and-crates/README.md)
Learn how Rust organizes code and manages dependencies.

<p align="center"> <sub>© 2025 Learn Rust by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. Licensed under the <a href="../LICENSE">MIT License</a>.</sub> </p>


