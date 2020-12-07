use std::fs::File;
use std::io::{BufRead, BufReader};

fn has_fields(fields: &Vec<String>, required: &Vec<&str>) -> bool {
    for r in required {
        if !fields.contains(&r.to_string()) {
            return false;
        }
    }
    true
}

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

fn check_fields(fields: &Vec<String>) -> bool {
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

pub fn part_1(file: &File) {
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let lines = BufReader::new(file).lines();

    let mut fields: Vec<String> = Vec::new();
    let mut answer = 0;

    for _line in lines {
        let line = _line.unwrap();

        if line == "" {
            if has_fields(&fields, &required) {
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

    println!("answer: {}", answer);
}

pub fn part_2(file: &File) {
    let lines = BufReader::new(file).lines();

    let mut fields: Vec<String> = Vec::new();
    let mut answer = 0;

    for _line in lines {
        let line = _line.unwrap();

        if line == "" {
            if check_fields(&fields) {
                answer += 1;
            }
            fields.clear();
        } else {
            for field in line.split(" ") {
                fields.push(field.to_string());
            }
        }
    }

    println!("answer: {}", answer);
}
