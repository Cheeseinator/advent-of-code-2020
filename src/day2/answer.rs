use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let mut answer = 0;

    for _line in lines {
        // policy: passwd
        let line = _line.unwrap();
        let mut split = line.split(": ");

        let policy = split.next().unwrap();
        let passwd = split.next().unwrap();

        // range target: passwd
        let mut psplit = policy.split(" ");
        let range = psplit.next().unwrap();
        let target_str = String::from(psplit.next().unwrap());
        let target = target_str.chars().next().unwrap();

        // min-max target: passwd
        let mut rsplit = range.split("-");
        let min = rsplit.next().unwrap().parse::<u8>().unwrap();
        let max = rsplit.next().unwrap().parse::<u8>().unwrap();

        let mut amount = 0;
        for character in passwd.chars() {
            if character == target {
                amount += 1;
            }
        } 

        if min <= amount && amount <= max {
            answer += 1;
        }
    }

    println!("Answer: {}", answer);
}