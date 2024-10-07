# Chapter 5: Compound Types

### 1. Introduction

In this chapter, we’ll explore *compound types* in Rust—ways to group multiple pieces of data into a single unit. Specifically, we’ll look at **tuples**, **arrays**, **structs**, and **enums**, along with how to *pattern match* on them.

- **Tuples** let you bundle a fixed number of items of different types.
- **Arrays** let you keep multiple items of the **same** type.
- **Structs** help you define more descriptive data by naming each field.
- **Enums** let you express different "forms" a type can take, each with its own associated data.
- *Pattern matching* helps you elegantly handle these different data forms.

---

### 2. Tuples and Arrays

#### 2.1 Tuples

Tuples are a collection of *fixed size*, where each element can be a different type. You declare them with parentheses:

```rust
fn main() {
    let my_tuple = (500, 6.4, true);
    // Access tuple elements by index
    println!("First value: {}", my_tuple.0);
    println!("Second value: {}", my_tuple.1);
    println!("Third value: {}", my_tuple.2);
}
```

- Each element in a tuple has a **fixed position**.
- You can destructure a tuple using pattern matching, which we’ll explore later.

#### 2.2 Arrays

Arrays are also fixed in size, but all elements must be of the **same** type:

```rust
fn main() {
    let my_array = [1, 2, 3, 4, 5];
    println!("First element: {}", my_array[0]);
    println!("Array length: {}", my_array.len());
}
```

- Arrays are allocated on the stack and are great for data that won’t grow or shrink.
- If you try to access an index that’s out of range, Rust will stop the program to keep you safe from invalid memory access.

---

### 3. Defining and Using Structs

Structs let you name each piece of data, making your code more readable:

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("rustacean123"),
        email: String::from("rustacean@example.com"),
        active: true,
    };

    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Is active? {}", user1.active);
}
```

- **Named fields** make it clear what each piece of data represents.
- Structs are perfect for grouping related information, like data about a user, product, or other entities.

---

### 4. Enums and Pattern Matching

Enums (short for “enumerations”) let you define a type by listing possible *variants*, each of which can hold different data:

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    match home {
        IpAddr::V4(addr) => println!("IPv4 address: {}", addr),
        IpAddr::V6(addr) => println!("IPv6 address: {}", addr),
    }
}
```

- The **match** expression checks which variant you have and can *destructure* any data inside it.
- Enums are a powerful way to handle different “shapes” of data, especially when combined with pattern matching.

---

### 5. Grouping Data Logically and Handling Different Data Forms

- **Grouping Data**: 
  - *Tuples* and *arrays* are handy for small, consistent sets of data.
  - **Structs** help you label fields for clarity.
  - **Enums** handle multiple data “shapes” in one type.

- **Handling Different Forms**:
  - *Pattern matching* (`match`, `if let`, etc.) is key to cleanly handling all possible variants.
  - Rust forces you to consider every variant, which makes your code more robust and less prone to missing edge cases.

---

- By combining **tuples**, **arrays**, **structs**, and **enums** with Rust’s *pattern matching*, you can model almost any data structure.
- When you need quick grouping without names, *tuples* are a go-to.
- If you must store many items of a single type, *arrays* (or **vectors**, covered later) are your friend.
- Use *structs* to create more descriptive data, and *enums* to handle multiple data forms.

### Next Up: [Chapter 6: Collections and Error Handling](../06-collections-and-error-handling/README.md)
Learn how Rust manages collections and error handling.

<p align="center"> <sub>© 2025 Learn Rust by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. Licensed under the <a href="../LICENSE">MIT License</a>.</sub> </p>

