use std::collections::HashSet;

pub fn adapter_array() {
    let input = std::fs::read_to_string("inputs/10.txt").unwrap();
    let joltages: HashSet<i32> = input.lines()
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
}