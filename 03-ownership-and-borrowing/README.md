# 03. Ownership and Borrowing

In this chapter, we’ll explore:

1. **Why Ownership Exists**
2. **The Rules of Ownership**
3. **Borrowing & References (Immutable vs. Mutable)**
4. **Lifetimes**
5. **Real-World Analogies**
6. **Wrapping Up**

By the end, you’ll see how Rust’s unique approach prevents many of the memory bugs common in lower-level languages—without needing a garbage collector.

## 1. Why Does Rust Have an Ownership System?
In many programming languages, you rely on a **garbage collector** or manual memory management (`malloc`, `free`, etc.). Rust’s designers took a different path to solve memory and concurrency problems at compile time using a concept called **ownership**.

- **No Garbage Collector**

  Rust doesn’t have a runtime that periodically cleans up unused memory. Instead, memory is freed as soon as it’s no longer needed.
- **Catch Bugs Early**

  Rust’s compiler checks ownership and borrowing rules before your program even runs, catching many potential issues that cause crashes or weird behavior in other languages.

Think of it like having a neat and organized closet. Instead of a cleaning service that occasionally comes by to tidy up (garbage collector), you (the Rust compiler) ensure each item is put away at the exact moment you no longer need it.

## 2. The Rules of Ownership

### Rule 1: Each Value Has a Single Owner

Whenever you create a piece of data, some variable owns it. For example:

```rust
let s1 = String::from("hello");
let s2 = s1; // Ownership moves from s1 to s2
```

Now, `s1` no longer owns the string; `s2` does. If you try to use `s1` after that, the compiler won’t let you because its ownership has moved.

### Rule 2: When the Owner Goes Out of Scope, the Value Is Dropped
```rust
{
    let s3 = String::from("world");
    // s3 is valid here
}
// s3 is no longer valid here, it's automatically "dropped"
```

## 3. Borrowing & References
Transferring ownership is powerful but sometimes you just want to **look at** or **read** data without taking it over. That’s where **borrowing** comes in.

### 3.1 Immutable References
```rust
fn main() {
    let s1 = String::from("hello");
    let length = calculate_length(&s1); 
    // Pass an immutable reference to s1

    println!("'{}' has length {}", s1, length);
}

fn calculate_length(s: &String) -> usize { 
    // s is a reference to a String, not the owner
    s.len()
}
```

- `&s1` means “I’m lending you s1,” so `calculate_length` can inspect the data but can’t change it.
- `s1` keeps its ownership, so you can still use it after the function call.

**Why immutable references?**
- You don’t want (or need) the function to change the original data.
- Multiple immutable references can exist at the same time, which is safe because none can alter the data.

### 3.2 Mutable References
If you really need a function (or another part of your code) to modify the data, you can lend it mutably:

```rust
fn main() {
    let mut greeting = String::from("Hi");
    change_greeting(&mut greeting);

    println!("New greeting: {}", greeting);
}

fn change_greeting(g: &mut String) {
    g.push_str(", Rustaceans!");
}
```

## 4. Lifetimes
References have to remain valid. **Lifetimes** guarantee that references don’t outlive the data they point to:

- When you borrow data, the reference’s lifetime **cannot** exceed the owner’s lifetime.
- Often, the compiler figures out lifetimes for you automatically.
- When it can’t, you add **lifetime annotations** to clarify how references relate to each other.

Don’t stress too much about lifetimes initially—they’ll make more sense once you dive deeper. Just know that **Rust checks** references at compile time to ensure no one’s left holding a reference to data that’s already been dropped.

## 5. Real-World Analogies

### 5.1 The Library Book Analogy
- **Ownership**: You own a book. If you give it away to a friend, you can’t read it anymore because they now own it.
- **Borrowing (Immutable)**: You let your friend read the book at your house, but they can’t write in it. You still own the book; they’re just checking out its content.
- **Borrowing (Mutable)**: You let your friend take the book home, and they can write in it. Only one person can do this at a time (one mutable reference), so the book doesn’t get conflicting changes from multiple people.

### 5.2 Scope and Cleanup
- Think of **scope** like **renting a hotel room**. You “own” the room for the duration of your stay. Once your check-out date arrives (end of the scope), you **automatically** lose access and the room is cleaned for the next guest—no messy manual cleanup on your part.   

### 5.3 Lifetimes
- Imagine you borrowed a friend’s phone (a reference). You can only use it while your friend is there and willing to let you borrow it. If they leave (the friend’s scope ends), you have to hand the phone back. You can’t keep using a phone (reference) that’s gone.

## 6. Wrapping Up
You’ve taken a deep dive into Rust’s ownership model and how borrowing and lifetimes work. These concepts are core to writing safe, efficient Rust code without a garbage collector.

**Key Takeaways**
- **Ownership** guarantees one owner at a time, and memory is freed when the owner goes out of scope.
- **Borrowing** lets you inspect or modify data without taking ownership, with strict rules to keep things safe.
- **Lifetimes** ensure references don’t outlive the data they point to.

Learning ownership and borrowing can feel tricky at first, but once it clicks, you’ll appreciate how it makes your code safer and more predictable. Next, we’ll jump into **functions** and **control flow** to level up your Rust skills!

### Next Up: [Chapter 4: Functions and Control Flow](../04-functions-and-control-flow/README.md)
Check it out to see how Rust organizes code into functions, how it handles statements vs. expressions, and more interesting ways to control program flow.

<p align="center"> <sub>© 2025 Learn Rust by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. Licensed under the <a href="../LICENSE">MIT License</a>.</sub> </p>
