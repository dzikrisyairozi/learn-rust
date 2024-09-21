// This program demonstrates Rust's ownership and borrowing concepts.
// It includes examples of ownership transfer, immutable borrowing,
// mutable borrowing, and lifetime annotations.

fn main() {
    // Ownership Transfer
    let s1 = String::from("Hello");
    println!("s1 owns the string: {}", s1);

    let s2 = take_ownership(s1);
    // s1 is no longer valid here
    println!("s2 now owns the string: {}", s2);

    // s1 cannot be used after transfer
    // Uncommenting the next line will cause a compile-time error
    // println!("s1 is still valid: {}", s1);

    // Borrowing (Immutable References)
    let s3 = String::from("Rust");
    let len = calculate_length(&s3);
    println!("The length of '{}' is {}", s3, len);
    // s3 is still valid after borrowing

    // Borrowing (Mutable References)
    let mut s4 = String::from("Hello");
    println!("Before mutation: {}", s4);
    change(&mut s4);
    println!("After mutation: {}", s4);

    // Demonstrating multiple immutable references
    let s5 = String::from("Borrowing Rules");
    let r1 = &s5;
    let r2 = &s5;
    println!("r1: {}, r2: {}", r1, r2);
    // Multiple immutable references are allowed

    // Demonstrating no simultaneous mutable and immutable references
    let mut s6 = String::from("Concurrency Safety");
    let r3 = &s6;
    // let r4 = &mut s6; // ❌ Cannot borrow as mutable while immutable references exist
    println!("r3: {}", r3);
    // To create a mutable reference, ensure no immutable references are active
    let r4 = &mut s6;
    r4.push_str(" is enforced by Rust.");
    println!("r4: {}", r4);
}

// Function that takes ownership of a String and returns it
fn take_ownership(s: String) -> String {
    println!("Taking ownership of: {}", s);
    s
}

// Function that takes an immutable reference to a String and returns its length
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Function that takes a mutable reference to a String and modifies it
fn change(s: &mut String) {
    s.push_str(" World!");
}

// Demonstrating lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_ownership() {
        let s = String::from("Ownership Test");
        let s = take_ownership(s);
        assert_eq!(s, "Ownership Test");
    }

    #[test]
    fn test_calculate_length() {
        let s = String::from("Length Test");
        let len = calculate_length(&s);
        assert_eq!(len, 11);
    }

    #[test]
    fn test_change() {
        let mut s = String::from("Mutable");
        change(&mut s);
        assert_eq!(s, "Mutable World!");
    }

    #[test]
    fn test_longest() {
        let string1 = String::from("short");
        let string2 = "longer string";
        let result = longest(&string1, string2);
        assert_eq!(result, "longer string");
    }
}