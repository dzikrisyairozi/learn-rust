## Run the Project

```bash
cargo run
```

You will see the output like this:

```bash
Hello, Dzikri!
Sum is: 12
The value of y is: 4
Demonstrating 'if' statements:
Not too hot.

Demonstrating 'loop':
Count: 1
Count: 2
Count: 3
Hit count 3, breaking out of the loop!

Demonstrating 'while' loop:
3!
2!
1!
Liftoff!

Demonstrating 'for' loop:
i is: 1
i is: 2
i is: 3

Demonstrating 'match' expression:
Wednesday
10 is even.
Counting: 1
Counting: 2
Counting: 3
Counting: 4
Counting: 5
```

## Run Test

Purpose: Validate that functions behave as expected.
Tests Included:
- `test_add_numbers`: Checks if the `add_numbers` function correctly adds two integers.
- `test_is_even`: Ensures that `is_even` accurately identifies even and odd numbers.
- `test_greet_user`: Verifies that `greet_user` executes without panicking.

```bash
cargo test
```

You will see the output like this:

```bash
running 3 tests
test tests::test_add_numbers ... ok
test tests::test_greet_user ... ok
test tests::test_is_even ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
