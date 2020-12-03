use std::fs;
use itertools::Itertools;
use regex::Regex;

pub fn password_philosophy() {
    let input = fs::read_to_string("inputs/2.txt").expect("did not find file");

    let matcher = Regex::new(r"(\d{1,2})-(\d{1,2}) (\w): (\w*)").unwrap();

    let (first_result, second_result): (usize, usize) = input.lines()
        .map(|line| {
            let captures = matcher.captures(line).unwrap();

            let low: usize = captures[1].parse().unwrap();
            let high: usize = captures[2].parse().unwrap();
            let letter: char = captures[3].parse().unwrap();
            let sequence: &str = line.split_whitespace().last().unwrap();
            (low, high, letter, sequence)
        })
        .map(|(low, high, letter, seq)| {
            let mut count: usize = seq
                .chars()
                .filter(|c| c == &letter)
                .count();
            let mut chars = seq.chars();
            let mut has_valid_indexes: bool =
                (chars.nth(low - 1) == Some(letter)) ^ (chars.nth(high - 1 - low) == Some(letter));
            return if count >= low && count <= high {
                (1 as usize, has_valid_indexes as usize)
            } else {
                (0 as usize, has_valid_indexes as usize)
            };
        })
        .fold((0, 0), |(a, b), (c, d)| {
            (a + c, b + d)
        });

    println!("part 1: {}", first_result);
    println!("part 2: {}", second_result);

    //     let mut splits = line.split_whitespace();
    //     let bounds = splits.next().unwrap();
    // }
}