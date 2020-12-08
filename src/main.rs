mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::fs::File;
use std::io::{prelude::*, SeekFrom};
use std::process::exit;

fn reset(file: &mut File) {
    file.seek(SeekFrom::Start(0)).unwrap();
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() != 3 {
        println!("usage: advent_of_code day /path/to/input");
        exit(1);
    }

    match &argv[1].parse::<u8>() {
        Ok(i) => match File::open(&argv[2]) {
            Ok(mut f) => match i {
                1 => day1::answer(f),
                2 => {
                    day2::part_1(&f);
                    reset(&mut f);
                    day2::part_2(&f);
                }
                3 => day3::answer(f),
                4 => {
                    day4::part_1(&f);
                    reset(&mut f);
                    day4::part_2(&f);
                }
                5 => day5::answer(f),
                6 => day6::answer(f),
                7 => day7::answer(f),
                8 => day8::answer(f),
                _ => {
                    println!("unknown day '{}'", i);
                    exit(1);
                }
            },
            _ => {
                println!("cannot open file '{}'", argv[2]);
                exit(1);
            }
        },
        _ => {
            println!("unknown day '{}'\nplease use a number such as 1", argv[1]);
            exit(1);
        }
    }
}
