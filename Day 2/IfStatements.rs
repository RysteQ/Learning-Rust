use std::io;

fn main() {
    let code:i32 = 2832;
    let mut user_input:String = String::new();
    let mut user_code:i32;

    println!("Enter code");
    print!("=> ");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading user input");

    user_code = user_input.trim().parse::<i32>().expect("Error converting input");
    
    if code == user_code {
        println!("Correct code entered");
    } else {
        println!("Incorrect code entered");
    }
}