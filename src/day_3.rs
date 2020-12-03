use std::fs;

pub fn toboggan_trajectory() {
    let file = fs::read_to_string("inputs/3.txt").expect("failed to read");

    let width: usize = 31;
    let mut index: usize = 0;
    let mut count = 0;
    const TREE: char = '#';
    const PATH: char = '.';
    for line in file.lines() {
        match line.chars().nth(index) {
            Some(TREE) => count += 1,
            _ => ()
        }
        index = (index + 3) % width;
    }
    println!("Total trees: {}", count);
}