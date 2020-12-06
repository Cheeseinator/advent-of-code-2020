use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

fn calc(s: String, seats: u32, low: char) -> u32 {
    let mut arr: Vec<u32> = (0..seats).collect();
    for c in s.chars() {
        if c == low {
            arr = arr[0..arr.len() / 2].to_vec();
        } else {
            arr = arr[arr.len() / 2..].to_vec();
        }
    }
    arr[0]
}

fn seat_id(seat: &String) -> u32 {
    let (row, col) = seat.split_at(7);
    let rows = calc(row.to_string(), 128, 'F') * 8;
    let cols = calc(col.to_string(), 8, 'L');
    rows + cols
}

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let mut seat_map = HashMap::new();
    let mut max = 0;
    for line_ in lines {
        let line = line_.unwrap();
        let id = seat_id(&line);
        seat_map.insert(id, line);
        if id > max {
            max = id;
        }
    }

    println!("Part 1 answer: {}", max);

    for i in 0..max - 1 {
        if !seat_map.contains_key(&i)
            && seat_map.contains_key(&(i + 1))
            && seat_map.contains_key(&(i - 1))
        {
            // Fun fact: it took me 15 minutes to get this because I thought we were supposed to
            // print out the string instead of the ID
            println!("Part 2 answer: {}", i);
        }
    }
}
