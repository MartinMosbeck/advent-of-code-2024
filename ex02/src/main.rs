use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::Ordering;

fn line_to_vector(line: &String) -> Vec<u32> {
    let mut parts = line.split_whitespace();
    let mut elements: Vec<u32> = Vec::new();

    while let Some(string) = parts.next() {
        match string.parse::<u32>() {
            Ok(number) => elements.push(number),
            Err(_) => break
        }
    }

    elements
}

fn is_order_same(elem1: &u32, elem2: &u32, cur_order: &Ordering) -> bool{
    return elem1.cmp(elem2) == *cur_order;
}

fn is_distance_ok(elem1: &u32, elem2: &u32) -> bool{
    let distance = (*elem1 as i32 - *elem2 as i32).abs();
    distance >= 1 && distance <= 3
}

fn is_safe_report(elem1: &u32, remaining: &[u32], cur_order: &mut Ordering) -> bool {
    // Safe if both:
    //   - The levels are either all increasing or all decreasing.
    //   - Two adjacent levels differ by at least one and at most three.

    let elem2 = match remaining.get(0) {
        Some(elem2) => elem2,
        _ => return true,
    };

    if *cur_order == Ordering::Equal {
        *cur_order = elem1.cmp(elem2);
    }

    if is_order_same(&elem1, elem2, cur_order) && is_distance_ok(&elem1, elem2) {
        return is_safe_report(&remaining[0], &remaining[1..], cur_order);
    }

    return false;
}

fn get_filename(use_example: &bool) -> String {
    if *use_example {
        return String::from("example.txt");
    }

    return String::from("input.txt");
}

fn main() {
    let use_example = false;

    let filename: String = get_filename(&use_example);

    let mut num_safe_reports: usize = 0;
    let mut num_safe_reports_with_dampener: usize = 0;

    let file = File::open(filename).expect("File not openable");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let content_line = line.expect("Error reading line");
        let report = line_to_vector(&content_line);

        if use_example {
            println!("-------------------------------------------------");
            println!("Report: {:?}", report);
        }

        let mut ordering = Ordering::Equal;
        let is_safe = is_safe_report(&report[0], &report[1..], &mut ordering);
        if is_safe {
            num_safe_reports += 1;
            num_safe_reports_with_dampener += 1;

            if use_example {
                println!("safe: {is_safe}");
            }
            continue
        }

        let is_safe_with_dampener: bool = {
            let mut found_safe = false;

            for i in 0..report.len() {
                let mut report_new = report.clone();
                let removed = report_new.remove(i);

                if use_example {
                    println!("Trying removed {removed}: {:?}", report_new);
                }

                let mut ordering = Ordering::Equal;
                let is_safe = is_safe_report(
                    &report_new[0], &report_new[1..], &mut ordering
                );

                if is_safe {
                    found_safe = true;
                    num_safe_reports_with_dampener += 1;
                    break;
                }
            }

            found_safe
        };

        if use_example {
            println!("safe: false , safe_with_dampener: {is_safe_with_dampener}");
        }

    }
    println!("===============================================================");
    println!("#Safe reports: {num_safe_reports}");
    println!("#Safe reports with dampener: {num_safe_reports_with_dampener}");
}
