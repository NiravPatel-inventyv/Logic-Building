use std::{thread, time::Duration};

use crate::modules::task_scheduler::shared_data::TaskType;

use super::shared_data::{Language, PENDING_TASKS, QUEUES};

pub fn biforgation_service() {
    thread::sleep(Duration::from_secs(5));
    if let Ok(mut queues) = QUEUES.write() {
        if let Ok(mut pending_tasks) = PENDING_TASKS.write() {
            for task in pending_tasks.pop_front().into_iter() {
                let skill = &task.skill;
                let tasktype = if &task.tasktype == &TaskType::Call {
                    "Call".to_string()
                } else {
                    "Chat".to_string()
                };
                let language = if &task.language == &Language::English {
                    "English".to_string()
                } else {
                    "Spanish".to_string()
                };
                let queue_name = format!("{}_{}_{}_L1", tasktype, skill, language);
                queues.entry(queue_name).and_modify(|q| q.push_back(task));
            }
        };
    };
}
