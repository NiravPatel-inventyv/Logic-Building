pub mod task1;

pub mod task2_modified;
use self::{task1::split_string, task2_modified::string_analyzer};
/// This module provides functions to perform various tasks using the `task1` and `task2` submodules.
/// function to call task 1
pub fn call_task1() {
    split_string();
}

/// function to call task 2
pub fn call_task2() {
    let res = string_analyzer();
    println!("{:?}", res);
}
/// merge function to get string with underscore replaced by chars
pub fn merged_task() {
    let mut task1_result = split_string();
    let mut task2_result = string_analyzer();
    // calling replace_string function and storing its result
    let result = replace_string(&mut task1_result, &mut task2_result.letter_count_vec);
    println!("orignal str  : \n \t\t {}", task1_result);
    println!("modified string will be : \n \t\t {}", result);
    // println!("modified vec will be : \n \t\t {:?}", task2_result);
}

/// Replaces underscores in a string with characters from a given letter count vector.
///
/// This function takes a string with underscores and a vector containing characters and
/// their counts. It replaces each underscore in the input string with characters from the
/// vector, ensuring that the counts are decremented appropriately.
pub fn replace_string(input_str: &mut str, filling_vec: &mut Vec<(char, u8)>) -> String {
    let mut result_string = String::new();
    let mut idx = 0;

    //looping through each char of string
    for c in input_str.chars() {
        //check if the char is underscore
        if c == '_' {
            // if index is less then filling vector lenght then execute logic
            if idx < filling_vec.len() {
                let (letter, count) = &mut filling_vec[idx];

                // Check if the count is greater than 0
                if *count > 0 {
                    // Decrease the count and add the letter to the result
                    *count -= 1;
                    result_string.push(*letter);
                } else {
                    // If count is zero, move to the next character in the vector
                    idx += 1;
                    let next_char = if idx < filling_vec.len() {
                        filling_vec[idx].0
                    } else {
                        '_'
                    };
                    result_string.push(next_char);
                }
            } else {
                // If the filling vector is empty, add an underscore to the result
                result_string.push('_');
            }
        } else {
            // If the character is not an underscore, add it to the result
            result_string.push(c);
        }
    }

    result_string
}
