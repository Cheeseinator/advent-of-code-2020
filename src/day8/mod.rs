use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

enum Oper {
    Accumulate(i32),
    Jump(i32),
    Nop(),
}

impl FromStr for Oper {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp: Vec<&str> = s.split(" ").collect();
        let num = sp[1].parse::<i32>()?;
        Ok(match sp[0] {
            "acc" => Self::Accumulate(num),
            "jmp" => Self::Jump(num),
            _ => Self::Nop(),
        })
    }
}

type Program = HashMap<usize, Oper>;

fn run(p: &Program, ignore: i32) -> (bool, i32) {
    let mut ptr: i32 = 0;
    let mut acc = 0;
    let mut exec: HashSet<i32> = HashSet::new();
    loop {
        let op = &p[&(ptr as usize)];
        if !exec.insert(ptr) {
            return (false, acc);
        } else if ptr == p.len() as i32 - 1 {
            return (true, acc);
        }

        match op {
            Oper::Accumulate(i) => acc += i,
            Oper::Jump(i) => {
                if ptr != ignore {
                    ptr += *i as i32
                } else {
                    ptr += 1
                }
            }
            Oper::Nop() => {}
        }

        if let Oper::Jump(_) = op {
            // increment the instruction pointer by 1 if we didn't run into a jmp instruction
        } else {
            ptr += 1
        }
    }
}

pub fn answer(f: File) {
    let mut program = Program::new();

    for (i, line_) in BufReader::new(f).lines().enumerate() {
        let line = line_.unwrap();
        program.insert(i, line.parse::<Oper>().unwrap());
    }

    println!("part 1 answer: {}", run(&program, -1).1);
    for i in 0..program.len() {
        let res = run(&program, i as i32);
        if res.0 {
            println!("part 2 answer: {}", res.1);
            return;
        }
    }
}
