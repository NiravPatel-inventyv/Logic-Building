use super::structures::Student;
use std::fs;

/// Reads student data from a JSON file, calculates percentage and grade for each student,
/// and updates the data in a new JSON file.
///
pub fn students() -> bool {
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

            // Serialize the updated data to a new JSON file.
            let updated_json =
                serde_json::to_string_pretty(&struct_data).expect("Failed to serialize data");

            // Write the updated data to a new JSON file.
            match fs::write("./src/data/student_task/updated_data.json", &updated_json) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}
