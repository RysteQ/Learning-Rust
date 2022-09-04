use std::fs;
use std::io;

fn main() {
    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading user input");

    let file_contents: String = fs::read_to_string(user_input.trim()).expect("Error reading file");

    println!("{file_contents}");
}