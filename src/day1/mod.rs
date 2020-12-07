use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_1(vec: &Vec<i32>) {
    let len = vec.len();
    for i in 0..len {
        for j in 0..len {
            if vec[i] + vec[j] == 2020 {
                println!("part 1 answer: {}", vec[i] * vec[j]);
                return;
            }
        }
    }
}

fn part_2(vec: &Vec<i32>) {
    let len = vec.len();
    for k in 0..len {
        for l in 0..len {
            for m in 0..len {
                if vec[k] + vec[l] + vec[m] == 2020 {
                    println!("part 2 answer: {}", vec[k] * vec[l] * vec[m]);
                    return;
                }
            }
        }
    }
}

pub fn answer(file: File) {
    let lines = BufReader::new(file).lines();

    let mut vec: Vec<i32> = Vec::new();

    for _line in lines {
        let line = _line.unwrap();
        vec.push(line.parse::<i32>().unwrap());
    }

    part_1(&vec);
    part_2(&vec);
}
