use modules::{call_employee_task, call_frequency_task, call_student_task};
pub mod modules;

/// Executes a sequence of tasks related to employees, students, and frequency analysis.
///
/// This function calls the `call_employee_task`, `call_student_task`, and `call_frequency_task`
/// functions in sequence to perform various tasks related to employees, students, and frequency analysis.
///
/// # Examples
///
/// ```rust
/// use modules_task::run;
///
/// // Execute the sequence of tasks
/// run();
/// ```
///
/// This example demonstrates how to use the `run` function to execute a sequence of tasks.
pub fn run() {
    call_employee_task();
    call_student_task();
    call_frequency_task();
}
