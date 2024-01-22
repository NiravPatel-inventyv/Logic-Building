//! Calls tasks related to employees, students, and frequency analysis.
//!
//! This module provides functions to perform various tasks related to employees, students,
//! and frequency analysis. The functions `call_employee_task`, `call_student_task`, and
//! `call_frequency_task` can be used to execute specific tasks.
//!
//! # Examples
//!
//! ```rust
//! use crate::modules::{call_employee_task, call_student_task, call_frequency_task};
//!
//! // Call employee-related task
//! call_employee_task();
//!
//! // Call student-related task
//! call_student_task();
//!
//! // Call frequency analysis task
//! call_frequency_task();
//! ```
//!
//! This example demonstrates how to use the functions provided by this module to perform
//! tasks related to employees, students, and frequency analysis.
use self::{
    employee_task::employee,
    frequency_task::merged_task,
    student_task::students,
};

pub mod frequency_task;
pub mod employee_task;
pub mod structures;
pub mod student_task;
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
