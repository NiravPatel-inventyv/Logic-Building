use std::{collections::VecDeque, ops::Deref, sync::Arc, thread, time::Duration};

use super::shared_data::{Request, Status, QUEUES, QUEUES_PRIORITY, SKILLS, USERS};

pub fn task_assigner_service() {
    thread::sleep(Duration::from_secs(1));
    let queue_priority = Arc::clone(&QUEUES_PRIORITY);
    if let Ok(mut queue) = Arc::clone(&QUEUES).write() {
        if let Ok(queue_ref) = queue_priority.write() {
            for queue_name in queue_ref.iter() {
                if let Some(queue_val) = queue.get_mut(queue_name) {
                    if !queue_val.is_empty() {
                        task_assigner(queue_val);
                    }
                }
            }
        };
    };
}
pub fn task_assigner(tasks: &mut VecDeque<Request>) {
    let user_ref = Arc::clone(&USERS);
    for request in tasks.pop_front().into_iter() {
        if let Ok(mut users) = user_ref.write() {
            let mut flag = false;
            for user in users.iter_mut() {
                if user.status == Status::Online
                    && user.skills.contains(&request.skill)
                    && user.language == request.language
                {
                    println!(
                        "user {} assigned to request {} after {:?} sec ",
                        &user.id,
                        request.id,
                        request.timestamp.elapsed().unwrap().as_secs()
                    );
                    flag = true;
                    user.status = Status::Offline;
                    break;
                }
            }
            if !flag {
                tasks.push_back(request);
            }
        }
    }
}
