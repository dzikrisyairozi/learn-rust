# 02. Basics

Welcome to **Chapter 2**, where you’ll get your hands dirty with Rust for the first time! We’ll write a simple program and learn about key language building blocks. By the end, you’ll feel more comfortable with Rust’s style and how it handles variables, types, and control flow.

## 1. Writing Your First Rust Program
Let’s start with the classic “Hello, Rust!” program. Even if you aren’t at your computer right now, reading through will help you understand how Rust code is structured.

1. **Create a New Project**

If you’re at your machine, you can create a new Cargo project by running:

```bash
cargo new hello_rust
cd hello_rust
```
This sets up a basic project with a `src/main.rs` file and a `Cargo.toml` manifest.

2. **Open `src/main.rs`**

By default, you’ll see something like this:
```rust
fn main() {
    println!("Hello, world!");
}
```

3. **Change the Greeting**

Let’s customize it:    
```rust
fn main() {
    println!("Hello, Rust!");
}
```

This will compile and execute your Rust code, printing "Hello, world!" to the console.

4. **Run the Program**

In your project directory, just type:

```bash
cargo run
```
You should see:

```
Hello, Rust!
```

**What happened?**

- The `fn main()` defines the entry point for a Rust program.
- `println!` is a macro (notice the `!`) that prints text to the screen.

It’s that simple! You’ve written and run your first Rust program.

## 2. Variables and Mutability

Rust encourages you to think carefully about how your data changes over time. By default, variables in Rust cannot be changed after you set them—this is known as immutability.

### 2.1. Immutability 
```rust
fn main() {
    let x = 5;
    println!("x is: {}", x);

    // x = 10; // ❌ This won't compile because x is immutable!
}

```

- `let x = 5`; creates an immutable variable named `x` with a value of 5.
- If you try to reassign `x`, the compiler will stop you and let you know that `x` is immutable.

### 2.2. Mutability

If you do want a variable you can change, you must mark it as `mut`:

```rust
fn main() {
    let mut y = 5;
    println!("y is: {}", y);

    y = 10; 
    println!("y changed to: {}", y);
}
```

- `let mut y = 5;` creates a mutable variable named `y`.
- Now Rust knows you’re allowed to change `y`’s value to 10.

**Why so strict?**

- Making variables immutable by default helps prevent accidental changes that can lead to bugs.
- When you really need to change something, you must make it clear by using `mut`.

## 3. Basic Data Types

Rust has a variety of data types to cover most use cases. Here are a few of the main ones:

1. **Integers**

- Examples: `i32`, `u32`, `i64`, `u8`, etc.
- `i` means signed (can be negative), while `u` means unsigned (no negatives).
- The number represents how many bits they take up (e.g., `i32` is a 32-bit integer).
```rust
let integer_example: i32 = -42;
```

2. **Floats**

- Used for decimal numbers like `3.14`.
- Commonly `f32` (32-bit float) or `f64` (64-bit float).
```rust
let pi: f64 = 3.14159;
```

3. **Booleans**

- Can be `true` or `false`.
```rust
let is_rust_fun: bool = true;
```

4. **Characters**

- Stores a single Unicode character, like 'a', 'Z', or even '😎'.
```rust
let emoji: char = '😎';
```

**Why do we care?**

Rust is **statically typed**, so it needs to know at compile time what type of data you’re dealing with to ensure safety and efficiency.

## 4. Simple Control Flow

Control flow is how you decide what happens based on certain conditions. Rust uses the usual suspects: if, else, and loops like while, for, and the special loop.

### 4.1 `if` Statements
```rust
fn main() {
    let number = 5;
    if number > 0 {
        println!("{} is positive", number);
    } else if number < 0 {
        println!("{} is negative", number);
    } else {
        println!("{} is zero", number);
    }
}
```

- `if`, `else if`, and `else` work just like in many other languages.
- You can have as many `else if` branches as you need.

### 4.2 `loop`

1. `loop` runs forever unless you manually `break` out.
```rust
fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 3 {
            break;
        }
    }
}
```

2. `while` – continues running while a condition is true.
```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("Liftoff!");
}
```

3. `for` – great for looping over ranges or collections.
```rust
fn main() {
    for i in 1..5 {
        println!("i is: {}", i);
    }
}
```
- The range `1..5` goes from 1 up to (but not including) 5.
- You can also do `1..=5` to include 5 in the range.

**Why does Rust do it this way?**

- Having clear loop types helps you express your intent.
- Rust checks for mistakes at compile time to prevent memory errors or out-of-bounds access.

## 5. Wrapping Up

In this chapter, you learned how to:

1.  Create and run a simple Rust program.
2.  Use **immutability** to keep your code tidy and safe, or opt for `mut` when you need to change values.
3. Work with **basic data types** (integers, floats, booleans, characters).
4. Write simple **control flow** to make your code do different things based on conditions.

With these essentials, you’re well on your way to writing fun and safe Rust code!

**Next up**: [Chapter 3: Ownership and Borrowing](../03-ownership-and-borrowing/README.md), where we’ll explore Rust’s most magical feature—the ownership system that makes memory safety a breeze.

Extra Resources (Optional)
- [Official Rust Book - Getting Started](https://lborb.github.io/book/official.html)
- [Rust By Example - Basics](https://doc.rust-lang.org/rust-by-example/index.html)

<p align="center"> <sub>© 2025 Learn Rust by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. Licensed under the <a href="../LICENSE">MIT License</a>.</sub> </p>