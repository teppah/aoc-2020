use std::fs;
use std::collections::HashSet;
use itertools::Itertools;

pub fn custom_customs() {
    let lines = fs::read_to_string("inputs/6.txt").unwrap();

    let count: usize = lines.split("\n\n")
        .map(|entry| {
            let chars: HashSet<char> = entry
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            return chars.len();
        })
        .sum();
    println!("Total of counts: {}", count);
}