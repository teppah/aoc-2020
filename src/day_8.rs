use std::fs;
use std::io::Lines;
use std::collections::HashSet;

pub fn handheld_halting() {
    let input = fs::read_to_string("inputs/8.txt").unwrap();


    let lines: Vec<(&str, i32)> = input.lines()
        .map(|line| line.split_at(3))
        .map(|(a, b)|
            (a, b.trim().parse().unwrap())
        )
        .collect();

    let (accumulator, index) = check_termination(&lines, None);
    println!("Accumulator value: {}, line index: {}", accumulator, index);

    // assume there is only one
    let (accumulator, _) = (0..lines.len())
        .map(|index| check_termination(&lines, Some(index)))
        .filter(|(_, index)| *index == lines.len())
        .next()
        .unwrap();
    println!("Accumulator after the program terminates: {}", accumulator);


    // for (index, line) in lines.iter().enumerate() {
    //     let result = check_termination(&lines, Some(index));
    //     if result.1 == lines.len() {
    //         break;
    //     }
    // }
}

fn flip(s: &str) -> Option<&'static str> {
    match s {
        "nop" => Some("jmp"),
        "jmp" => Some("nop"),
        _ => None
    }
}


// (accumulator, index)
fn check_termination(lines: &[(&str, i32)], line_replace: Option<usize>) -> (i32, usize) {
    let mut index: usize = 0;
    let mut accumulator: i32 = 0;
    let mut visited: HashSet<usize> = HashSet::new();

    while !visited.contains(&index) && index < lines.len() {
        let (inst, val) = lines[index];
        let inst = match (line_replace, index, flip(inst)) {
            (Some(target), i, Some(replacement)) if target == i => replacement,
            _ => inst
        };

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
    (accumulator, index)
}
