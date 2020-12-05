use std::fs;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};

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

enum Pos {
    UPPER,
    LOWER,
}

impl<'a> Seat<'a> {
    pub fn from_str(row_partition: &'a str, col_partition: &'a str) -> Seat<'a> {
        let row = Seat::find_num(0, 127, row_partition);
        let col = Seat::find_num(0, 7, col_partition);
        let seat_id = row * 8 + col;
        Seat {
            row_partition,
            col_partition,
            row,
            col,
            seat_id,
        }
    }

    fn get_half(c: char) -> Pos {
        match c {
            'F' | 'L' => Pos::LOWER,
            'B' | 'R' => Pos::UPPER,
            _ => panic!("Unknown letter: {}", c)
        }
    }

    fn find_num(lower: usize, higher: usize, s: &str) -> usize {
        let mut lower = lower;
        let mut higher = higher;
        for c in s.chars() {
            let mid = (lower + higher) as f64 / 2f64;
            match Seat::get_half(c) {
                Pos::LOWER => {
                    higher = mid.floor() as usize;
                }
                Pos::UPPER => {
                    lower = mid.ceil() as usize;
                }
            }
        }
        lower
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


pub fn binary_boarding() {
    let input = fs::read_to_string("inputs/5.txt")
        .unwrap();
    let max = input.lines()
        .map(|line| {
            let (row_str, col_str) = line.split_at(7);
            Seat::from_str(row_str, col_str)
        })
        .max()
        .unwrap()
        ;
    println!("max is: {}", max);
}