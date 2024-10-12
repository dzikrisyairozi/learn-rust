# Compound Types in Rust Codebase Demo Explanation

## Overview

This chapter demonstrates Rust's compound types code demo through a practical employee management system example. We explore **tuples**, **arrays**, **structs**, **enums**, and their combinations to model real-world data relationships.

## Core Types and Implementations

### 1. Structs

#### Basic Struct Definitions

```rust
#[derive(Debug, Clone)]
struct Employee {
    id: i32,
    name: String,
    role: Role,
    salary: f64,
}

#[derive(Debug)]
struct Department {
    name: String,
    employees: Vec<Employee>,
}

#[derive(Debug)]
struct Project(String, i32);  // Tuple struct
```

Key features:
- `Employee` struct represents individual employees with their attributes
- `Department` struct contains a collection of employees
- `Project` demonstrates a tuple struct with unnamed fields

### 2. Enums

#### Role Definition and Implementation

```rust
#[derive(Debug, Clone)]
enum Role {
    Developer,
    Manager,
    Designer,
}

impl Role {
    fn description(&self) -> &str {
        match self {
            Role::Developer => "Writes and maintains code.",
            Role::Manager => "Oversees team operations.",
            Role::Designer => "Designs user interfaces and experiences.",
        }
    }
}
```

Features:
- Defines distinct role variants
- Implements descriptive functionality
- Supports pattern matching

### 3. Display Implementation

```rust
impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_str = match self {
            Role::Developer => "Developer",
            Role::Manager => "Manager",
            Role::Designer => "Designer",
        };
        write!(f, "{}", role_str)
    }
}
```

Benefits:
- Provides formatted string representation
- Enables direct printing of Role variants
- Enhances readability in output

## Working with Compound Types

### 1. Creating and Managing Employees

```rust
let emp1 = Employee {
    id: 201,
    name: String::from("Bob"),
    role: Role::Developer,
    salary: 85000.0,
};

let emp2 = Employee {
    id: 202,
    name: String::from("Carol"),
    role: Role::Manager,
    salary: 95000.0,
};
```

Operations:
- Instance creation
- Field access
- Data manipulation

### 2. Department Management

```rust
let engineering = Department {
    name: String::from("Engineering"),
    employees: vec![emp1, emp2],
};
```

Features:
- Grouping employees
- Vector-based storage
- Nested struct usage

### 3. Pattern Matching

```rust
match emp.role {
    Role::Developer => println!("{} is a Developer.", emp.name),
    Role::Manager => println!("{} is a Manager.", emp.name),
    Role::Designer => println!("{} is a Designer.", emp.name),
}
```

Capabilities:
- Exhaustive matching
- Role-based logic
- Safe variant handling

## Utility Functions

### 1. Salary Calculations

```rust
fn calculate_average_salary(employees: &Vec<Employee>) -> Option<f64> {
    if employees.is_empty() {
        return None;
    }
    let total: f64 = employees.iter().map(|e| e.salary).sum();
    Some(total / (employees.len() as f64))
}
```

Features:
- Null safety with `Option`
- Iterator usage
- Vector processing

### 2. Information Display

```rust
fn print_department_info(dept: &Department) {
    println!("Department: {}", dept.name);
    for emp in &dept.employees {
        println!(
            " - ID: {}, Name: {}, Role: {:?}, Salary: ${}",
            emp.id, emp.name, emp.role, emp.salary
        );
    }
}
```

Capabilities:
- Formatted output
- Nested data traversal
- Reference-based access

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_calculate_average_salary() {
        let employees = vec![
            Employee {
                id: 501,
                name: String::from("Ivan"),
                role: Role::Developer,
                salary: 70000.0,
            },
            Employee {
                id: 502,
                name: String::from("Judy"),
                role: Role::Designer,
                salary: 75000.0,
            },
        ];
        let avg = calculate_average_salary(&employees);
        assert!(avg.is_some());
        assert_eq!(avg.unwrap(), 72500.0);
    }
}
```

Test coverage:
- Salary calculations
- Department operations
- Role functionality
- Data structure integrity

## Best Practices Demonstrated

### 1. Type Safety
- Strong typing with structs and enums
- Pattern matching for exhaustive handling
- Option type for null safety

### 2. Data Organization
- Logical grouping with structs
- Clear role definitions with enums
- Vector-based collections

### 3. Code Structure
- Clear type definitions
- Implemented traits where needed
- Comprehensive unit tests

## Running the Application

### Run the Project

To run the demonstration project:

```bash
cargo run
```

You will see output similar to this:

```bash
--- Tuples Demonstration ---
Employee Info: ((101, "Alice"), (75000.0, "Engineering"))
ID: 101, Name: Alice, Salary: 75000, Department: Engineering
Project: Project("Apollo", 12)
Project Name: Apollo
Duration: 12 months

--- Arrays Demonstration ---
Matrix: [[1, 2, 3], [4, 5, 6]]
Element at [0][2]: 3
Iterating over the matrix:
1 2 3 
4 5 6 

--- Structs Demonstration ---
Employee 1: Employee { id: 201, name: "Bob", role: Developer, salary: 85000.0 }
Employee 2: Employee { id: 202, name: "Carol", role: Manager, salary: 95000.0 }
Department: Engineering
 - ID: 201, Name: Bob, Role: Developer, Salary: $85000
 - ID: 202, Name: Carol, Role: Manager, Salary: $95000

--- Enums Demonstration ---
Role: Developer
Role: Manager
Developer: Writes and maintains code.
Manager: Oversees team operations.

--- Pattern Matching Demonstration ---
Dave is a Manager.
Department Option contains: Human Resources
Department: Human Resources
 - ID: 302, Name: Eve, Role: Designer, Salary: $90000

--- Combining Compound Types ---
Department: Engineering
 - ID: 401, Name: Frank, Role: Developer, Salary: $80000
 - ID: 402, Name: Grace, Role: Developer, Salary: $82000
Department: Human Resources
 - ID: 403, Name: Heidi, Role: Manager, Salary: $90000
Average Salary across all departments: $84000
```

### Run Tests

To run the test suite:

```bash
cargo test
```

You will see output like this:

```bash
running 3 tests
test tests::test_calculate_average_salary ... ok
test tests::test_print_department_info ... ok
test tests::test_role_description ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Test coverage includes:
- `test_calculate_average_salary`: Validates salary calculation logic
- `test_print_department_info`: Ensures department information printing works
- `test_role_description`: Verifies role descriptions are correct

## Conclusion

This demonstration showcases Rust's compound types through a practical example, highlighting:
- Struct and enum usage for data modeling
- Implementation of useful traits
- Pattern matching and data manipulation
- Safe and efficient data organization

The example provides a foundation for building larger systems using Rust's type system effectively.