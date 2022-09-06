use std::fs;

fn main() {
    let data_to_write: &str = "Random sh.... stuff";
    fs::write("output.txt", data_to_write).expect("Error writing to output file");
}
