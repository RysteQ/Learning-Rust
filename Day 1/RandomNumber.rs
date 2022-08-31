use rand::Rng;

fn main() {
    let minimum = 0;
    let maximum = 100;

    let random_number = rand::thread_rng().gen_range(minimum..maximum);

    println!("Random number ({minimum} - {maximum})\n=> {random_number}");
}