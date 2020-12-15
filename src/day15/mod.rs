use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn answer(mut f: File) {
    let mut s = String::new();
    let mut starting_nums: Vec<u64> = Vec::new();
    f.read_to_string(&mut s).unwrap();
    for n in s.trim().split(",") {
        starting_nums.push(n.parse().unwrap());
    }
    let mut turn = 1;
    let mut set: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut last = 0;
    for num in starting_nums {
        set.insert(num, vec![turn]);
        last = num;
        turn += 1;
    }
    while turn <= 30000000 {
        if set[&last].len() <= 1 {
            last = 0;
        } else {
            let len = set[&last].len();
            last = set[&last][len - 1] - set[&last][len - 2];
        }
        if !set.contains_key(&last) {
            set.insert(last, Vec::new());
        }
        let mut v = set.remove(&last).unwrap();
        v.push(turn);
        set.insert(last, v);
        if turn == 2020 {
            println!("part 1 answer: {}", last);
        }
        turn += 1;
    }
    println!("part 2 answer: {}", last);
}
