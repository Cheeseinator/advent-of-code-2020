use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

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

        // first-second target: passwd
        let mut rsplit = range.split("-");
        let first = rsplit.next().unwrap().parse::<usize>().unwrap();
        let second = rsplit.next().unwrap().parse::<usize>().unwrap();

        println!("{}", line);
        let pchars: Vec<char> = passwd.chars().collect();
        let match1 = pchars[first - 1] == target;
        let match2 = pchars[second - 1] == target;
        if (match1 || match2) && match1 != match2 {
            answer += 1;
        }
    }

    println!("Answer: {}", answer);
}
