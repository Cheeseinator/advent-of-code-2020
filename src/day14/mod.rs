use itertools::Itertools;
use regex::Regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::str::FromStr;

const REGEX: &str = r"mem\[(\d+)\] = (\d+)";

struct Bitmask {
    mask: Vec<char>,
    len: usize,
}

#[derive(Debug)]
struct Unused;

impl FromStr for Bitmask {
    type Err = Unused;
    fn from_str(s: &str) -> Result<Bitmask, Self::Err> {
        let s: Vec<&str> = s.split(" = ").collect();
        Ok(Bitmask {
            mask: s[1].chars().collect(),
            len: s[1].len(),
        })
    }
}

impl Bitmask {
    fn empty() -> Bitmask {
        Bitmask {
            mask: Vec::new(),
            len: 0,
        }
    }
    // part 1
    fn decode(&self, x: u64) -> u64 {
        let mut b: Vec<char> = format!("{:0l$b}", x, l = self.len).chars().collect();
        for i in 0..b.len() {
            match self.mask[i] {
                'X' => continue,
                '0' => b[i] = '0',
                '1' => b[i] = '1',
                _ => unreachable!(),
            }
        }
        u64::from_str_radix(&String::from_iter(b), 2).unwrap()
    }
    // part 2
    fn write(&self, mem: &mut HashMap<u64, u64>, addr: u16, x: u64) {
        let mut b: Vec<char> = format!("{:0l$b}", addr, l = self.len).chars().collect();
        let mut mutations = 0;
        for i in 0..b.len() {
            match self.mask[i] {
                '0' => continue,
                '1' => b[i] = '1',
                'X' => {
                    b[i] = 'X';
                    mutations += 1
                }
                _ => unreachable!(),
            }
        }
        for a in "10"
            .repeat(5) // generate enough combinations
            .chars()
            .combinations_with_replacement(mutations)
            .unique()
        {
            let mut m = b.clone();
            for c in &a {
                for ii in 0..b.len() {
                    if m[ii] == 'X' {
                        m[ii] = *c;
                        break;
                    }
                }
            }
            mem.insert(u64::from_str_radix(&String::from_iter(m), 2).unwrap(), x);
        }
    }
}

fn part_1(lines: &Vec<String>) {
    let mut b = Bitmask::empty();
    let mut mem = HashMap::new();
    let re = Regex::new(REGEX).unwrap();
    for line in lines {
        if line.starts_with("mask") {
            b = line.parse().unwrap();
        } else {
            let cap = re.captures(&line).unwrap();
            let addr: u16 = cap[1].parse().unwrap();
            mem.insert(addr, b.decode(cap[2].parse().unwrap()));
        }
    }
    let mut total = 0;
    for (_, v) in mem {
        total += v;
    }
    println!("part 1 answer: {}", total);
}

fn part_2(lines: &Vec<String>) {
    let mut b = Bitmask::empty();
    let mut mem = HashMap::new();
    let re = Regex::new(REGEX).unwrap();
    for line in lines {
        if line.starts_with("mask") {
            b = line.parse().unwrap();
        } else {
            let cap = re.captures(&line).unwrap();
            let addr: u16 = cap[1].parse().unwrap();
            b.write(&mut mem, addr, cap[2].parse().unwrap());
        }
    }
    let mut total = 0;
    for (_, v) in mem {
        total += v;
    }
    println!("part 2 answer: {}", total);
}

pub fn answer(f: File) {
    let mut lines = Vec::new();
    for line_ in BufReader::new(f).lines() {
        lines.push(line_.unwrap());
    }
    part_1(&lines);
    part_2(&lines);
}
