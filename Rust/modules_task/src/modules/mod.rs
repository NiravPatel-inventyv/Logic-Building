//! Calls tasks related to employees, students, and frequency analysis.
//!
//! This module provides functions to perform various tasks related to employees, students,
//! and frequency analysis. The functions `call_employee_task`, `call_student_task`, and
//! `call_frequency_task` can be used to execute specific tasks..
use self::{
    employee_hashmap::employee_with_hashmap, employee_task::employee, frequency_task::merged_task,
    student_hashmap::students_with_hashmap, student_task::students,
    table_task_with_font_mapping::table_task_with_font_mapping,
    table_task_with_font_mapping_2::table_task_with_font_mapping_2, task_scheduler::task_scheduler,
    threads::threads,
};
pub mod employee_hashmap;
pub mod employee_task;
pub mod frequency_task;
pub mod structures;
pub mod student_hashmap;
pub mod student_task;
pub mod table_task_with_font_mapping;
pub mod table_task_with_font_mapping_2;
pub mod task_scheduler;
pub mod threads;
/// Calls the task related to updating student records.
///
/// This function calls the `students` function to update student records based on certain
/// calculations, and it prints a success or failure message accordingly.
pub fn call_student_task() {
    let res = students();
    if res {
        println!("Students records updated succesfully!");
    } else {
        println!("Fail to update student records!");
    }
}

/// Calls the task related to filtering employee records.
///
/// This function calls the `employee` function to filter employee records based on certain
/// criterias, and it prints a success or failure message accordingly.
pub fn call_employee_task() {
    let res = employee();
    if res {
        println!("Employee records filtered succesfully!");
    } else {
        println!("Fail to filter employee records!");
    }
}

/// Calls tasks related to frequency analysis.
///
/// This function calls the `merged_task` function from the `frequency_task` module to
/// perform a merged task involving frequency analysis. It can be used to execute specific
/// frequency analysis tasks on strings.
pub fn call_frequency_task() {
    // call_task1();
    // call_task2();
    merged_task();
}

/// This function calls the `students` function to update student records based on certain
/// calculations, and it prints a success or failure message accordingly.
pub fn call_student_hashmap_task() {
    let res = students_with_hashmap();
    if res {
        println!("Students records updated succesfully!");
    } else {
        println!("Fail to update student records!");
    }
}

/// this function calls the employee_hashset task
pub fn call_employee_hashmap_task() {
    let res = employee_with_hashmap();
    if res {
        println!("Employee records filtered succesfully!");
    } else {
        println!("Fail to filter employee records!");
    }
}

/// this function calls the table_task_with_font_mapping
pub fn call_table_task_with_font_mapping() {
    table_task_with_font_mapping();
}
/// this function calls the table_task_with_font_mapping
pub fn call_table_task_with_font_mapping2() {
    table_task_with_font_mapping_2();
}

/// this function calls the threads_task
pub fn call_threads() {
    threads()
}

/// this function calls the threads_task
pub fn call_task_schedular() {
    task_scheduler();
}
