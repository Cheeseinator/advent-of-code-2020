use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn check(set: &HashSet<String>, c: char) -> bool {
    for s in set {
        if !s.contains(c) {
            return false;
        }
    }
    true
}

fn main() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line_ in reader.lines() {
        lines.push(line_.unwrap());
    }
    lines.push(String::new()); // Add an empty string to catch EOF

    let mut s1 = HashSet::new();
    let mut s2 = HashSet::new();
    let mut a1 = 0;
    let mut a2 = 0;
    for line in lines {
        if line == "" {
            for c in &s1 {
                if check(&s2, *c) {
                    a2 += 1;
                }
            }

            s1.clear();
            s2.clear();
        } else {
            let mut p = String::new();
            for c in line.chars() {
                p.push(c);

                if s1.insert(c) {
                    a1 += 1;
                }
            }
            s2.insert(p);
        }
    }

    println!("Part 1 answer: {}", a1);
    println!("Part 2 answer: {}", a2);
}
