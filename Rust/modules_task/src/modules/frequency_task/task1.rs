//! Reads a string from the user, prompts for a word to replace, and splits the input string
//! by the specified word. The resulting substrings are then joined using underscores and the
//! final string is returned.
//!
//! # Examples
//!
//! ```rust
//! use std::io;
//!
//! let result = split_string();
//! println!("Result: {}", result);
//! ```
//!
//! # Panics
//!
//! This function panics if there is an error while reading input using `std::io::stdin()`.
//!
//! # Returns
//!
//! - Returns a `String` containing the input string with substrings joined by underscores.
//!
//! # Note
//!
//! - Leading and trailing whitespaces are trimmed from the input strings before processing.
//! - The user is prompted to enter the input string and the word to replace.
//!
//! # Errors
//!
//! This function may panic if there is an error while reading input using `std::io::stdin()`.
//!
//! # Safety
//!
//! This function assumes that the input strings are valid UTF-8 encoded strings.
//!
//! # Panics
//!
//! This function may panic if there is an error while reading input using `std::io::stdin()`.

use std::io;
pub fn split_string() -> String {
    let mut _str1 = String::new();
    let mut _letter = String::new();

    // Read the input string from the user.
    println!("Enter string :");
    io::stdin()
        .read_line(&mut _str1)
        .expect("error while reading input");

    // Read the word to replace from the user.
    println!("Enter word to replace :");
    io::stdin()
        .read_line(&mut _letter)
        .expect("error while reading input");

    // Trim leading and trailing whitespaces from the input strings.
    let str1 = _str1.trim();
    let letter = _letter.trim();

    // Split the input string by the word to replace and join the substrings with underscores.
    let substrings: Vec<&str> = str1.split(letter).collect();
    let joined_result: String = substrings.join("_");

    // Return the final result.
    joined_result
}
