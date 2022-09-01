use rand::Rng;

fn main() {
    let minimum = 0;
    let maximum = 10;
    let mut times_array:[i32; 10] = [0;10];

    for i in 0..100000 {
        let random_number:usize = rand::thread_rng().gen_range(minimum..maximum);

        times_array[random_number] += 1;
    }

    for i in 0..10 {
        println!("{i}: {}", times_array[i]);
    }
}
