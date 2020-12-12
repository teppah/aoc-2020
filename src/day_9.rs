use std::fs;
use std::collections::VecDeque;

pub fn encoding_error() {
    let lines = fs::read_to_string("inputs/9.txt").unwrap();
    let mut current_numbers: VecDeque<i64> = lines.lines()
        .take(25)
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    println!("{:?}", current_numbers);
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
}