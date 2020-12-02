use std::fs;
use std::collections::{HashSet};
use std::iter::FromIterator;

fn main() {
    report_repair_1();
}

fn report_repair_1() {
    let lines = fs::read_to_string("inputs/1.txt")
        .expect("did not find file!");
    let numbers: Vec<i32> = lines
        .lines()
        .into_iter()
        .map(|num| {
            return num.parse().unwrap();
        })
        .collect();
    println!("list:\n{:?}", numbers);

    let mut numbers =
        HashSet::<i32>::from_iter(numbers.iter().cloned());

    let total = 2020;
    println!("---TWO NUMBERS---");
    for val in numbers.iter() {
        let remainder = total - val;
        if numbers.contains(&remainder) {
            println!("numbers: {} and {}", val, remainder);
            println!("multiply both: {}", val * remainder);
            break;
        }
    }
}