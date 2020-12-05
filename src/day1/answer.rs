use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let mut vec: Vec<i32> = Vec::new();

    for _line in lines {
        let line = _line.unwrap();
        vec.push(line.parse::<i32>().unwrap());
    }

    let len = vec.len();

    for i in 0..len {
        for j in 0..len {
            if vec[i] + vec[j] == 2020 {
                println!("Answer: {}", vec[i] * vec[j]);
                return;
            }
        }
    }
}
