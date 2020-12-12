use std::fs::File;
use std::io::{BufRead, BufReader};

type Instructions = Vec<(char, i64)>;

struct Ship {
    x: i64,
    y: i64,
    dir: i64,
}

struct Waypoint {
    x: i64,
    y: i64,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            dir: 90,
        }
    }

    fn go_to(&mut self, w: &Waypoint, t: i64) {
        self.x += w.x * t;
        self.y += w.y * t;
    }
}

impl Waypoint {
    fn new() -> Waypoint {
        Waypoint { x: 10, y: 1 }
    }

    fn rotate(&mut self, d: i64) {
        match d {
            /*
                90:  x, y ->  y, -x
                180: x, y ->  y, -x
                270: x, y -> -y,  x
            */
            90 => {
                let temp = -self.x;
                self.x = self.y;
                self.y = temp;
            }
            180 => {
                self.x = -self.x;
                self.y = -self.y;
            }
            270 => {
                let temp = -self.y;
                self.y = self.x;
                self.x = temp;
            }
            x => panic!("Unknown direction {}", x),
        }
    }
}

fn part_1(instructions: &Instructions) {
    // XY coordinates
    let mut ship = Ship::new();
    for i in instructions {
        match i.0 {
            'N' => ship.y += i.1,
            'E' => ship.x += i.1,
            'S' => ship.y -= i.1,
            'W' => ship.x -= i.1,
            'R' => ship.dir += i.1,
            'L' => match i.1 {
                90 => ship.dir += 270,
                180 => ship.dir += 180,
                270 => ship.dir += 90,
                x => panic!("Unknown direction {}", x),
            },
            'F' => match ship.dir / 90 % 4 {
                // N
                0 => ship.y += i.1,
                // E
                1 => ship.x += i.1,
                // S
                2 => ship.y -= i.1,
                // W
                3 => ship.x -= i.1,
                // this shouldn't happen
                x => panic!("Unknown ship direction {} ({})", ship.dir, x),
            },
            x => panic!("Unknown instruction {}", x),
        }
    }
    println!("part 1 answer: {}", ship.x.abs() + ship.y.abs())
}

fn part_2(instructions: &Instructions) {
    let mut ship = Ship::new();
    let mut point = Waypoint::new();
    for i in instructions {
        match i.0 {
            'N' => point.y += i.1,
            'E' => point.x += i.1,
            'S' => point.y -= i.1,
            'W' => point.x -= i.1,
            'R' => point.rotate(i.1),
            'L' => match i.1 {
                90 => point.rotate(270),
                180 => point.rotate(180),
                270 => point.rotate(90),
                x => panic!("Unknown direction {}", x),
            },
            'F' => ship.go_to(&point, i.1),
            x => panic!("Unknown instruction {}", x),
        }
    }
    println!("part 2 answer: {}", ship.x.abs() + ship.y.abs())
}

pub fn answer(f: File) {
    let mut instructions = Instructions::new();
    for line_ in BufReader::new(f).lines() {
        let mut line = line_.unwrap();
        instructions.push((line.remove(0), line.parse().unwrap()));
    }
    part_1(&instructions);
    part_2(&instructions);
}
