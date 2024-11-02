# Chapter 7: Modules and Crates

### 1. Introduction

As your Rust projects grow larger, organizing code and managing dependencies become crucial. In this chapter, we’ll look at:

- *Modules* – how to break your code into logical sections.
- *Crates* – how Rust packages code into libraries or executables.
- *Cargo.toml* – how to manage dependencies and metadata.
- *Best practices* for modular code and external libraries.

---

### 2. Organizing Code Using Modules

A **module** in Rust is like a folder for related functions, structs, or enums. It helps keep your code clear and organized.

```rust
// src/main.rs
mod greetings;

fn main() {
    greetings::hello();
}
```

```rust
// src/greetings.rs
pub fn hello() {
    println!("Hello from the greetings module!");
}
```

- **mod greetings** tells Rust we have a module named *greetings*.
- We define the module’s code in a separate file called *greetings.rs*.
- The **pub** keyword makes the function publicly accessible outside its module.

#### Nested Modules

You can nest modules inside one another for more structure:

```rust
mod library {
    pub mod books {
        pub fn list_books() {
            println!("Listing books...");
        }
    }
}

fn main() {
    library::books::list_books();
}
```

- If you find your file getting too big, you can split nested modules into more files.

---

### 3. Creating a Library Crate vs. a Binary Crate

A **crate** is the smallest compilation unit in Rust. It can be:

- **Binary crate**: Produces an *executable* (like a CLI tool).  
- **Library crate**: Produces *shared code* you can reuse in other crates.

```sh
# Create a binary crate
cargo new my_binary

# Create a library crate (use --lib)
cargo new my_library --lib
```

- In a **binary crate**, *src/main.rs* is the entry point (`fn main()`).
- In a **library crate**, *src/lib.rs* defines the library’s public API.

---

### 4. Managing Dependencies with Cargo and Understanding Cargo.toml

When you create a new Rust project, **Cargo** generates a **Cargo.toml** file, which contains:

- **Metadata** (package name, version, authors).
- **Dependencies** (external crates from [crates.io](https://crates.io/)).
- Build configuration.

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
```

- Add a dependency by specifying its name and version under **[dependencies]**.
- To use it in your code:  
  `use serde::Serialize;`

#### Local and Git Dependencies

You can also use local paths or Git repositories:

```toml
[dependencies]
my_local_crate = { path = "../my_local_crate" }
my_github_crate = { git = "https://github.com/username/my_repo.git" }
```

- **path** points to a local directory where the crate is located.
- **git** fetches the crate directly from a Git repository.

---

### 5. Best Practices for Modular Code and External Libraries

- **Keep modules focused**: Group related items together; don’t cram everything in one file.
- **Use `pub` selectively**: Only expose what other parts of your code (or external users) need to see.
- **Document your modules**: Adding comments and docstrings (using `///`) helps both you and others understand your codebase.
- **Rely on Cargo**: Let Cargo handle your dependencies. Pin down versions to avoid unexpected breaking changes (`"^1.2.3"` or `"=1.2.3"`).
- **Publish to crates.io**: If you create a useful library crate, consider publishing it so the broader Rust community can benefit.

---

By effectively using **modules** and **crates**, you’ll keep your Rust projects tidy, scalable, and easy to collaborate on. Next, we’ll dive into more **advanced topics** to broaden your Rust skills even further.

### Next Up: [Chapter 8: Advanced Topics](../08-advanced-topics/README.md)
Learn more advanced topics in Rust.

<p align="center"> <sub>© 2025 Learn Rust by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. Licensed under the <a href="../LICENSE">MIT License</a>.</sub> </p>


