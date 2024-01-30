use serde_json::{json, Value};

use super::structures::Student;
use std::{collections::HashMap, fs};

/// Reads student data from a JSON file, calculates percentage and grade for each student,
/// and updates the data in a new JSON file.
///
pub fn students_with_hashmap() -> bool {
    let path = "./src/data/StudentData.json";
    let file_data = fs::read_to_string(path);

    match file_data {
        Ok(data) => {
            let mut struct_data: Vec<Student> = serde_json::from_str(&data).unwrap();

            // Calculate percentage and grade for each student.
            for i in 0..struct_data.len() {
                struct_data[i].calculate_percentage();
                struct_data[i].calculate_grade();
            }
            let mut hashmap_vec: Vec<HashMap<String, Value>> = Vec::new();
            for student_data in struct_data {
                let mut data_hashmap: HashMap<String, Value> = HashMap::new();
                data_hashmap.insert("name".to_string(), json!(student_data.name));
                data_hashmap.insert("email".to_string(), json!(student_data.email));
                data_hashmap.insert("phone".to_string(), json!(student_data.phone));
                data_hashmap.insert("city".to_string(), json!(student_data.city));
                data_hashmap.insert("address".to_string(), json!(student_data.address));
                data_hashmap.insert("marks".to_string(), json!(student_data.marks));
                data_hashmap.insert("percentage".to_string(), json!(student_data.percentage));
                data_hashmap.insert("grade".to_string(), json!(student_data.grade));
                hashmap_vec.push(data_hashmap);
            }

            // Serialize the updated data to a new JSON file.
            let updated_json =
                serde_json::to_string_pretty(&hashmap_vec).expect("Failed to serialize data");

            // Write the updated data to a new JSON file.
            match fs::write(
                "./src/data/student_hashmap_task/updated_data.json",
                &updated_json,
            ) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}
