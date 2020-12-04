use std::fs;
use std::collections::HashSet;
use regex::Regex;

pub fn passport_processing() {
    let input = fs::read_to_string("inputs/4.txt").unwrap();
    let required: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .collect();
    let fields = Regex::new(r"(\w{3}):").unwrap();
    let count: usize = input.split("\n\n")
        .map(|passport: &str| {
            let captures: HashSet<&str> = fields
                .captures_iter(passport)
                .map(|cap| cap.get(1).unwrap().as_str())
                .collect();
            captures
        })
        .filter(|fields| fields.is_superset(&required))
        .count();
    println!("Valid: {}", count);
}