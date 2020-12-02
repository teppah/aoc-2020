use std::fs;
use itertools::Itertools;

pub fn password_philosophy() {
    let input = fs::read_to_string("inputs/2.txt").expect("did not find file");

    let first_result: u32 = input.lines()
        .map(|line| {
            let mut entries = line.split_whitespace();
            let mut bounds = entries.next().unwrap().split("-");
            let letter = entries.next().unwrap();
            let sequence = entries.next().unwrap();

            let low: u32 = bounds.next().unwrap().parse().unwrap();
            let high: u32 = bounds.next().unwrap().parse().unwrap();
            let letter: char = letter.chars().nth(0).unwrap();

            (low, high, letter, sequence)
        })
        .map(|(low, high, letter, seq)| {
            let mut count: u32 = 0;
            for c in seq.chars() {
                if c == letter {
                    count += 1;
                }
            }
            return if count >= low && count <= high {
                1
            } else {
                0
            };
        })
        .sum();
    println!("part 1:{}", first_result);

    //     let mut splits = line.split_whitespace();
    //     let bounds = splits.next().unwrap();
    // }
}