use std::fs::File;
use std::io::{BufRead, BufReader};

mod part1;
mod part2;

pub fn answer(f: File) {
    let mut lines = Vec::new();
    for line in BufReader::new(f).lines() {
        lines.push(line.unwrap());
    }
    println!("part 1 answer: {}", part1::answer(&lines));
    println!("part 2 answer: {}", part2::answer(&lines));
}
