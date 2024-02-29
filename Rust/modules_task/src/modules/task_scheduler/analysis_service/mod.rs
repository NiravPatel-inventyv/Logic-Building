use std::{
    collections::{HashMap, VecDeque},
    thread,
    time::Duration,
};

use super::shared_data::{Request, QUEUES};

pub fn analysis_service() {

    thread::sleep(Duration::from_secs(30));
    if let Ok(mut queues) = QUEUES.write() {
        let mut escalated_data = HashMap::new();
        for (queue_name, queue_val) in queues.iter_mut() {
            let mut reqs = VecDeque::new();
            let mut new_queue_name: Option<String> = None;
            for each_req in queue_val.pop_front().into_iter() {
                let time_elapsed = each_req.timestamp.elapsed().unwrap_or_default();
                let mut escalation_level ;

                match time_elapsed.as_secs() {
                    0..=30 => escalation_level = 1,
                    31..=60 => escalation_level = 2,
                    61..=90 => escalation_level = 4,
                    91..=120 => escalation_level = 5,
                    _ => escalation_level = 5,
                }
                if escalation_level > 1 {
                    let mut old_queue_name: Vec<String> =
                        queue_name.split("_").map(|s| s.to_string()).collect();
                    let que_len = old_queue_name.len();
                    old_queue_name[que_len - 1] = format!("L{}", escalation_level);
                    let new_name = old_queue_name.join("_");
                    new_queue_name = Some(new_name);
                    reqs.push_back(each_req);
                }
            }
            escalated_data
                .entry(new_queue_name)
                .and_modify(|queue: &mut VecDeque<Request>| queue.append(&mut reqs))
                .or_insert(reqs);
        }

        for (queue_name, mut queue_val) in escalated_data {
            if queue_name.is_some() {
                queues
                    .entry(queue_name.unwrap())
                    .and_modify(|queue| queue.append(&mut queue_val));
            }
        }
    }
}
