use std::fs::File;
use std::io::{BufRead, BufReader};

type Numbers = Vec<u64>;

fn is_sum(v: &Numbers, x: u64) -> bool {
    let len = v.len();
    for i in 0..len {
        for j in 0..len {
            if i != j {
                if v[i] + v[j] == x {
                    return true;
                }
            }
        }
    }
    false
}

pub fn answer(f: File) {
    let mut nums = Numbers::new();
    let mut arr = Numbers::new();
    for line in BufReader::new(f).lines() {
        let num = line.unwrap().parse::<u64>().unwrap();
        nums.push(num);
    }
    for (i, num_p) in nums.iter().enumerate() {
        let num = *num_p;
        if i < 25 {
            // preamble
            arr.push(num);
            continue;
        }
        if !is_sum(&arr, num) {
            println!("part 1 answer: {}", num);

            let len = nums.len();
            for i in 0..len {
                for j in 0..len {
                    if i < j {
                        let mut t = 0;
                        for k in i..j {
                            t += nums[k];
                        }
                        if t == num {
                            let slice = &nums[i..j];
                            let min = slice.iter().min().unwrap();
                            let max = slice.iter().max().unwrap();
                            println!("part 2 answer: {}", min + max);
                            return;
                        }
                    };
                }
            }
        }
        arr.push(num);
        arr.remove(0);
    }
}
