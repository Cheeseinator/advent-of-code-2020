use regex::Regex;

use std::fs::File;
use std::io::{BufRead, BufReader};

const FULL: &str = r"(\w+ \w+) bags contain (.*)";
const BAGS: &str = r"(\d+) (\w+ \w+) bags?[,.] ?";
const EMPTY: &str = r"^(\w+ \w+) bags contain no other bags\.$";

type Contains = Vec<(u8, String)>;

struct Bag {
    name: String,
    contains: Contains,
}

fn search1(bag: &Bag, known: &Vec<Bag>) -> u16 {
    let mut count = 0;
    for c in &bag.contains {
        if c.1 == "shiny gold" {
            count += 1;
        } else {
            for b in known {
                if b.name == c.1 {
                    count += search1(b, known);
                }
            }
        }
    }
    count
}

fn search2(bag: &Bag, known: &Vec<Bag>) -> u64 {
    let mut count = 0;
    for c in &bag.contains {
        count += c.0 as u64;
        for b in known {
            if b.name == c.1 {
                count += search2(b, known) * c.0 as u64;
            }
        }
    }
    count
}

pub fn answer(file: File) {
    let full_regex = Regex::new(FULL).unwrap();
    let empty_regex = Regex::new(EMPTY).unwrap();
    let bags_regex = Regex::new(BAGS).unwrap();
    let mut known_bags: Vec<Bag> = Vec::new();
    let lines = BufReader::new(file).lines();
    for line_ in lines {
        let line = &line_.unwrap();
        if empty_regex.is_match(line) {
            known_bags.push(Bag {
                contains: Vec::new(),
                name: empty_regex.captures(line).unwrap()[1].to_string(),
            });
            continue;
        }
        let full = full_regex.captures(line).unwrap();
        let mut contains: Contains = Contains::new();
        for c in bags_regex.captures_iter(full.get(2).unwrap().as_str()) {
            contains.push((c[1].parse::<u8>().unwrap(), c[2].to_string()));
        }
        known_bags.push(Bag {
            name: full[1].to_string(),
            contains,
        });
    }
    let mut answer1 = 0;
    for bag in &known_bags {
        if search1(bag, &known_bags) > 0 {
            answer1 += 1
        }
    }
    println!("part 1 answer: {}", answer1);
    let mut answer2 = 0;
    for bag in &known_bags {
        if bag.name == "shiny gold" {
            answer2 = search2(&bag, &known_bags);
        }
    }
    println!("part 2 answer: {}", answer2);
}
