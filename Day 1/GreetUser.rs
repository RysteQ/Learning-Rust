use std::io;

fn main() {
    println!("What is your name ?");
    print!("=> ");

    let mut username:String = String::new();

    io::stdin().read_line(&mut username).expect("Error getting input from the user");

    println!("Hello there {}", username);
}