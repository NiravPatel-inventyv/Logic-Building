use std::{collections::VecDeque, fs, ops::Deref, sync::Arc};

use super::shared_data::{User, QUEUES, QUEUES_PRIORITY, SKILLS, USERS};

pub fn static_loading() {
    load_users();
    load_queues();
}
fn load_users() {
    let user_data_str = fs::read_to_string("./src/data/Master_Data.json");
    match user_data_str {
        Ok(user_data_str) => {
            let user_data: Result<Vec<User>, serde_json::Error> =
                serde_json::from_str(&user_data_str);
            match user_data {
                Ok(user_data) => {
                    let user_ref = Arc::clone(&USERS);
                    let users_vec = user_ref.write();
                    match users_vec {
                        Ok(mut users) => {
                            users.extend(user_data);
                        }
                        Err(_) => {
                            println!("error while adding user data");
                        }
                    }
                }
                Err(_) => {
                    println!("error while deserializong user data");
                }
            }
        }
        Err(_) => {
            println!("error while reading user data file");
        }
    }
}

fn load_queues() {
    let skills = SKILLS.read().unwrap();
    let task_types = vec!["Call", "Chat"];
    let languages = vec!["English", "Spanish"];
    let intervals = vec!["L5","L4","L3","L2","L1"];
    let mut queues = QUEUES.write().unwrap();
    let mut queues_priority = QUEUES_PRIORITY.write().unwrap();
    for interval in &intervals {
    for skill in skills.deref() {
        for task_type in &task_types {
            for language in &languages {
                    let queue_name = format!("{}_{}_{}_{}", task_type, skill, language, interval);
                    queues.insert(queue_name.clone(), VecDeque::new());
                    queues_priority.push(queue_name);
                }
            }
        }
    }
}
