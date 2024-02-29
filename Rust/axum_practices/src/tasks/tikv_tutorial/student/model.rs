use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

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
    pub marks: Vec<u16>,
    /// The calculated percentage based on the student's marks.
    pub percentage: Option<f32>,
    /// The calculated grade based on the student's percentage.
    pub grade: Option<char>,
}

lazy_static::lazy_static! {
    #[derive(Debug,Clone)]
    pub static ref ALL_STUDENTS: Arc<RwLock<HashMap<u32,Student>>> = Arc::new(RwLock::new(HashMap::new()));
}
