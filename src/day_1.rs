use std::collections::{HashSet, HashMap};
use std::fs;
use itertools::Itertools;

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

    let possible_sums: HashMap<i32, (i32, i32)> = numbers.iter()
        .cartesian_product(numbers.iter())
        .filter(|(v1, v2)| v1 != v2)
        .map(|(v1, v2)| {
            let sum = *v1 + *v2;
            return (sum, (*v1, *v2));
        })
        .collect();

    let total = 2020;
    if let Some((val1, val2)) = possible_sums.get(&total) {
        println!("{} * {} = {}", val1, val2, val1 * val2);
    }
    // if let Some((val1, val2)) = get_summing_to(&numbers, total) {
    //     println!("{} * {} = {}", val1, val2, val2 * val2);
    // }
    for val in numbers.iter() {
        let remainder = total - val;
        //     if let Some((val1, val2)) = get_summing_to(&numbers, remainder) {
        //         println!("{} * {} * {} = {}", val, val1, val2, val2 * val2);
        //         break;
        //     }
        if let Some((val1, val2)) = possible_sums.get(&remainder) {
            println!("{} * {} * {} = {}", val, val1, val2, val * val1 * val2);
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
