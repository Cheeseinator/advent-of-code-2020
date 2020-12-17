use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod part1;
mod part2;

pub fn answer(f: File) {
    let mut cube_3d = HashSet::new();
    let mut cube_4d = HashSet::new();
    for (y, line) in BufReader::new(f).lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                cube_3d.insert((x as i64, y as i64, 0));
                cube_4d.insert((x as i64, y as i64, 0, 0));
            }
        }
    }
    for _i in 0..6 {
        #[cfg(debug_assertions)]
        eprintln!("iteration {}", _i);
        cube_3d = part1::next(cube_3d);
        cube_4d = part2::next(cube_4d);
    }
    println!("part 1 answer: {}", cube_3d.len());
    println!("part 2 answer: {}", cube_4d.len());
}
