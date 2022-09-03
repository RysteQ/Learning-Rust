use std::io;

fn main() {
    let mut balance_vector: Vec<i32> = vec![];
    let mut user_input: String = String::new();

    println!("Add new values, enter END to stop execution and DISPLAY to display the values");

    while true {
        print!("=> ");

        user_input = String::new();
        
        io::stdin()
            .read_line(&mut user_input)
            .expect("Error reading user input");

        if user_input.trim() == "END" {
            break;
        } else if user_input.trim() == "DISPLAY" {
            for i in 0..balance_vector.len() {
                println!("{i}: {}", balance_vector[i]);
            }

            continue;
        }

        balance_vector.push(user_input.trim().parse::<i32>().expect("Error parsing number"));
    }
}
