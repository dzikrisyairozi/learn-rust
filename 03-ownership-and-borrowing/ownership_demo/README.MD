# Explanation of the Code

## 1. Ownership Transfer
- `s1` owns the `String`.
- `take_ownership(s1)` transfers ownership of the `String` to the function.
- After the transfer, `s1` is no longer valid; attempting to use it will result in a compile-time error.

## 2. Immutable Borrowing
- `s3` is borrowed by `calculate_length(&s3)`.
- Multiple immutable references can coexist.
- The original owner (`s3`) remains valid and cannot be modified through immutable references.

## 3. Mutable Borrowing
- `s4` is declared as mutable.
- A mutable reference `&mut s4` allows modification of the `String`.
- Rust enforces that only one mutable reference exists at a time, preventing data races.

## 4. Multiple Immutable References
- `s5` is borrowed immutably by both `r1` and `r2`.
- Both references can coexist without issues.

## 5. Preventing Simultaneous Mutable and Immutable References
- `s6` first has an immutable reference `r3`.
- Attempting to create a mutable reference `r4` while `r3` exists will cause a compile-time error.
- After `r3` is no longer used, a mutable reference `r4` is created safely.

## 6. Lifetime Annotations
- The `longest` function demonstrates basic lifetime annotations to ensure that the returned reference is valid as long as both input references.

## 7. Unit Tests
- The tests module contains unit tests for each function, ensuring they behave as expected.

## Running the Project
Navigate to the ownership_demo directory and run the project:
```bash
cd ownership_demo
cargo run
```
You should see the following output:
```
s1 owns the string: Hello
Taking ownership of: Hello
s2 now owns the string: Hello
The length of 'Rust' is 4
Before mutation: Hello
After mutation: Hello World!
r1: Borrowing Rules, r2: Borrowing Rules
r3: Concurrency Safety
r4: Concurrency Safety is enforced by Rust.
```

## Running Tests
To ensure everything works as intended, run the tests:
```bash
cargo test
```
You should see output indicating that all tests have passed:
```bash
running 4 tests
test tests::test_take_ownership ... ok
test tests::test_calculate_length ... ok
test tests::test_change ... ok
test tests::test_longest ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Conclusion
This Rust project effectively demonstrates the core concepts of ownership and borrowing:
- Ownership Transfer: Moving ownership of data between variables and functions.
- Immutable Borrowing: Allowing multiple references without permitting mutation.
- Mutable Borrowing: Allowing a single reference to modify data.
- Lifetime Annotations: Ensuring references remain valid.

By exploring and modifying this project, you'll gain a deeper understanding of how Rust manages memory safety and concurrency without a garbage collector.

Feel free to expand upon this example by introducing more complex scenarios, such as struct ownership, enums with ownership semantics, or implementing your own data structures that leverage Rust's ownership principles.

Happy coding!

<p align="center"> <sub>© 2025 Learn Rust by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. Licensed under the <a href="../LICENSE">MIT License</a>.</sub> </p>
