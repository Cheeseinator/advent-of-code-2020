use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1(file: &File) {
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

    println!("part 1 answer: {}", answer);
}

pub fn part_2(file: &File) {
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

        // first-second target: passwd
        let mut rsplit = range.split("-");
        let first = rsplit.next().unwrap().parse::<usize>().unwrap();
        let second = rsplit.next().unwrap().parse::<usize>().unwrap();

        let pchars: Vec<char> = passwd.chars().collect();
        let match1 = pchars[first - 1] == target;
        let match2 = pchars[second - 1] == target;
        if (match1 || match2) && match1 != match2 {
            answer += 1;
        }
    }

    println!("part 2 answer: {}", answer);
}
