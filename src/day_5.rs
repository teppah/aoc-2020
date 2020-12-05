use std::fs;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Eq, Debug)]
pub struct Seat<'a> {
    row_partition: &'a str,
    col_partition: &'a str,
    row: usize,
    col: usize,
    seat_id: usize,
}

impl Display for Seat<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(id:{}, row:{}, col: {})", self.seat_id, self.row, self.col)
    }
}

impl<'a> Seat<'a> {
    pub fn from_str(row_partition: &'a str, col_partition: &'a str) -> Seat<'a> {
        let row = Seat::find_num(row_partition);
        let col = Seat::find_num(col_partition);
        let seat_id = row * 8 + col;
        Seat {
            row_partition,
            col_partition,
            row,
            col,
            seat_id,
        }
    }

    fn find_num(s: &str) -> usize {
        let mut val: usize = s.chars()
            .rev()
            .enumerate()
            .map(|(i, c, )| {
                return if c == 'B' || c == 'R' {
                    if i == 0 {
                        1
                    } else {
                        2 << i - 1
                    }
                } else { 0 };
            })
            .sum();
        val
    }
}

impl Ord for Seat<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.seat_id.cmp(&other.seat_id)
    }
}

impl PartialOrd for Seat<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Seat<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.seat_id == other.seat_id
    }
}

impl Hash for Seat<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.seat_id);
    }
}


pub fn binary_boarding() {
    let input = fs::read_to_string("inputs/5.txt")
        .unwrap();
    let values: HashMap<usize, Seat> = input.lines()
        .map(|line| {
            let (row_str, col_str) = line.split_at(7);
            Seat::from_str(row_str, col_str)
        })
        .map(|seat| (seat.seat_id, seat))
        .collect();
    let max = values.iter().max().unwrap();
    println!("Max is: {}", max.1);

    for id in 1..=*max.0 {
        let higher = values.get(&(id + 1));
        let lower = values.get(&(id - 1));
        let mine = values.get(&id);
        if let (None, Some(l), Some(h)) = (mine, lower, higher) {
            println!("My seat_id is: {}", id);
            println!("Above: {}, under: {}", h, l);
        }
    }

    let (mine, before, after) = (1..*max.0)
        .map(|i|
            (i, values.get(&i), values.get(&(i - 1)), values.get(&(i + 1)))
        )
        .filter(|(_, mine, before, after)|
            mine.is_none() && before.is_some() && after.is_some())
        .map(|(i, _, before, after)|
            (i, before.unwrap(), after.unwrap()))
        .next().unwrap();
    println!("Functional: id={}", mine);
}