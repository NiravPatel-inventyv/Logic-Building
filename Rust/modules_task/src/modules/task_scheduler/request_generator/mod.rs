use super::shared_data::{Request, PENDING_TASKS, SKILLS};
use rand::Rng;
use std::{sync::Arc, thread, time::Duration};

pub fn request_generator() {
    let pending_task_ref = Arc::clone(&PENDING_TASKS);
    let mut count = 1;
    loop {
        thread::sleep(Duration::from_secs(2));
        let req_data = generate_random_data(count);
        match pending_task_ref.write() {
            Ok(mut pending_task) => {
                pending_task.push_back(req_data);
            }
            Err(_) => {
                println!("Error while generating requests");
            }
        }
        count += 1;
    }
}

fn generate_random_data(id: i32) -> Request {
    let skills = SKILLS.read().unwrap();
    let task_types = vec!["Call", "Chat"];
    let languages = vec!["English", "Spanish"];
    let random_skill_index = rand::thread_rng().gen_range(0..skills.len());
    let random_task_type_index = rand::thread_rng().gen_range(0..task_types.len());
    let random_language_index = rand::thread_rng().gen_range(0..languages.len());

    let random_skill = skills[random_skill_index].clone();
    let random_task_type = task_types[random_task_type_index].to_string();
    let random_language = languages[random_language_index].to_string();

    let req_data = Request::new(id, random_skill, random_task_type, random_language);
    req_data
}
