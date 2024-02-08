use lazy_static::lazy_static;
use rand::{seq::SliceRandom, Rng};
use serde::Deserialize;
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, RwLock},
    time::SystemTime,
};
#[derive(Deserialize, Debug, PartialEq)]
pub enum Status {
    Online,
    Offline,
}
#[derive(Deserialize, Debug, PartialEq)]
pub enum Language {
    English,
    Spanish,
}
#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub skills: Vec<String>,
    pub status: Status,
    pub language: Language,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum TaskType {
    Call,
    Chat,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct Request {
    pub id: i32,
    pub tasktype: TaskType,
    pub skill: String,
    pub language: Language,
    pub timestamp: SystemTime,
}
impl Request {
    pub fn new(id: i32, skill: String, task_type: String, language: String) -> Request {
        let task_type_enum = match task_type.as_str() {
            "Call" => TaskType::Call,
            "Chat" => TaskType::Chat,
            _ => panic!("Invalid task type"),
        };

        let language_enum = match language.as_str() {
            "English" => Language::English,
            "Spanish" => Language::Spanish,
            _ => panic!("Invalid language"),
        };

        Request {
            id,
            tasktype: task_type_enum,
            skill,
            language: language_enum,
            timestamp: SystemTime::now(),
        }
    }
}

lazy_static! {
    pub static ref USERS: Arc<RwLock<Vec<User>>> = Arc::new(RwLock::new(Vec::new()));
    pub static ref PENDING_TASKS: Arc<RwLock<VecDeque<Request>>> =
        Arc::new(RwLock::new(VecDeque::new()));
    pub static ref SKILLS: Arc<RwLock<Vec<String>>> = {
        let skills = vec![
            "Customer Service",
            "Problem-solving",
            "Product Knowledge",
            "Effective Communication",
            "Time Management",
            "Adaptability",
            "Team Collaboration",
            "Feedback Analysis",
            "Proactive Engagement",
            "Technical Proficiency",
            "Cultural Sensitivity",
            "Documentation",
        ]
        .iter()
        .map(|&s| s.to_string())
        .collect();
        Arc::new(RwLock::new(skills))
    };
    pub static ref QUEUES: Arc<RwLock<HashMap<String, VecDeque<Request>>>> =
        Arc::new(RwLock::new(HashMap::new()));
    pub static ref QUEUES_PRIORITY: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(Vec::new()));
}
