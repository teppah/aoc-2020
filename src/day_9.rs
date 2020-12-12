use std::collections::VecDeque;
use std::fs;

use itertools::Itertools;

pub fn encoding_error() {
    let lines = fs::read_to_string("inputs/9.txt").unwrap();
    let all_numbers: Vec<i64> = lines.lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let mut current_numbers: VecDeque<i64> = all_numbers.iter()
        .cloned()
        .take(25)
        .collect();
    let found: i64 = lines.lines()
        .skip(25)
        .map(|n| n.parse::<i64>().unwrap())
        .filter(|val| {
            let mut should_stay = true;
            for subtrahend in &current_numbers {
                let result = val - subtrahend;
                if current_numbers.contains(&result) {
                    if subtrahend == &result {
                        continue;
                    } else {
                        should_stay = false;
                        break;
                    }
                }
            }
            current_numbers.pop_front().unwrap();
            current_numbers.push_back(*val);
            return should_stay;
        })
        .next()
        .unwrap();
    println!("Found {}", found);

    for i in 0..(all_numbers.len() - 1) {
        let mut running_sum: i64 = all_numbers[i];
        for j in (i + 1)..all_numbers.len() {
            running_sum += all_numbers[j];
            if running_sum == found {
                println!("Found sequence from indexes {} to {}", i, j);
                let smallest: i64 = all_numbers[i..=j].iter().cloned().max().unwrap();
                let largest: i64 = all_numbers[i..=j].iter().cloned().min().unwrap();
                println!("{} + {} = {}", smallest, largest, smallest + largest);
                break;
            }
        }
    }
}