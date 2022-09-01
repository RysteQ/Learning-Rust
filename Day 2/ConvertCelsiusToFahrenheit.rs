use std::io;

fn main() {
    let mut user_input:String = String::new();
    let mut celsius:f32;
    let mut fahrenheit:f32;

    print!("Celsius: ");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading user input");

    celsius = user_input.trim().parse::<f32>().expect("Error converting string to float");
    fahrenheit = (celsius * 1.8) + 32;

    println!("Fahrenheit: {fahrenheit}");
}