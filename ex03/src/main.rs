use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn get_valid_calculations(line: &String) -> Vec<(u32, u32)> {
    let mut valid_calculations: Vec<(u32, u32)> = Vec::new();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for caps in re.captures_iter(line) {
        let first_number: u32 = caps[1].parse().unwrap();
        let second_number: u32 = caps[2].parse().unwrap();
        valid_calculations.push((first_number, second_number));

        if USE_EXAMPLE {
            println!("mul({first_number},{second_number})");
            println!("------------------------");
        }
    }
    valid_calculations
}

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

fn collect_valid_calculations(reader: &mut BufReader<File>) -> Vec<(u32, u32)> {
    let mut valid_calculations: Vec<(u32, u32)> = Vec::new();
    let mut memory_string = String::new();

    for line in reader.lines() {
        memory_string.push_str(&line.unwrap());
    }

    if USE_EXAMPLE {
        println!("{:?}", memory_string);
    }

    let parts: Vec<&str> = memory_string.split("do()").collect();
    let mut parts_sanitized: Vec<String> = Vec::new();
    for part in parts {
        match part.find("don't()") {
            Some(pos) => parts_sanitized.push(String::from(&part[..pos])),
            _ => parts_sanitized.push(String::from(part)),
        }

        if USE_EXAMPLE {
            println!("{:?}", parts_sanitized);
            println!("--------------------------");
        }
    }
    for part in parts_sanitized {
        valid_calculations.extend(get_valid_calculations(&part));
    }

    valid_calculations
}


fn get_sum_calculations(valid_calculations: &Vec<(u32, u32)>) -> u32 {
    let mut sum = 0;
    for (num1, num2) in valid_calculations {
        sum += num1 * num2;
    }

    sum
}

const USE_EXAMPLE: bool = false;

fn main() {
    let mut file_reader = generate_buf_reader();
    let valid_calculations = collect_valid_calculations(&mut file_reader);
    let sum_calculations = get_sum_calculations(&valid_calculations);
    println!("Sum: {sum_calculations}");
}
