use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn get_filename() -> String {
    if USE_EXAMPLE {
        return String::from("example.txt");
    }
    String::from("input.txt")
}

fn generate_buf_reader() -> BufReader<File> {
    let filename: String = get_filename();
    let file = File::open(filename).expect("File not openable");

    BufReader::new(file)
}

const USE_EXAMPLE: bool = true;

fn main() {
    let mut file_reader = generate_buf_reader();
}
