use std::io;
use std::process;

fn main() {
    let mut user_input:String = String::new();
    let number_one:u32;
    let number_two:u32;
    let result:u32;

    println!("1st number");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading user input");

    number_one = user_input.trim().parse::<u32>().expect("Error converting number");
    user_input = String::new();

    println!("2nd number");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading user input");

    number_two = user_input.trim().parse::<u32>().expect("Error converting number");
    user_input = String::new();

    println!("Operation (+, -, /, %, *, ^)");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading user input");

    match user_input.trim() {
        "+" => result = number_one + number_two,
        "-" => result = number_one - number_two,
        "/" => result = number_one / number_two,
        "%" => result = number_one % number_two,
        "*" => result = number_one * number_two,
        "^" => result = u32::pow(number_one, number_two),
        _ => {
            println!("Incorrect operation type");
            process::exit(-1);
        }
    }

    println!("{number_one} {} {number_two} = {result}", user_input.trim());
}