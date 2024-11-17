# Chapter 8: Advanced Topics

### 1. Introduction

You’ve come a long way in your Rust journey! Now let’s explore some of the **advanced features** that make Rust both powerful and flexible:

- **Traits** and **Generics**: For abstraction and code reuse.
- **Closures** and **Iterators**: Write concise, functional-style code.
- **Basic Concurrency**: Using threads, channels.
- **Async Programming & Smart Pointers**: Future-based concurrency, `Box`, `Rc`, `Arc`.
- **Performance** and **Best Practices**: Fine-tuning and advanced tips.

---

### 2. Traits and Generics

#### 2.1 Traits

A *trait* in Rust defines *shared behavior* that types can implement. It’s somewhat like an interface:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

fn main() {
    let article = Article {
        title: String::from("Rust in Action"),
        author: String::from("Ferris Crab"),
    };
    println!("{}", article.summarize());
}
```

- **Traits** let you define a contract, and types that implement the trait must provide the required methods.
- This allows polymorphism: different types can be treated the same way if they implement the same trait.

#### 2.2 Generics

Generics enable code to work with multiple data types:

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    let numbers = vec![10, 40, 30];
    let chars = vec!['a', 'x', 'm'];

    println!("Largest number: {}", largest(&numbers));
    println!("Largest char: {}", largest(&chars));
}
```

- `T` is a *generic type parameter*.
- The `PartialOrd` + `Copy` traits set constraints: T must be comparable and copyable.
- Generics prevent code duplication by letting you write once and reuse with many types.

---

### 3. Closures and Iterators

#### 3.1 Closures

Closures are *anonymous functions* you can store in variables or pass as arguments:

```rust
fn main() {
    let add_one = |x: i32| x + 1;
    println!("4 + 1 = {}", add_one(4));
}
```

- Closures can capture variables from their environment, making them more flexible than normal functions.
- They’re handy for functional-style operations like map, filter, reduce, etc.

#### 3.2 Iterators

An *iterator* lets you process a sequence of items one at a time:

```rust
fn main() {
    let nums = vec![1, 2, 3];
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
}
```

- `nums.iter()` returns an iterator over references to elements in `nums`.
- `.map(|x| x * 2)` applies the closure to each element.
- `.collect()` turns the transformed iterator back into a vector.

---

### 4. Basic Concurrency: Threads and Channels

Rust’s **thread** API lets you run code in parallel:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 1..5 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(50));
    }

    handle.join().unwrap();
}
```

- `thread::spawn` creates a new thread, running the closure in parallel to the main thread.
- `join()` waits for the spawned thread to finish.

Channels facilitate *communication* between threads:

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("Hello from another thread").unwrap();
    });

    println!("{}", rx.recv().unwrap());
}
```

- `mpsc::channel()` gives you a **transmitter (tx)** and a **receiver (rx)**.
- `tx.send(data)` sends data, while `rx.recv()` waits for data.
- This is how you safely pass data across threads in Rust without data races.

---

### 5. Introduction to Async Programming and Smart Pointers

#### 5.1 Async Programming

Rust’s asynchronous programming model uses `async` and `await` keywords:

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    hello_async().await;
}

async fn hello_async() {
    println!("Hello from async!");
    sleep(Duration::from_secs(1)).await;
    println!("Done sleeping!");
}
```

- Async code in Rust is typically powered by executors like **Tokio** or **async-std**.
- `async fn` creates a function that returns a **future**, and `.await` yields control until the future completes.

#### 5.2 Smart Pointers: Box, Rc, Arc

- **Box**: Stores data on the heap rather than the stack, useful for recursive data or large structs.
  
```rust
fn main() {
    let boxed_number = Box::new(5);
    println!("Boxed number: {}", boxed_number);
}
```

- **Rc**: A reference-counted pointer for shared ownership in single-threaded contexts.
- **Arc**: Like `Rc` but safe to share across multiple threads (`Arc` = Atomic Reference Counted).

These pointers extend Rust’s ownership to more complex patterns without losing safety.

---

### 6. Performance Considerations and Advanced Best Practices

- **Zero-cost abstractions**: Rust ensures features like generics, iterators, and traits don’t add significant overhead.
- **Inlined code**: The compiler aggressively optimizes, often inlining small functions or closures.
- **Avoid unnecessary allocations**: Use **slices** where possible, keep data on the stack if feasible, and minimize heap usage.
- **Use profiling tools**: Tools like `perf` (Linux) or `Instruments` (macOS) help you identify bottlenecks.
- **Unsafe code**: Rust lets you write `unsafe` code when you need to bypass compiler checks (e.g., FFI). Use it **sparingly** and document thoroughly.

---

By mastering these **advanced topics**—traits, generics, closures, iterators, concurrency, async, and smart pointers—you’ll be able to build highly efficient, scalable, and safe applications. Rust’s ecosystem offers **zero-cost abstractions** that let you write high-level code without sacrificing performance.

You’ve reached the end of this learning repository’s journey. Now you’re ready to explore real-world Rust projects, libraries, and more in-depth resources. Happy coding in Rust!

<p align="center"> <sub>© 2025 Learn Rust by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. Licensed under the <a href="../LICENSE">MIT License</a>.</sub> </p>