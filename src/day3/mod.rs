use std::fs::File;
use std::io::{BufRead, BufReader};

fn slope(vec: &Vec<Vec<bool>>, right: usize, down: usize) -> u32 {
    let mut x = 0;
    let mut trees = 0;
    for y in (0..vec.len()).step_by(down) {
        if vec[y][x] {
            trees += 1;
        }
        x += right;
    }

    println!("slope({}, {}) = {}", right, down, trees);
    trees
}

pub fn answer(file: File) {
    let lines = BufReader::new(file).lines();

    let mut map: Vec<Vec<bool>> = Vec::new();

    for _line in lines {
        let line = _line.unwrap().repeat(100); // just to be safe
        let mut row: Vec<bool> = Vec::new();

        for chr in line.chars() {
            row.push(chr == '#')
        }
        map.push(row);
    }

    println!(
        "total: {}",
        slope(&map, 1, 1)
            * slope(&map, 3, 1)
            * slope(&map, 5, 1)
            * slope(&map, 7, 1)
            * slope(&map, 1, 2)
    );
}
