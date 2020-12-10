use std::fs;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug)]
struct BagInfo<'a> {
    amount: u32,
    variant: &'a str,
}

pub fn handy_haversacks() {
    let empty_bag = Regex::new(r"no other").unwrap();
    let origin_bag = Regex::new(r"(\w* \w*) bags contain").unwrap();
    let target_bag = Regex::new(r"(\d*) (\w* \w*) bag").unwrap();

    let input = fs::read_to_string("inputs/7.txt").unwrap();

    let containers: HashMap<&str, HashSet<&str>> = input.lines()
        .map(|line| {
            let origin = origin_bag.captures(line).unwrap();
            let origin = origin.get(1).unwrap().as_str();
            return if !empty_bag.is_match(line) {
                let destinations: HashSet<&str> = target_bag.captures_iter(line)
                    .map(|cap| cap.get(2).unwrap().as_str())
                    .collect();
                (origin, destinations)
            } else {
                (origin, HashSet::new())
            };
        })
        .collect();

    let mut status_cache: HashMap<&str, bool> = HashMap::new();
    let contains = containers.keys()
        .map(|bag| check_color(bag, &containers, &mut status_cache))
        .filter(|b| *b)
        .count();
    println!("Number: {}", contains);
}

const TARGET: &'static str = "shiny gold";

fn check_color<'a>(color: &'a str,
                   map: &HashMap<&'a str, HashSet<&'a str>>,
                   cache: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(status) = cache.get(color) {
        return *status;
    }
    let dests = map.get(color).unwrap();
    if dests.contains(TARGET) {
        cache.insert(color, true);
        return true;
    }
    for dest in dests {
        if check_color(dest, map, cache) {
            return true;
        }
    }
    cache.insert(color, false);
    return false;
}