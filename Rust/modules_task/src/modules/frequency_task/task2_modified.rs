use crate::modules::structures::Task2Response;
use std::io;
/// function to take input of two strings and find of letter frequency
pub fn string_analyzer() -> Task2Response {
    // initializing empty string for inputs
    let mut _str1 = String::new();
    let mut _str2 = String::new();
    //input strings
    println!("Enter srting1 :");
    io::stdin()
        .read_line(&mut _str1)
        .expect("error while reading input");
    println!("Enter srting2 :");
    io::stdin()
        .read_line(&mut _str2)
        .expect("error while reading input");
    //triming the string as it contains
    let str1 = _str1.trim();
    let str2 = _str2.trim();

    // return both the vectors in struct format
    let result = fill_vectors(str1, str2);
    result
}

pub fn letter_count(s: &str) -> Vec<(char, u8)> {
    let s = s.to_lowercase();
    let mut char_count = Vec::new();
    // looping through each char in string and storing in hashmap
    for each_char in s.chars() {
        let mut found = false;

        for i in 0..char_count.len() {
            let (c, count) = char_count[i];
            if c == each_char {
                char_count[i].1 += 1;
                found = true;
                break;
            }
        }

        if !found {
            char_count.push((each_char, 1));
        }
    }
    char_count
}

/// function to take two strings and return the letter_count_vector and leftout vector
pub fn fill_vectors(str1: &str, str2: &str) -> Task2Response {
    // Count the occurrences of characters in str1 and str2
    let str1_count: Vec<(char, u8)> = letter_count(str1);
    let str2_count: Vec<(char, u8)> = letter_count(str2);
    // Initialize vectors
    let mut letter_count_vec: Vec<(char, u8)> = Vec::new();
    let mut left_out: Vec<(char, u8)> = Vec::new();

    // Iterate through characters in str1_count
    for i in 0..str1_count.len() {
        let (letter, char_count_in_str1) = str1_count[i];

        // Check if the character is also present in str2
        let mut found_in_str2 = false;
        let mut char_count_in_str2 = 0;

        for j in 0..str2_count.len() {
            let (c, count) = str2_count[j];
            if c == letter {
                found_in_str2 = true;
                char_count_in_str2 = count;
                break;
            }
        }

        // Calculate total count
        let total_count = char_count_in_str1 + char_count_in_str2;

        // If total count is greater than 1, add to letter_count_vec
        if total_count > 1 {
            letter_count_vec.push((letter, total_count));
        } else {
            // If character is not present in str2, add to left_out
            if !found_in_str2 {
                left_out.push((letter, 0));
            }
        }
    }

    // Iterate through characters in str2_count
    for i in 0..str2_count.len() {
        let (letter, char_count_in_str2) = str2_count[i];

        // Check if the character is also present in str1
        let mut found_in_str1 = false;

        for j in 0..str1_count.len() {
            let (c, _) = str1_count[j];
            if c == letter {
                found_in_str1 = true;
                break;
            }
        }

        // If character is not present in str1, add to left_out
        if !found_in_str1 {
            left_out.push((letter, 0));
        }
    }

    //sorting both the vectors
    sort_vecs(&mut letter_count_vec);
    sort_vecs(&mut left_out);
    letter_count_vec.remove(0);
    // Return the filled vectors
    Task2Response {
        left_out,
        letter_count_vec,
    }
}

//sorting vectors alphabetically
pub fn sort_vecs(arr: &mut Vec<(char, u8)>) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j].0 > arr[j + 1].0 {
                arr.swap(j, j + 1);
            }
        }
    }
}
