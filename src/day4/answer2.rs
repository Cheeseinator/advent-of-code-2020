use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn get_field(fields: &Vec<String>, name: &str) -> Option<String> {
    for f in fields {
        let mut fsplit = f.split(":");
        let fname = fsplit.next().unwrap();
        let fvalue = fsplit.next().unwrap();

        if fname == name {
            return Some(fvalue.to_string());
        }
    }
    None
}

fn validate(fields: &Vec<String>) -> bool {
    match get_field(fields, "byr") {
        Some(s) => {
            let byr = s.parse::<u32>().unwrap();
            if 1920 > byr || byr > 2002 {
                return false;
            };
        }
        None => return false,
    }

    match get_field(fields, "iyr") {
        Some(s) => {
            let iyr = s.parse::<u32>().unwrap();
            if 2010 > iyr || iyr > 2020 {
                return false;
            };
        }
        None => return false,
    }

    match get_field(fields, "eyr") {
        Some(s) => {
            let eyr = s.parse::<u32>().unwrap();
            if 2020 > eyr || eyr > 2030 {
                return false;
            };
        }
        None => return false,
    }

    match get_field(fields, "hgt") {
        Some(mut s) => {
            if s.ends_with("cm") {
                s.truncate(s.len() - 2);
                let cm = s.parse::<u32>().unwrap();
                if 150 > cm || cm > 193 {
                    return false;
                };
            } else if s.ends_with("in") {
                s.truncate(s.len() - 2);
                let in_ = s.parse::<u32>().unwrap();
                if 59 > in_ || in_ > 76 {
                    return false;
                };
            } else {
                return false;
            };
        }
        None => return false,
    }

    match get_field(fields, "hcl") {
        Some(mut s) => {
            if s.chars().next().unwrap() != '#' {
                return false;
            }
            s.remove(0);
            for c in s.chars() {
                if ('0' > c || c > '9') && ('a' > c || c > 'f') {
                    return false;
                }
            }
        }
        None => return false,
    }

    match get_field(fields, "ecl") {
        Some(s) => {
            let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            if !colors.contains(&s.as_str()) {
                return false;
            }
        }
        None => return false,
    }

    match get_field(fields, "pid") {
        Some(s) => {
            if s.len() != 9 {
                return false;
            };
            match s.parse::<u32>() {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        None => false,
    }
}

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let mut fields: Vec<String> = Vec::new();
    let mut answer = 0;

    for _line in lines {
        let line = _line.unwrap();

        if line == "" {
            if validate(&fields) {
                answer += 1;
            }
            fields.clear();
        } else {
            for field in line.split(" ") {
                fields.push(field.to_string());
            }
        }
    }

    println!("Answer: {}", answer);
}
