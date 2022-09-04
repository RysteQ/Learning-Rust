struct Colour {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let favourite_colour: Colour = Colour {
        red: 255,
        green: 69,
        blue: 0
    };

    println!("My favourite colour in RGB is {} {} {}", favourite_colour.red, favourite_colour.green, favourite_colour.blue);
}
