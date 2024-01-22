//! Reads employee data from a JSON file, categorizes employees based on their position
//! and skills, and writes the categorized data to separate JSON files.
//!
//! # Returns
//!
//! Returns `true` if the entire process of reading, categorizing, and writing to files
//! is successful. Returns `false` if any step encounters an error.
//!
//! # Examples
//!
//! ```rust
//! use modules_task::modules::employee_task::employee;
//!
//! let success = employee();
//! assert_eq!(success, true);
//! ```
//!
//! # Panics
//!
//! Panics if there is an error during the process of reading, deserializing, categorizing,
//! or writing to files.
//!
//! # Notes
//!
//! - Reads employee data from the file located at "./src/data/Employee.json".
//! - Categorizes employees into three groups: mid_rust, jr_java, and sr_or_c.
//! - Writes the categorized data to separate JSON files under "./src/data/emp_task/".
//!
//! # Errors
//!
//! Returns `false` if there is any error during the process, such as reading the file,
//! deserialization failure, or writing to files.
//!
//! # Safety
//!
//! Assumes that the file paths are valid and the data can be successfully deserialized
//! into the expected structures.
//!
//! # Panics
//!
//! Panics if there is an error during the process of reading, deserializing, categorizing,
//! or writing to files.
//!

use crate::modules::structures::{Employee, Position};
use std::fs;
pub fn employee() -> bool {
    //reading the content of file as string
    let employee_data = fs::read_to_string("./src/data/Employee.json");

    //handling the result
    match employee_data {
        Ok(data) => {
            // deserializing data into vector of Employee struct
            let employee_data: Result<Vec<Employee>, serde_json::Error> =
                serde_json::from_str(&data);
            // handling the result of deserialization
            match employee_data {
                Ok(data) => {
                    // initializing empty vectors according to requirements
                    let mut mid_and_rust: Vec<Employee> = vec![];
                    let mut jr_and_java: Vec<Employee> = vec![];
                    let mut sr_or_c: Vec<Employee> = vec![];

                    //iterating through each employee
                    for curr_employee in data {
                        match curr_employee.position {
                            // Categorize employees based on their position and skills.
                            Some(Position::SoftwareDeveloper)
                                if curr_employee.skills.contains(&String::from("Rust")) =>
                            {
                                mid_and_rust.push(curr_employee);
                            }
                            Some(Position::JuniorDeveloper)
                                if curr_employee.skills.contains(&String::from("Java")) =>
                            {
                                jr_and_java.push(curr_employee);
                            }
                            Some(Position::SeniorDeveloper) => {
                                sr_or_c.push(curr_employee);
                            }
                            // Categorize employees with skills in C#.
                            _ => {
                                if curr_employee.skills.contains(&String::from("C#")) {
                                    sr_or_c.push(curr_employee);
                                }
                            }
                        }
                    } // Write categorized employee data to separate JSON files.
                    fs::write(
                        "./src/data/emp_task/mid_rust.json",
                        serde_json::to_string_pretty(&mid_and_rust)
                            .expect("failed to serialize data"),
                    )
                    .expect("failed to write file");
                    fs::write(
                        "./src/data/emp_task/jr_java.json",
                        serde_json::to_string_pretty(&jr_and_java)
                            .expect("failed to serialize data"),
                    )
                    .expect("failed to write file");
                    fs::write(
                        "./src/data/emp_task/sr_or_c.json",
                        serde_json::to_string_pretty(&sr_or_c).expect("failed to serialize data"),
                    )
                    .expect("failed to write file");
                }
                Err(_) => return false, // Return false if deserialization fails.
            }
        }

        Err(_) => return false, // Return false if reading the file fails.
    };
    return true; // Return true if the entire process is successful.
}
