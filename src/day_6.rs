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

    let total_common: usize = lines.split("\n\n")
        .map(|entry| {
            entry.lines()
                .map(|line| {
                    line.chars().collect::<HashSet<char>>()
                })
                .fold1(|mut set1, set2| {
                    set1.retain(|c| set2.contains(c));
                    set1
                })
                .unwrap()
        })
        .map(|common| common.len())
        .sum();
    println!("Total common: {}", total_common);
}