use modules::{
    call_employee_hashmap_task, call_employee_task, call_frequency_task, call_student_hashmap_task,
    call_student_task, call_table_task_with_font_mapping, call_table_task_with_font_mapping2,
    call_task_schedular, call_threads,
};
pub mod modules;

/// Executes a sequence of tasks related to employees, students, and frequency analysis.
///
/// This function calls the `call_employee_task`, `call_student_task`, and `call_frequency_task`
/// functions in sequence to perform various tasks related to employees, students, and frequency analysis.
///

pub fn run() {
    // call_employee_task();
    // call_student_task();
    // call_frequency_task();
    // call_employee_hashmap_task();
    // call_student_hashmap_task();
    // call_table_task_with_font_mapping2();
    // call_threads();
    call_task_schedular();
}
