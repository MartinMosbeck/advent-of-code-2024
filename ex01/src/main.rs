use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::hash_map::Entry as HashMapEntry;
use std::cmp::Ordering;

fn get_numbers_from_line(line: String) -> (u32, u32) {
    let mut parts = line.split_whitespace();
    let a: u32 = parts.next().expect("Missing first number").parse().unwrap();
    let b: u32 = parts.next().expect("Missing second number").parse().unwrap();
    (a, b)
}

fn get_sorted_lists_from_file(file_name: &String) -> (Vec<u32>, Vec<u32>) {
    let mut list1:Vec<u32> = Vec::new();
    let mut list2:Vec<u32> = Vec::new();

    let file = File::open(file_name).expect("File not openable");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                panic!();
            },
            Ok(content) => {
                let (num1, num2) = get_numbers_from_line(content);
                list1.push(num1);
                list2.push(num2);
            }
        }
    }
    list1.sort();
    list2.sort();

    (list1, list2)
}

fn calc_distance(list1: &Vec<u32>, list2: &Vec<u32>) -> u32 {
    let mut distance: u32 = 0;

    for i in 0..list1.len() {
        let temp: i32 = list1[i] as i32 - list2[i] as i32;
        distance += temp.abs() as u32;
    }

    distance
}

fn calc_similarity_with_hashmap(list1: &Vec<u32>, list2: &Vec<u32>) -> u32 {
    let mut similarity: u32 = 0;
    let mut occurence_count: HashMap<u32, u32> = HashMap::new();

    let mut index_list2: usize = 0;
    let len_list2 = list2.len();

    let mut cur_entry_hashmap;

    'loop_elem1: for elem in list1 {
        if index_list2 == len_list2 {
            break;
        }

        //println!("{elem}");
        match occurence_count.entry(*elem) {
            HashMapEntry::Occupied(entry) => {
                similarity += elem * entry.get();
                //println!("\t {:?}", similarity);
                continue;
            },
            HashMapEntry::Vacant(entry) => {
                entry.insert(0);
            },
        }

        match occurence_count.entry(*elem) {
            HashMapEntry::Occupied(entry) => {
                cur_entry_hashmap = entry;
            },
            _ => panic!(),
        }

        if index_list2 == len_list2 {
            continue;
        }

        for i in index_list2..len_list2 {
            let elem_list2: u32 = *list2.get(i).unwrap();
            match elem_list2.cmp(elem) {
                Ordering::Greater => {
                    //println!("{elem_list2} > {elem}");
                    similarity += elem * cur_entry_hashmap.get();
                    continue 'loop_elem1
                },
                Ordering::Less => {
                    //println!("{elem_list2} < {elem}");
                    ()
                },
                Ordering::Equal => {
                    //println!("{elem_list2} == {elem}");
                    *cur_entry_hashmap.get_mut() += 1;
                },
            }

            index_list2 +=1;
        }

    }

    similarity
}


fn calc_similarity(list1: &Vec<u32>, list2: &Vec<u32>) -> u32 {
    let mut similarity: u32 = 0;

    let mut index_list2: usize = 0;
    let len_list2 = list2.len();

    let mut last_elem1 = 0;
    let mut last_similarity = 0;

    'loop_elem1: for elem1 in list1 {
        if index_list2 == len_list2 {
            break;
        }

        if *elem1 == last_elem1 {
            similarity += elem1 * last_similarity;
            continue;
        }

        last_similarity = 0;

        for i in index_list2..len_list2 {
            let elem_list2: u32 = *list2.get(i).unwrap();
            match elem_list2.cmp(elem1) {
                Ordering::Greater => {
                    similarity += elem1 * last_similarity;
                    continue 'loop_elem1
                },
                Ordering::Less => {
                    ()
                },
                Ordering::Equal => {
                    last_similarity += 1;
                    last_elem1 = *elem1;
                },
            }

            index_list2 +=1;
        }
    }

    similarity
}

fn main() {
    //let input_file = String::from("example.txt");
    let input_file = String::from("input.txt");

    let (list1, list2) = get_sorted_lists_from_file(&input_file);

    let distance = calc_distance(&list1, &list2);
    println!("Distance: {distance}");

    let similarity = calc_similarity_with_hashmap(&list1, &list2);
    println!("SimilarityV1: {similarity}");

    let similarity = calc_similarity(&list1, &list2);
    println!("SimilarityV2: {similarity}"); // time 0.267
}
