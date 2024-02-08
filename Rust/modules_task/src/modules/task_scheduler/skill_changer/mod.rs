use std::{thread, time::Duration};

use rand::{seq::SliceRandom, Rng};

use super::shared_data::{Status, User, SKILLS, USERS};

pub fn skill_changer_service() {
    thread::sleep(Duration::from_secs(2));

    if let Ok(mut users) = USERS.write() {
        let random_user_index = rand::thread_rng().gen_range(0..users.len());
        change_status(&mut users[random_user_index]);
        change_skills(&mut users[random_user_index]);
    }
}

fn change_skills(user: &mut User) {
    let skills = SKILLS.read().unwrap();
    let num_skills_to_select = rand::thread_rng().gen_range(1..=4);
    let selected_skills: Vec<String> = skills
        .choose_multiple(&mut rand::thread_rng(), num_skills_to_select)
        .cloned()
        .collect();
    user.skills = selected_skills;
}
pub fn change_status(user: &mut User) {
    match user.status {
        Status::Online => user.status = Status::Online,
        Status::Offline => user.status = Status::Online,
        
    }
}
