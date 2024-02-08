use std::thread;

use self::{
    analysis_service::analysis_service, biforgation::biforgation_service,
    request_generator::request_generator, skill_changer::skill_changer_service,
    static_loading::static_loading, task_assigner::task_assigner_service,
};
pub mod analysis_service;
pub mod biforgation;
pub mod request_generator;
pub mod shared_data;
pub mod skill_changer;
pub mod static_loading;
pub mod task_assigner;

pub fn task_scheduler() {
    static_loading();
    let req_service_thread = thread::spawn(|| {
        request_generator();
    });
    let biforgation_service_thread = thread::spawn(|| loop {
        biforgation_service();
    });
    let analysis_service_thread = thread::spawn(|| loop {
        analysis_service();
    });
    let task_assigner_service_thread = thread::spawn(|| loop {
        task_assigner_service();
    });

    let skill_changer_service_thread = thread::spawn(|| loop {
        skill_changer_service()
    });
    req_service_thread.join().unwrap();
    biforgation_service_thread.join().unwrap();
    analysis_service_thread.join().unwrap();
    task_assigner_service_thread.join().unwrap();
    skill_changer_service_thread.join().unwrap();
}
