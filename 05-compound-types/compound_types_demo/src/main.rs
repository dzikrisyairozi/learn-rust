// This program provides an advanced demonstration of Rust's compound types,
// including tuples, arrays, structs, enums, and pattern matching.
// All functionalities are contained within a single file for simplicity.

// Importing necessary standard library features
use std::fmt;

// ----------------------------
// Struct Definitions
// ----------------------------

// The `Employee` struct represents an employee with various attributes.
#[derive(Debug, Clone)]
struct Employee {
    id: i32,
    name: String,
    role: Role,
    salary: f64,
}

// The `Department` struct represents a department containing a list of employees.
#[derive(Debug)]
struct Department {
    name: String,
    employees: Vec<Employee>,
}

// The `Project` struct is a tuple struct representing a project name and its duration in months.
#[derive(Debug)]
struct Project(String, i32);

// ----------------------------
// Enum Definitions
// ----------------------------

// The `Role` enum defines different roles an employee can have.
#[derive(Debug, Clone)]
enum Role {
    Developer,
    Manager,
    Designer,
}

// Implementing methods for `Role` to provide descriptive text.
impl Role {
    // Returns a description of the role.
    fn description(&self) -> &str {
        match self {
            Role::Developer => "Writes and maintains code.",
            Role::Manager => "Oversees team operations.",
            Role::Designer => "Designs user interfaces and experiences.",
        }
    }
}

// Implementing the Display trait for `Role` to allow formatted printing.
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

// ----------------------------
// Helper Functions
// ----------------------------

/// Calculates the average salary of a list of employees.
/// Returns `None` if the list is empty.
fn calculate_average_salary(employees: &Vec<Employee>) -> Option<f64> {
    if employees.is_empty() {
        return None;
    }
    let total: f64 = employees.iter().map(|e| e.salary).sum();
    Some(total / (employees.len() as f64))
}

/// Prints detailed information about a department, including its employees.
fn print_department_info(dept: &Department) {
    println!("Department: {}", dept.name);
    for emp in &dept.employees {
        println!(
            " - ID: {}, Name: {}, Role: {}, Salary: ${}",
            emp.id, emp.name, emp.role, emp.salary
        );
    }
}

/// Prints detailed information about a project.
fn print_project_info(project: &Project) {
    println!("Project Name: {}", project.0);
    println!("Duration: {} months", project.1);
}

// ----------------------------
// Demonstration Functions
// ----------------------------

/// Demonstrates the use of tuples, including nested tuples and tuple structs.
fn demonstrate_tuples() {
    println!("--- Tuples Demonstration ---");

    // Creating a nested tuple containing employee information.
    let employee_info: ((i32, &str), (f64, &str)) = ((101, "Alice"), (75000.0, "Engineering"));
    println!("Employee Info: {:?}", employee_info);

    // Destructuring the nested tuple to access individual elements.
    let ((id, name), (salary, department)) = employee_info;
    println!("ID: {}, Name: {}, Salary: {}, Department: {}", id, name, salary, department);

    // Creating and printing a tuple struct.
    let project = Project(String::from("Apollo"), 12);
    println!("Project: {:?}", project);
    print_project_info(&project);
}

/// Demonstrates the use of arrays, including multi-dimensional arrays and iteration.
fn demonstrate_arrays() {
    println!("\n--- Arrays Demonstration ---");

    // Creating a multi-dimensional array (matrix).
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    println!("Matrix: {:?}", matrix);

    // Accessing a specific element in the array.
    println!("Element at [0][2]: {}", matrix[0][2]);

    // Iterating over the multi-dimensional array and printing elements.
    println!("Iterating over the matrix:");
    for row in &matrix {
        for &val in row {
            print!("{} ", val);
        }
        println!();
    }
}

/// Demonstrates the use of structs, including creating instances and manipulating data.
fn demonstrate_structs() {
    println!("\n--- Structs Demonstration ---");

    // Creating instances of the `Employee` struct.
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

    // Printing the created employees.
    println!("Employee 1: {:?}", emp1);
    println!("Employee 2: {:?}", emp2);

    // Creating an instance of the `Department` struct containing employees.
    let engineering = Department {
        name: String::from("Engineering"),
        employees: vec![emp1, emp2],
    };

    // Printing department information.
    print_department_info(&engineering);
}

/// Demonstrates the use of enums and pattern matching.
fn demonstrate_enums() {
    println!("\n--- Enums Demonstration ---");

    // Creating instances of the `Role` enum.
    let role1 = Role::Developer;
    let role2 = Role::Manager;

    // Matching on the first role.
    match role1 {
        Role::Developer => println!("Role: Developer"),
        Role::Manager => println!("Role: Manager"),
        Role::Designer => println!("Role: Designer"),
    }

    // Matching on the second role.
    match role2 {
        Role::Developer => println!("Role: Developer"),
        Role::Manager => println!("Role: Manager"),
        Role::Designer => println!("Role: Designer"),
    }

    // Printing role descriptions.
    println!("{}: {}", role1, role1.description());
    println!("{}: {}", role2, role2.description());
}

/// Demonstrates pattern matching with structs and enums, including `if let`.
fn demonstrate_pattern_matching() {
    println!("\n--- Pattern Matching Demonstration ---");

    // Creating an instance of the `Employee` struct.
    let emp = Employee {
        id: 301,
        name: String::from("Dave"),
        role: Role::Manager,
        salary: 105000.0,
    };

    // Matching on the employee's role to print appropriate messages.
    match emp.role {
        Role::Developer => println!("{} is a Developer.", emp.name),
        Role::Manager => println!("{} is a Manager.", emp.name),
        Role::Designer => println!("{} is a Designer.", emp.name),
    }

    // Using `if let` to handle an `Option<Department>`.
    let department_option: Option<Department> = Some(Department {
        name: String::from("Human Resources"),
        employees: vec![
            Employee {
                id: 302,
                name: String::from("Eve"),
                role: Role::Designer,
                salary: 90000.0,
            },
        ],
    });

    if let Some(dept) = department_option {
        println!("Department Option contains: {}", dept.name);
        print_department_info(&dept);
    } else {
        println!("No department found.");
    }
}

/// Demonstrates combining various compound types to model complex data structures.
fn combine_compound_types() {
    println!("\n--- Combining Compound Types ---");

    // Creating a vector of departments, each containing employees.
    let departments = vec![
        Department {
            name: String::from("Engineering"),
            employees: vec![
                Employee {
                    id: 401,
                    name: String::from("Frank"),
                    role: Role::Developer,
                    salary: 80000.0,
                },
                Employee {
                    id: 402,
                    name: String::from("Grace"),
                    role: Role::Developer,
                    salary: 82000.0,
                },
            ],
        },
        Department {
            name: String::from("Human Resources"),
            employees: vec![
                Employee {
                    id: 403,
                    name: String::from("Heidi"),
                    role: Role::Manager,
                    salary: 90000.0,
                },
            ],
        },
    ];

    // Iterating over each department and printing its information.
    for dept in &departments {
        print_department_info(dept);
    }

    // Calculating and printing the average salary for all employees.
    let all_employees: Vec<Employee> = departments.iter()
        .flat_map(|dept| dept.employees.clone())
        .collect();
    
    if let Some(avg_salary) = calculate_average_salary(&all_employees) {
        println!("Average Salary across all departments: ${}", avg_salary);
    } else {
        println!("No employees to calculate average salary.");
    }
}

fn main() {
    // Demonstrating Tuples
    demonstrate_tuples();

    // Demonstrating Arrays
    demonstrate_arrays();

    // Demonstrating Structs
    demonstrate_structs();

    // Demonstrating Enums
    demonstrate_enums();

    // Demonstrating Pattern Matching
    demonstrate_pattern_matching();

    // Demonstrating Combination of Compound Types
    combine_compound_types();
}

// Implementing Clone for Department to allow cloning in pattern matching.
impl Clone for Department {
    fn clone(&self) -> Self {
        Department {
            name: self.name.clone(),
            employees: self.employees.clone(),
        }
    }
}

// ----------------------------
// Unit Tests
// ----------------------------

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_print_department_info() {
        let emp = Employee {
            id: 601,
            name: String::from("Karl"),
            role: Role::Manager,
            salary: 95000.0,
        };
        let dept = Department {
            name: String::from("Marketing"),
            employees: vec![emp],
        };
        // This test ensures that the function runs without panicking.
        print_department_info(&dept);
    }

    #[test]
    fn test_role_description() {
        let role_dev = Role::Developer;
        let role_mgr = Role::Manager;
        let role_des = Role::Designer;

        assert_eq!(role_dev.description(), "Writes and maintains code.");
        assert_eq!(role_mgr.description(), "Oversees team operations.");
        assert_eq!(role_des.description(), "Designs user interfaces and experiences.");
    }
}