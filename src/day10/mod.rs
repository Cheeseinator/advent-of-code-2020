use std::fs::File;
use std::io::{BufRead, BufReader};

type Adapters = Vec<u64>;

fn part_1(a: &Adapters) {
    // this part was easy, just measure the diffs
    let mut d1 = 1; // initial voltage is 0
    let mut d3 = 1; // final voltage is always +3
    for i in 1..a.len() {
        match a[i] - a[i - 1] {
            1 => d1 += 1,
            2 => {}
            3 => d3 += 1,
            // this shouldn't happen
            x => panic!("unknown number {}", x),
        }
    }
    println!("part 1 answer: {}", d1 * d3);
}

fn part_2(a: &Adapters) {
    // i'm not good at math problems and i searched on the internet and someone said to do fibonacci numbers
    let mut fib: Vec<u64> = vec![0, 0, 1];
    for i in a {
        // add 1 to `i` to prevent underflow
        let i = *i as usize + 1;
        // append 0 to `fib` to prevent out of bounds
        while i >= fib.len() {
            fib.push(0);
        }
        // somehow this works
        fib.push(fib[i - 2] + fib[i - 1] + fib[i]);
    }

    println!("part 2 answer: {}", fib.last().unwrap());
}

pub fn answer(f: File) {
    let mut adapters = Adapters::new();
    for line in BufReader::new(f).lines() {
        adapters.push(line.unwrap().parse::<u64>().unwrap());
    }
    adapters.sort();
    part_1(&adapters);
    part_2(&adapters);
}
