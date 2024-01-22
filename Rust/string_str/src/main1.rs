// fn main() {
// //  References
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

// }

// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//     s.len()
// } 

  // fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

/// Dangling
fn main() {
    let reference_to_nothing = no_dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }show  error as string will be dropped after function execution
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}