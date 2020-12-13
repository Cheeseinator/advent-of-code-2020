use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone)]
struct Buses {
    ids: Vec<Option<u64>>,
    times: Vec<Option<u64>>,
    len: usize,
}

impl FromStr for Buses {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buses = Buses {
            ids: Vec::new(),
            times: Vec::new(),
            len: 0,
        };
        for i in s.split(",") {
            buses.len += 1;
            if i != "x" {
                let x = i.parse()?;
                buses.ids.push(Some(x));
                buses.times.push(Some(x))
            } else {
                buses.ids.push(None);
                buses.times.push(None);
            }
        }
        Ok(buses)
    }
}

impl Buses {
    fn part_1(mut self, d: u64) -> u64 {
        loop {
            for i in 0..self.len {
                if self.ids[i].is_none() {
                    continue;
                }
                let bus_id = self.ids[i].unwrap();
                let time = self.times[i].unwrap();
                if time >= d {
                    return (time - d) * bus_id;
                } else {
                    self.times[i] = Some(time + bus_id);
                }
            }
        }
    }
    fn part_2(self) -> u64 {
        let mut res = 1;
        let mut x = 1;
        for i in 0..self.len {
            if self.ids[i].is_none() {
                continue;
            }
            let bus_id = self.ids[i].unwrap();
            while (res + i as u64) % bus_id != 0 {
                res += x;
            }
            x *= bus_id;
        }
        res
    }
}

pub fn answer(f: File) {
    let mut lines = BufReader::new(f).lines();
    let depart: u64 = lines.next().unwrap().unwrap().parse().unwrap();
    let buses: Buses = lines.next().unwrap().unwrap().parse().unwrap();
    println!("part 1 answer: {}", buses.clone().part_1(depart));
    println!("part 2 answer: {}", buses.part_2());
}
