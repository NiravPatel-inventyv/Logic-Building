//! Reads employee data from a JSON file, categorizes employees based on their position
//! and skills, and writes the categorized data to separate JSON files with the use of hashmap.

use serde_json::{json, Value};

use crate::modules::structures::{Employee, Position};
use std::{collections::HashMap, fs};
pub fn employee_with_hashmap() -> bool {
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
                    }

                    let mut mid_and_rust: Vec<HashMap<String, Value>> =
                        convert_hashmap(mid_and_rust);
                    let mut jr_and_java: Vec<HashMap<String, Value>> = convert_hashmap(jr_and_java);
                    let mut sr_or_c: Vec<HashMap<String, Value>> = convert_hashmap(sr_or_c);

                    // Write categorized employee data to separate JSON files.
                    fs::write(
                        "./src/data/emp_hashmap_task/mid_rust.json",
                        serde_json::to_string_pretty(&mid_and_rust)
                            .expect("failed to serialize data"),
                    )
                    .expect("failed to write file");
                    fs::write(
                        "./src/data/emp_hashmap_task/jr_java.json",
                        serde_json::to_string_pretty(&jr_and_java)
                            .expect("failed to serialize data"),
                    )
                    .expect("failed to write file");
                    fs::write(
                        "./src/data/emp_hashmap_task/sr_or_c.json",
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

pub fn convert_hashmap(arr: Vec<Employee>) -> Vec<HashMap<String, Value>> {
    let mut hashmap_vec = Vec::new();
    for data in arr {
        let mut curr_emp_hashmap = HashMap::new();
        curr_emp_hashmap.insert("name".to_string(), json!(data.name));
        curr_emp_hashmap.insert("age".to_string(), json!(data.age));
        curr_emp_hashmap.insert("position".to_string(), json!(data.position));
        curr_emp_hashmap.insert("skills".to_string(), json!(data.skills));
        curr_emp_hashmap.insert("experience".to_string(), json!(data.experience));
        hashmap_vec.push(curr_emp_hashmap);
    }

    hashmap_vec
}
