use std::io;
use rand::Rng;

fn main() {
    let minimum:i32 = 0;
    let maximum:i32 = 100;
    let random_number:i32 = rand::thread_rng().gen_range(minimum..maximum); 
    let mut user_input:String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading user input");

    let user_number:i32 = user_input.trim().parse::<i32>().expect("Error converting number");

    if user_number == random_number {
        println!("You guessed the right number");
    } else {
        println!("The right number was {random_number}")
    }
}
