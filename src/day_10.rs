use std::collections::HashSet;
use itertools::Itertools;

pub fn adapter_array() {
    let input = std::fs::read_to_string("inputs/10.txt").unwrap();
    let mut joltages: HashSet<i64> = input.lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let max = joltages.iter().cloned().max().unwrap();
    println!("Max: {}", max);

    let mut one_jolt = 0;
    // device already supports 3
    let mut three_jolt = 1;
    let mut current = 0;
    while current < max {
        for i in 1..=3 {
            current += 1;
            if joltages.contains(&current) {
                match i {
                    1 => {
                        one_jolt += 1;
                        break;
                    }
                    3 => {
                        three_jolt += 1;
                        break;
                    }
                    2 => (),
                    _ => panic!("None found for {}", current)
                }
            }
        }
    }
    println!("low: {}, high: {}, product: {}", one_jolt, three_jolt, one_jolt * three_jolt);
    joltages.insert(0);
    let mut joltages =
        joltages.iter().cloned().collect::<Vec<i64>>();
    joltages.sort_unstable();
    let final_result = joltages
        .windows(2)
        .collect::<Vec<_>>()
        .split(|n| n[1] - n[0] == 3)
        .map(|n| match n.len() {
            4 => 7,
            3 => 4,
            2 => 2,
            _ => 1,
        })
        .product::<usize>();
    println!("Final result: {}", final_result);
}