// I already know someone can simplify this, I just want to use consts and not care about the logic

fn main() {
    const limit: u8 = 160;
    let mut counter: u8 = 1;

    loop {
        if counter % 3 == 0 && counter % 5 == 0 {
            println!("Fizzbuzz");
        } else if counter % 3 == 0 {
            println!("Fizz");
        } else if counter % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{counter}");
        }

        if counter == limit {
            break;
        }

        counter += 1;
    }
}
