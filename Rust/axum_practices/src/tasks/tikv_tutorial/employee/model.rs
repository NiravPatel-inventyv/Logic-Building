use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
/// Represents the position or role of an employee.
#[derive(Serialize, Deserialize, Debug,Clone)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Employee {
    pub id : u32,
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

lazy_static::lazy_static! {
#[derive( Debug,Clone)]
    pub static ref ALL_EMPLOYEES: Arc<RwLock<HashMap<u32,Employee>>> = Arc::new(RwLock::new(HashMap::new()));
}
