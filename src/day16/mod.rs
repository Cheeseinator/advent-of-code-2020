use regex::{Captures, Regex};

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

const FIELD_REGEX: &str = r"(.+): (\d+)\-(\d+) or (\d+)\-(\d+)";

type Ranges = (Range<u64>, Range<u64>);

fn parse_field(opt: Option<Captures>) -> (String, Ranges) {
    let captures = opt.unwrap();
    let name = captures[1].to_string();
    let r1: u64 = captures[2].parse().unwrap();
    let r2: u64 = captures[3].parse().unwrap();
    let r3: u64 = captures[4].parse().unwrap();
    let r4: u64 = captures[5].parse().unwrap();
    // ranges are inclusive
    (name, ((r1..r2 + 1), (r3..r4 + 1)))
}

fn parse_ticket(s: String) -> Vec<u64> {
    let mut v = Vec::new();
    for i in s.split(",") {
        v.push(i.parse().unwrap());
    }
    v
}

fn validate(ranges: &Ranges, v: u64) -> bool {
    ranges.0.contains(&v) || ranges.1.contains(&v)
}

fn validate_all(fields: &HashMap<String, Ranges>, v: u64) -> bool {
    for (_, ranges) in fields {
        if validate(&ranges, v) {
            return true;
        }
    }
    false
}

fn valid_tickets(fields: &HashMap<String, Ranges>, nearby: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut rate = 0;
    let mut result: Vec<Vec<u64>> = Vec::new();
    for ticket in nearby {
        let mut valid = true;
        for value in ticket {
            if !validate_all(&fields, *value) {
                rate += value;
                valid = false;
            }
        }
        if valid {
            result.push(ticket.clone());
        }
    }
    println!("part 1 answer: {}", rate);
    result
}

pub fn answer(f: File) {
    let re = Regex::new(FIELD_REGEX).unwrap();
    let mut sections = Vec::new();
    let mut fields = HashMap::new();
    let mut section = Vec::new();
    for line_ in BufReader::new(f).lines() {
        let line = line_.unwrap();
        if line == "" {
            sections.push(section.clone());
            section.clear();
        } else if !line.ends_with(":") {
            section.push(line);
        }
    }
    // handle EOF
    if sections.len() > 0 {
        sections.push(section);
    }
    for line in &sections[0] {
        let field = parse_field(re.captures(&line));
        fields.insert(field.0, field.1);
    }
    let my_ticket = parse_ticket(sections[1][0].to_string());
    let mut tickets = Vec::new();
    for t in &sections[2] {
        tickets.push(parse_ticket(t.to_string()));
    }
    tickets = valid_tickets(&fields, &tickets);
    tickets.push(my_ticket.clone());
    let len = tickets[0].len();
    let mut indices: HashMap<String, Vec<usize>> = HashMap::new();
    for i in (0..len).rev() {
        for (name, ranges) in &fields {
            let mut valid = true;
            for ticket in &tickets {
                if !validate(&ranges, ticket[i]) {
                    valid = false;
                    break;
                }
            }
            if valid {
                let s = String::from(name);
                match indices.remove(name) {
                    Some(mut v) => {
                        v.push(i);
                        indices.insert(s, v);
                    }
                    None => {
                        indices.insert(s, vec![i]);
                    }
                }
            }
        }
    }
    let mut real_indices = HashMap::new();
    while real_indices.len() < len {
        let mut new_indices = indices.clone();
        for (s, a) in &indices {
            if a.len() == 1 {
                let n = a[0];
                real_indices.insert(s.clone(), n);
                for (s, a) in &indices {
                    let mut v = a.clone();
                    for i in 0..v.len() {
                        if v[i] == n {
                            v.remove(i);
                            break;
                        }
                    }
                    new_indices.insert(s.to_string(), v);
                }
                break;
            }
        }
        for (k, v) in new_indices {
            indices.insert(k, v);
        }
    }
    let mut res = 1;
    for (s, i) in real_indices {
        if s.starts_with("departure") {
            res *= my_ticket[i];
        }
    }
    println!("part 2 answer: {}", res);
}
