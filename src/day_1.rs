use std::collections::{HashSet};
use std::fs;

pub fn report_repair() {
    let lines = fs::read_to_string("inputs/1.txt")
        .expect("did not find file!");
    let numbers: HashSet<i32> = lines
        .lines()
        .into_iter()
        .map(|num| {
            return num.parse().unwrap();
        })
        .collect();

    let total = 2020;
    if let Some((val1, val2)) = get_summing_to(&numbers, total) {
        println!("{} * {} = {}", val1, val2, val2 * val2);
    }
    for val in numbers.iter() {
        let remainder = total - val;
        if let Some((val1, val2)) = get_summing_to(&numbers, remainder) {
            println!("{} * {} * {} = {}", val, val1, val2, val2 * val2);
            break;
        }
    }
}

fn get_summing_to(numbers: &HashSet<i32>, target: i32) -> Option<(i32, i32)> {
    for i in numbers.iter() {
        let remainder = target - i;
        if remainder < 0 {
            continue;
        }
        if numbers.contains(&remainder) {
            return Some((*i, remainder));
        }
    }
    None
}
