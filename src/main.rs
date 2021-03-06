// surely there must be a more efficient way to do this
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::fs::File;
use std::io::{prelude::*, SeekFrom};
use std::process::exit;

fn reset(file: &mut File) {
    file.seek(SeekFrom::Start(0)).unwrap();
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 2 || argv.len() > 3 {
        println!("usage: advent_of_code day [input_file]");
        exit(1);
    }

    let path: String;
    if argv.len() == 3 {
        // user provided path
        path = argv[2].clone();
    } else {
        // user provided only day
        path = format!("src/day{}/input", &argv[1]);
    }

    match &argv[1].parse::<u8>() {
        Ok(i) => match File::open(&path) {
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
                9 => day9::answer(f),
                10 => day10::answer(f),
                11 => day11::answer(f),
                12 => day12::answer(f),
                13 => day13::answer(f),
                14 => day14::answer(f),
                15 => day15::answer(f),
                16 => day16::answer(f),
                17 => day17::answer(f),
                18 => day18::answer(f),
                19 => day19::answer(f),
                _ => {
                    println!("unknown day '{}'", i);
                    exit(1);
                }
            },
            _ => {
                println!("cannot open file '{}'", &path);
                exit(1);
            }
        },
        _ => {
            println!("unknown day '{}'\nplease use a number such as 1", argv[1]);
            exit(1);
        }
    }
}
