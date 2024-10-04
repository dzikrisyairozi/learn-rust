# 04. Functions and Control Flow
In this chapter, we’ll cover:

1. **Defining & Calling Functions**
2. **Expressions vs. Statements**
3. **Advanced Control Flow**
   - Nested `if` statements
   - Loops: `loop`, `while`, `for`
   - `match` expressions
4. **Practical Examples & Real-World Analogies**

By the end, you’ll be more comfortable structuring your Rust code and making decisions (branching) in clear, safe ways.

## 1. Defining & Calling Functions

### 1.1 Basic Function Definition
Functions in Rust are declared using the `fn` keyword, followed by a name and parentheses for parameters:

```rust
fn main() {
    greet_user("Dzikri");
}

fn greet_user(name: &str) {
    println!("Hello, {}!", name);
}

```

- `fn greet_user(name: &str)` declares a function with one parameter `name`, which is a string slice (`&str`).
- You call the function in `main` with `greet_user("Dzikri")`.

#### Why organize code into functions?

- Functions allow you to break down your logic into small, reusable pieces.
- This makes code easier to read, test, and maintain—no more spaghetti code!

### 1.2 Returning Values

Functions can return values using the `->` syntax.

```rust
fn main() {
    let sum = add_numbers(5, 7);
    println!("Sum is: {}", sum);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b  // Returns the result without a semicolon
}

```

- If you leave off the semicolon in the last line, that final expression is returned.
- If you **do** use a semicolon, it becomes a statement, so you’d need an explicit `return`.

## 2. Expressions vs. Statements
Understanding expressions and statements is key to writing clean Rust code:

- **Expression**: Produces a value.
    - Examples: `2 + 2`, `a + b`, `5`, `"hello"`.

- **Statement**: Performs an action but does not return a value.
    - Example: `let x = 5;` (this assigns a value to `x`, but the statement itself doesn’t produce a value).

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1  // This line is an expression
    }; 

    // y is now 4 (3 + 1)
    println!("The value of y is: {}", y);
}

```

- The code block after `let y = { ... }` is a statement containing an expression (`x + 1`).
- That expression becomes the value assigned to `y`.

## 3. Advanced Control Flow
### 3.1 Nested `if` Statements
You already know `if-else`, but let’s look at a nested example:

```rust
fn main() {
    let temperature = 30;

    if temperature > 35 {
        if temperature > 40 {
            println!("It's extremely hot!");
        } else {
            println!("It's hot!");
        }
    } else {
        println!("Not too hot.");
    }
}
```

- Nested `if` statements allow you to get more granular in your decision-making.
- Be careful not to over-nest, or your code might become hard to read.

## 3.2 Loops: `loop`, `while`, `for`

### 3.2.1 `loop`
`loop` runs forever unless you explicitly break out of it.

```rust
fn main() {
    let mut count = 0;

    loop {
        count += 1;
        if count == 3 {
            println!("Hit count 3, breaking out!");
            break;
        }
    }
}
```

### 3.2.2 `while`
Runs as long as a condition is `true`.

```rust
fn main() {
    let mut number = 3;

    while number > 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("Liftoff!");
}

```

### 3.2.3 `for`
Great for looping over a range or a collection:

```rust
fn main() {
    for i in 1..4 {
        println!("i is: {}", i);
    }
    // 1..4 goes 1, 2, 3
}
```
- Ranges: `1..4` excludes 4, `1..=4` includes 4.
- You can also iterate over arrays, vectors, or other iterable types:

```rust

let fruits = ["apple", "banana", "cherry"];
for fruit in fruits.iter() {
    println!("{}", fruit);
}
```

- `fruits.iter()` returns an iterator over the elements of `fruits`.
- `for fruit in fruits.iter()` iterates over each element, printing it.

### 3.3 `match` Expressions
`match` is a powerful branching construct that lets you compare a value against a series of patterns:

```rust
fn main() {
    let day = 3;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        _ => println!("Some other day"),
    }
}
```

- `match day { ... }` checks `day` against each pattern (1, 2, 3, etc.).
- The `_` pattern (called a “catch-all”) matches any value that doesn’t fit previous patterns.

Why `match` is cool:

- It forces you to handle every possible case, which can prevent logical oversights.
- You can match complex patterns, like enums or even de-structured types.

## 4. Practical Examples & Analogies
### 4.1 The “Function as a Recipe” Analogy
Imagine a **cookbook recipe**:

- **Parameters** = ingredients.
- **Instructions** (function body) = steps to make the dish.
- **Return value** = the final dish you serve.

This structure helps keep your “kitchen” (code) organized, so you don’t mix up processes and end up with a mess.

### 4.2 The “Traffic Light” Match Analogy
Say you have a traffic light: `Red`, `Yellow`, or `Green` (three states). A match expression is like:

- `Red` => “Stop the car.”
- `Yellow` => “Slow down, prepare to stop.”
- `Green` => “Go.”
- `_` => “We didn’t recognize that color.”

This ensures you handle every possible light color, avoiding confusion on the road.

## 5. Wrapping Up
In this chapter, you learned:

1. How to **define and call functions** in Rust.
2. The difference between **expressions** (produce values) and **statements** (perform actions).
3. How to manage **advanced control flow** with nested `if` statements, the different kinds of loops, and the versatile `match` expression.

Next, you’ll dive into **compound types** like structs, enums, and tuples—giving you powerful ways to organize data.

### Next Up: [Chapter 5: Compound Types](../05-compound-types/README.md)
Learn how Rust groups related pieces of data and how you can use pattern matching on enums for flexible code design.

<p align="center"> <sub>© 2025 Learn Rust by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. Licensed under the <a href="../LICENSE">MIT License</a>.</sub> </p>

