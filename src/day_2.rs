use std::fs;
use itertools::Itertools;

pub fn password_philosophy() {
    let input = fs::read_to_string("inputs/2.txt").expect("did not find file");

    let (first_result, second_result): (usize, usize) = input.lines()
        .map(|line| {
            let mut entries = line.split_whitespace();
            let mut bounds = entries.next().unwrap().split("-");
            let letter = entries.next().unwrap();
            let sequence = entries.next().unwrap();

            let low: usize = bounds.next().unwrap().parse().unwrap();
            let high: usize = bounds.next().unwrap().parse().unwrap();
            let letter: char = letter.chars().nth(0).unwrap();

            (low, high, letter, sequence)
        })
        .map(|(low, high, letter, seq)| {
            let mut count: usize = 0;
            let mut has_valid_indexes: bool = false;
            for (i, c) in seq.chars().enumerate() {
                if c == letter {
                    count += 1;
                    if i + 1 == low || i + 1 == high {
                        has_valid_indexes = !has_valid_indexes;
                    }
                }
            }
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