use std::fs;
use std::collections::{HashSet};
use std::iter::FromIterator;

fn main() {
    report_repair_1();
}

fn report_repair_1() {
    let lines = fs::read_to_string("inputs/1.txt")
        .expect("did not find file!");
    let list: Vec<i32> = lines
        .lines()
        .into_iter()
        .map(|num| {
            return num.parse().unwrap();
        })
        .collect();
    println!("list:\n{:?}", list);

    let mut numbers =
        HashSet::<i32>::from_iter(list.iter().cloned());

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
    println!("---THREE NUMBERS---");

    'outer: for first_val in list.iter() {
        let remainder = total - first_val;
        for second_val in list.iter() {
            if second_val == first_val {
                continue;
            }
            let new_remainder = remainder - second_val;
            if numbers.contains(&new_remainder) {
                println!("vals are {}, {} and {}", first_val, second_val, new_remainder);
                println!("add to {}", first_val + second_val + new_remainder);
                println!("multiply: {}", first_val * second_val * new_remainder);
                break 'outer;
            }
        }
    }
}