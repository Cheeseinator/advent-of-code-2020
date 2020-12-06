use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn validate(fields: &Vec<String>, required: &Vec<&str>) -> bool {
    for r in required {
        if !fields.contains(&r.to_string()) {
            return false;
        }
    }
    true
}

fn main() {
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let mut fields: Vec<String> = Vec::new();
    let mut answer = 0;

    for _line in lines {
        let line = _line.unwrap();

        if line == "" {
            if validate(&fields, &required) {
                answer += 1;
            }
            fields.clear();
        } else {
            for field in line.split(" ") {
                let (name, _) = field.split_at(3);
                fields.push(name.to_string());
            }
        }
    }

    println!("Answer: {}", answer);
}
