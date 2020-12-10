use std::fs;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};


type Destinations<'a> = HashMap<&'a str, u32>;

pub fn handy_haversacks() {
    let empty_bag = Regex::new(r"no other").unwrap();
    let origin_bag = Regex::new(r"(\w* \w*) bags contain").unwrap();
    let target_bag = Regex::new(r"(\d*) (\w* \w*) bag").unwrap();

    let input = fs::read_to_string("inputs/7.txt").unwrap();


    let containers: HashMap<&str, Destinations> = input.lines()
        .map(|line| {
            let origin = origin_bag.captures(line).unwrap();
            let origin = origin.get(1).unwrap().as_str();
            return if !empty_bag.is_match(line) {
                let destinations: Destinations = target_bag.captures_iter(line)
                    .map(|cap| {
                        let count: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
                        let val = cap.get(2).unwrap().as_str();
                        (val, count)
                    })
                    .collect();
                (origin, destinations)
            } else {
                (origin, HashMap::new())
            };
        })
        .collect();

    let mut status_cache: HashMap<&str, bool> = HashMap::new();
    let contains = containers.keys()
        .map(|bag| check_color(bag, &containers, &mut status_cache))
        .filter(|b| *b)
        .count();
    println!("Number: {}", contains);

    let gold_number = trace(TARGET, &containers);
    println!("Part 2: {}", gold_number - 1);
}

const TARGET: &'static str = "shiny gold";

// assume bags don't contain each other
fn check_color<'a>(color: &'a str,
                   map: &HashMap<&'a str, Destinations<'a>>,
                   cache: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(status) = cache.get(color) {
        return *status;
    }
    let dests = map.get(color).unwrap();
    if dests.contains_key(TARGET) {
        cache.insert(color, true);
        return true;
    }
    for dest in dests {
        if check_color(dest.0, map, cache) {
            return true;
        }
    }
    cache.insert(color, false);
    return false;
}

fn trace(bag: &str, map: &HashMap<&str, Destinations>) -> u32 {
    let dests = map.get(bag).unwrap();
    if dests.is_empty() {
        return 1;
    }
    return 1 + dests.iter()
        .map(|(name, count)| count * trace(name, map))
        .sum::<u32>();
}