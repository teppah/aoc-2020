use std::fs;
use std::io::Lines;
use std::collections::HashSet;

pub fn handheld_halting() {
    let input = fs::read_to_string("inputs/8.txt").unwrap();

    let mut index: usize = 0;
    let mut accumulator: i32 = 0;
    let mut visited: HashSet<usize> = HashSet::new();

    while !visited.contains(&index) {
        let line = input.lines().nth(index).unwrap();
        let (inst, val) = line.split_at(3);
        // assume valid
        let val: i32 = val.trim().parse().unwrap();

        match inst {
            "nop" => {
                visited.insert(index);
                index += 1;
            }
            "acc" => {
                visited.insert(index);
                accumulator += val;
                index += 1;
            }
            "jmp" => {
                visited.insert(index);
                index = (val + index as i32) as usize;
            }
            other => panic!("Unknown instruction: {}", other)
        }
    }
    println!("Final accumulator value: {} at index {}", accumulator, index);
}