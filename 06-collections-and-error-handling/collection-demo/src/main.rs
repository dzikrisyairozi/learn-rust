use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};

// Custom error type to demonstrate error handling
#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ParseError(String),
    ValidationError(String),
}

// Implement From trait for convenient error conversion
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IoError(error)
    }
}

// Structure to store student information
#[derive(Debug)]
struct Student {
    name: String,
    grades: Vec<i32>,
}

impl Student {
    // Create a new student with empty grades
    fn new(name: String) -> Student {
        Student {
            name,
            grades: Vec::new(),
        }
    }

    // Add a grade, returning Result in case of invalid grade
    fn add_grade(&mut self, grade: i32) -> Result<(), AppError> {
        if grade < 0 || grade > 100 {
            return Err(AppError::ValidationError(
                "Grade must be between 0 and 100".to_string(),
            ));
        }
        self.grades.push(grade);
        Ok(())
    }

    // Calculate average grade, returning Option since student might have no grades
    fn average_grade(&self) -> Option<f64> {
        if self.grades.is_empty() {
            None
        } else {
            let sum: i32 = self.grades.iter().sum();
            Some(sum as f64 / self.grades.len() as f64)
        }
    }
}

// Function to demonstrate vector operations
fn demonstrate_vectors() -> Result<(), AppError> {
    println!("\n--- Vector Operations ---");
    
    let mut student = Student::new("Alice".to_string());
    
    // Adding grades with error handling
    student.add_grade(85)?;
    student.add_grade(92)?;
    student.add_grade(78)?;
    
    println!("Student grades: {:?}", student.grades);
    
    // Using Option with match
    match student.average_grade() {
        Some(avg) => println!("Average grade: {:.2}", avg),
        None => println!("No grades available"),
    }
    
    Ok(())
}

// Function to demonstrate string operations
fn demonstrate_strings() {
    println!("\n--- String Operations ---");
    
    let mut essay = String::from("Rust is ");
    essay.push_str("a systems ");
    essay.push_str("programming language");
    
    // String manipulation
    let words: Vec<&str> = essay.split_whitespace().collect();
    println!("Word count: {}", words.len());
    println!("Essay: {}", essay);
}

// Function to demonstrate HashMap operations with error handling
fn demonstrate_hashmaps() -> Result<(), AppError> {
    println!("\n--- HashMap Operations ---");
    
    let mut grade_book = HashMap::new();
    
    // Insert some grades
    grade_book.insert("Alice".to_string(), vec![85, 92, 78]);
    grade_book.insert("Bob".to_string(), vec![88, 79, 95]);
    
    // Try to get grades with error handling
    let student_name = "Charlie";
    let grades = grade_book.get(student_name).ok_or_else(|| {
        AppError::ParseError(format!("No grades found for student: {}", student_name))
    })?;
    
    println!("Grade book: {:?}", grade_book);
    Ok(())
}

// Function to demonstrate file operations with error handling
fn demonstrate_file_operations() -> Result<(), AppError> {
    println!("\n--- File Operations ---");
    
    // Writing to file
    let mut file = File::create("grades.txt")?;
    file.write_all(b"Alice: 85, 92, 78\nBob: 88, 79, 95\n")?;
    
    // Reading from file
    let mut content = String::new();
    File::open("grades.txt")?.read_to_string(&mut content)?;
    
    println!("File contents:\n{}", content);
    Ok(())
}

fn main() {
    // Demonstrate various collection operations with error handling
    if let Err(e) = demonstrate_vectors() {
        eprintln!("Vector demonstration failed: {:?}", e);
    }
    
    demonstrate_strings();
    
    if let Err(e) = demonstrate_hashmaps() {
        eprintln!("HashMap demonstration failed: {:?}", e);
    }
    
    if let Err(e) = demonstrate_file_operations() {
        eprintln!("File operations failed: {:?}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_grade_validation() {
        let mut student = Student::new("Test Student".to_string());
        assert!(student.add_grade(85).is_ok());
        assert!(student.add_grade(101).is_err());
        assert!(student.add_grade(-1).is_err());
    }

    #[test]
    fn test_average_grade() {
        let mut student = Student::new("Test Student".to_string());
        assert_eq!(student.average_grade(), None);
        
        student.add_grade(85).unwrap();
        student.add_grade(95).unwrap();
        assert_eq!(student.average_grade(), Some(90.0));
    }
}