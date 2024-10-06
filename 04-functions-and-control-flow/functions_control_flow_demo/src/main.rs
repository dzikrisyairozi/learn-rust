// This program demonstrates Rust's functions and control flow constructs.
// It includes examples of defining and calling functions, expressions vs. statements,
// various control flow mechanisms like if statements, loops, and match expressions.

fn main() {
    // 1. Defining and Calling Functions
    greet_user("Dzikri");
    let sum = add_numbers(5, 7);
    println!("Sum is: {}", sum);

    // 2. Expressions vs. Statements
    let y = {
        let x = 3;
        x + 1 // This is an expression whose value is returned
    };
    println!("The value of y is: {}", y);

    // 3. Advanced Control Flow
    // 3.1 Nested if Statements
    let temperature = 30;
    check_temperature(temperature);

    // 3.2 Loops: loop, while, for
    demonstrate_loops();

    // 3.3 match Expressions
    let day = 3;
    match_day(day);

    // 4. Practical Example Combining Functions and Control Flow
    let number = 10;
    if is_even(number) {
        println!("{} is even.", number);
    } else {
        println!("{} is odd.", number);
    }

    // Using a for loop to iterate over a range
    for i in 1..=5 {
        println!("Counting: {}", i);
    }
}

// Function that takes a string slice and greets the user
fn greet_user(name: &str) {
    println!("Hello, {}!", name);
}

// Function that adds two integers and returns the sum
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b // Implicit return without a semicolon
}

// Function to check temperature using nested if statements
fn check_temperature(temp: i32) {
    if temp > 35 {
        if temp > 40 {
            println!("It's extremely hot!");
        } else {
            println!("It's hot!");
        }
    } else {
        println!("Not too hot.");
    }
}

// Function to demonstrate different types of loops
fn demonstrate_loops() {
    // loop
    let mut count = 0;
    println!("Demonstrating 'loop':");
    loop {
        count += 1;
        println!("Count: {}", count);
        if count == 3 {
            println!("Hit count 3, breaking out of the loop!");
            break;
        }
    }

    // while loop
    let mut number = 3;
    println!("\nDemonstrating 'while' loop:");
    while number > 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!");

    // for loop over a range
    println!("\nDemonstrating 'for' loop:");
    for i in 1..4 {
        println!("i is: {}", i);
    }
}

// Function to demonstrate match expressions
fn match_day(day: u32) {
    println!("\nDemonstrating 'match' expression:");
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day"),
    }
}

// Function to check if a number is even
fn is_even(num: i32) -> bool {
    num % 2 == 0 // Returns true if num is even, false otherwise
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
        assert_eq!(add_numbers(-1, 1), 0);
    }

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(7), false);
    }

    #[test]
    fn test_greet_user() {
        // Since greet_user prints to the console, we can at least ensure it doesn't panic
        greet_user("Test User");
    }
}