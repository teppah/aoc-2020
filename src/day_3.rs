use std::fs;
use itertools::Itertools;

pub fn toboggan_trajectory() {
    let file = fs::read_to_string("inputs/3.txt").expect("failed to read");
    const TREE: char = '#';
    const PATH: char = '.';
    const WIDTH: usize = 31;

    let mut index: usize = 0;
    let mut count = 0;
    for line in file.lines() {
        match line.chars().nth(index) {
            Some(TREE) => count += 1,
            _ => ()
        }
        index = (index + 3) % WIDTH;
    }
    println!("Total trees: {}", count);

    let (a, b, c, d, e) = file.lines()
        .enumerate()
        .map(|(i, line)| {
            // one
            let count1 = match_char(line, i % WIDTH, TREE);

            // three
            let count2 = match_char(line, i * 3 % WIDTH, TREE);

            // five
            let count3 = match_char(line, i * 5 % WIDTH, TREE);

            // seven
            let count4 = match_char(line, i * 7 % WIDTH, TREE);

            // down 2
            let count5: usize = if i % 2 == 1 {
                0
            } else {
                match_char(line, i / 2 % WIDTH, TREE)
            };
            (count1, count2, count3, count4, count5)
        })
        .fold((0, 0, 0, 0, 0),
              |(a, b, c, d, e),
               (f, g, h, i, j)| {
                  (a + f, b + g, c + h, d + i, e + j)
              })
        ;
    println!("Product: {}", a * b * c * d * e);
}

fn match_char(line: &str, index: usize, c: char) -> usize {
    if line.chars().nth(index) == Some(c) { 1 } else { 0 }
}