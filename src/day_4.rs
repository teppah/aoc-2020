use std::fs;
use std::collections::{HashSet, HashMap};
use regex::Regex;
use itertools::Itertools;
use std::thread;

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
            fields
        })
        .filter(|val| {
            return check_valid(val);
        })
        .count();
    println!("Valid: {}", second_valid);
}

fn check_valid(vals: &HashMap<&str, &str>) -> bool
{
    type F = for<'r> fn(&'r str) -> bool;
    const EXPECTED_VALID: usize = 7;
    let validators: HashMap<&str, F> =
        vec![
            ("byr", valid_byr as F),
            ("iyr", valid_iyr),
            ("eyr", valid_eyr),
            ("hgt", valid_hgt),
            ("hcl", valid_hcl),
            ("ecl", valid_ecl),
            ("pid", valid_pid)
        ].into_iter()
            .collect();
    let valid: usize = vals.iter()
        .filter(|(k, v)| {
            return if let Some(validator) = validators.get(*k) {
                validator(*v)
            } else { false };
        })
        .count();
    return valid == EXPECTED_VALID;
}

fn valid_byr(val: &str) -> bool {
    if val.len() != 4 {
        return false;
    }
    let parse: Result<i32, _> = val.parse();
    return if let Ok(number) = parse {
        (1920..=2002).contains(&number)
    } else {
        false
    };
}

fn valid_iyr(val: &str) -> bool {
    if val.len() != 4 {
        return false;
    }
    let parse: Result<i32, _> = val.parse();
    return if let Ok(number) = parse {
        (2010..=2020).contains(&number)
    } else {
        false
    };
}

fn valid_eyr(val: &str) -> bool {
    if val.len() != 4 {
        return false;
    }
    let parse: Result<i32, _> = val.parse();
    return if let Ok(number) = parse {
        (2020..=2030).contains(&number)
    } else {
        false
    };
}

fn valid_hgt(val: &str) -> bool {
    let parsed: i32 = match val[..val.len() - 2].parse() {
        Ok(val) => val,
        Err(_) => { return false; }
    };
    return if val.contains("cm") {
        (150..=193).contains(&parsed)
    } else if val.contains("in") {
        (59..=76).contains(&parsed)
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