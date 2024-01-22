use serde::{Deserialize, Serialize};

/// Represents the position or role of an employee.
#[derive(Serialize, Deserialize, Debug)]
pub enum Position {
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

/// Represents an employee with personal details, skills, position, and experience.
#[derive(Serialize, Deserialize, Debug)]
pub struct Employee {
    /// The name of the employee.
    pub name: String,
    /// The age of the employee.
    pub age: i32,
    /// The skills possessed by the employee.
    pub skills: Vec<String>,
    /// The position or role of the employee.
    pub position: Option<Position>,
    /// The experience of the employee in years.
    #[serde(rename = "experiance(y)")]
    pub experience: Option<i32>,
}

/// Represents a student with personal details, contact information, academic performance, and calculated metrics.
#[derive(Debug, Deserialize, Serialize)]
pub struct Student {
    /// The name of the student.
    pub name: String,
    /// The email address of the student.
    pub email: String,
    /// The phone number of the student.
    pub phone: String,
    /// The city where the student resides.
    pub city: String,
    /// The address of the student.
    pub address: String,
    /// The marks obtained by the student in various subjects.
    pub marks: Vec<u16>,
    /// The calculated percentage based on the student's marks.
    pub percentage: Option<f32>,
    /// The calculated grade based on the student's percentage.
    pub grade: Option<char>,
}

impl Student {
    /// Calculates the percentage of the student based on their marks.
    pub fn calculate_percentage(&mut self) {
        let mut total_marks = 0.0;
        for i in 0..self.marks.len() {
            total_marks += self.marks[i] as f32;
        }
        self.percentage = Some(total_marks / self.marks.len() as f32);
    }

    /// Calculates the grade of the student based on their percentage.
    pub fn calculate_grade(&mut self) {
        let grade = match self.percentage.unwrap() {
            p if p >= 90.0 => 'A',
            p if p >= 80.0 => 'B',
            p if p >= 70.0 => 'C',
            p if p >= 60.0 => 'D',
            _ => 'F',
        };
        self.grade = Some(grade);
    }
}

/// Represents the response structure for a Task 2 operation involving letter counts.
#[derive(Debug)]
pub struct Task2Response {
    /// Vector containing characters and their counts.
    pub letter_count_vec: Vec<(char, u8)>,
    /// Vector containing characters left out after analysis.
    pub left_out: Vec<(char, u8)>,
}
