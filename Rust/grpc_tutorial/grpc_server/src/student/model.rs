use serde::{Deserialize, Serialize};

use super::Student::StudentRequest;

/// Represents a student with personal details, contact information, academic performance, and calculated metrics.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Student {
    pub id: u32,
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
    pub marks: Vec<u32>,
    /// The calculated percentage based on the student's marks.
    pub percentage: f32,
    /// The calculated grade based on the student's percentage.
    pub grade: String,
}

pub fn map_struct_data(data: StudentRequest) -> Student {
    let student = Student {
        id: data.id,
        name: data.name,
        email: data.email,
        phone: data.phone,
        city: data.city,
        address: data.address,
        marks: data.marks,
        percentage: data.percentage,
        grade: data.grade,
    };

    student
}
