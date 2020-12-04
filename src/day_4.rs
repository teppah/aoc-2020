use std::fs;
use std::collections::{HashSet, HashMap};
use regex::Regex;
use itertools::Itertools;


pub fn passport_processing() {
    let input = fs::read_to_string("inputs/4.txt").unwrap();
    let required: HashSet<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .into_iter()
        .collect();
    let fields = Regex::new(r"(\w{3}):").unwrap();
    let first_valid: usize = input.split("\n\n")
        .map(|passport: &str| {
            let captures: HashSet<&str> = fields
                .captures_iter(passport)
                .map(|cap| cap.get(1).unwrap().as_str())
                .collect();
            captures
        })
        .filter(|fields| fields.is_superset(&required))
        .count();
    println!("Valid: {}", first_valid);

    let second_valid: usize = input.split("\n\n")
        .map(|passport: &str| {
            let fields: HashMap<&str, &str> = passport
                .split_whitespace()
                .map(|entry| {
                    let mut split: (&str, &str) = entry
                        .split(":")
                        .next_tuple()
                        .unwrap();
                    split
                })
                .collect();
            // println!("lol: -- {:?}", fields);
            fields
        })
        .filter(|val| {
            return check_valid(val);
        })
        .count();
    println!("Valid: {}", second_valid);
}

fn check_valid(vals: &HashMap<&str, &str>) -> bool {
    let mut valid: usize = 0;
    const EXPECTED_VALID: usize = 7;
    for (name, val) in vals.iter() {
        match *name {
            "byr" => {
                if !valid_byr(*val) {
                    return false;
                }
                valid += 1;
            }
            "iyr" => {
                if !valid_iyr(*val) {
                    return false;
                }
                valid += 1;
            }
            "eyr" => {
                if !valid_eyr(*val) {
                    return false;
                }
                valid += 1;
            }
            "hgt" => {
                if !valid_hgt(*val) {
                    return false;
                }
                valid += 1;
            }
            "hcl" => {
                if !valid_hcl(*val) {
                    return false;
                }
                valid += 1;
            }
            "ecl" => {
                if !valid_ecl(*val) {
                    return false;
                }
                valid += 1;
            }
            "pid" => {
                if !valid_pid(*val) {
                    return false;
                }
                valid += 1;
            }
            _ => ()
        }
    }
    return valid == EXPECTED_VALID;
}

fn valid_byr(val: &str) -> bool {
    if val.len() != 4 {
        return false;
    }
    let parse: Result<usize, _> = val.parse();
    return if let Ok(number) = parse {
        number >= 1920 && number <= 2002
    } else {
        false
    };
}

fn valid_iyr(val: &str) -> bool {
    if val.len() != 4 {
        return false;
    }
    let parse: Result<usize, _> = val.parse();
    return if let Ok(number) = parse {
        number >= 2010 && number <= 2020
    } else {
        false
    };
}

fn valid_eyr(val: &str) -> bool {
    if val.len() != 4 {
        return false;
    }
    let parse: Result<usize, _> = val.parse();
    return if let Ok(number) = parse {
        number >= 2020 && number <= 2030
    } else {
        false
    };
}

fn valid_hgt(val: &str) -> bool {
    let parsed: usize = match val[..val.len() - 2].parse() {
        Ok(val) => val,
        Err(_) => { return false; }
    };
    return if val.contains("cm") {
        parsed >= 150 && parsed <= 193
    } else if val.contains("in") {
        parsed >= 59 && parsed <= 76
    } else {
        false
    };
}

fn valid_hcl(val: &str) -> bool {
    if !val.contains("#") {
        return false;
    }
    let count = val.chars()
        .filter(|c|
            (48..=57).contains(&(*c as i32)) || (97..=102).contains(&(*c as i32)))
        .count();
    return count == 6;
}

fn valid_ecl(val: &str) -> bool {
    let eyes: HashSet<&str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].into_iter().collect();
    return eyes.contains(val);
}

fn valid_pid(val: &str) -> bool {
    if val.len() != 9 {
        return false;
    }
    let parsed: Result<i32, _> = val.parse();
    return if let Ok(_) = parsed {
        true
    } else {
        false
    };
}