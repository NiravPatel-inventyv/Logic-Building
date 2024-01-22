use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
enum Position {
    #[serde(rename = "Software Developer")]
    SoftwareDeveloper,
    #[serde(rename = "Sr. Software Developer")]
    SeniorDeveloper,
    #[serde(rename = "Jr. Software Developer")]
    JuniorDeveloper,
    #[serde(rename = "Project Manager")]
    ProjectManager,
    #[serde(rename = "Team Lead")]
    TeamLead,
}

#[derive(Serialize, Deserialize, Debug)]
struct Employee {
    name: String,
    age: i32,
    skills: Vec<String>,
    position: Option<Position>,
    #[serde(rename = "experiance(y)")]
    experience: Option<i32>,
}

fn main() {
    // let json_data=serde_json::to_string()
    let employee_data = fs::read_to_string("./src/Employee.json");
    match employee_data {
        Ok(data) => {
            let employee_data: Result<Vec<Employee>, serde_json::Error> =
                serde_json::from_str(&data);

            match employee_data {
                Ok(data) => {
                    let mut mid_and_rust: Vec<Employee> = vec![];
                    let mut jr_and_java: Vec<Employee> = vec![];
                    let mut sr_or_c: Vec<Employee> = vec![];
                    for curr_employee in data {
                        match curr_employee.position {
                            Some(Position::SoftwareDeveloper)
                                if curr_employee.skills.contains(&String::from("Rust")) =>
                            {
                                mid_and_rust.push(curr_employee)
                            }
                            Some(Position::JuniorDeveloper)
                                if curr_employee.skills.contains(&String::from("Java")) =>
                            {
                                jr_and_java.push(curr_employee)
                            }
                            Some(Position::SeniorDeveloper)
                                if curr_employee.skills.contains(&String::from("C#")) =>
                            {
                                sr_or_c.push(curr_employee)
                            }
                            _ => {
                                if curr_employee.skills.contains(&String::from("C#")) {
                                    sr_or_c.push(curr_employee)
                                }
                            }
                        }
                    }
                    fs::write(
                        "./src/data/mid_rust.json",
                        serde_json::to_string_pretty(&mid_and_rust).expect("msg"),
                    )
                    .expect("msg");
                    println!("mid_rust.json created successfully");
                    fs::write(
                        "./src/data/jr_java.json",
                        serde_json::to_string_pretty(&jr_and_java).expect("msg"),
                    )
                    .expect("msg");
                    println!("jr_java.json created successfully");
                    fs::write(
                        "./src/data/sr_or_c.json",
                        serde_json::to_string_pretty(&sr_or_c).expect("msg"),
                    )
                    .expect("msg");
                    println!("sr_or_c.json created successfully");
                }

                _ => (),
            }
        }

        Err(_) => (),
    };
}
