use std::io::{BufRead, BufReader};

type Board = Vec<Vec<char>>;

enum Rules {
    Part1,
    Part2,
}

fn tile_at(b: &Board, x: isize, y: isize) -> char {
    if x == -1 || y == -1 {
        return '?';
    }
    match b.get(y as usize) {
        Some(row) => match row.get(x as usize) {
            Some(c) => *c,
            _ => '?',
        },
        None => '?',
    }
}

fn adjacent(b: &Board, x: isize, y: isize) -> Vec<char> {
    let mut res = Vec::new();
    for h in -1..2 {
        for w in -1..2 {
            if (h, w) != (0, 0) {
                let t = tile_at(b, x + w, y + h);
                if t != '?' {
                    res.push(t);
                }
            }
        }
    }
    res
}

// get a visible tile in a direction
fn visible_tile(b: &Board, x: isize, y: isize, offset: (isize, isize)) -> char {
    let mut new_x = x + offset.0;
    let mut new_y = y + offset.1;

    loop {
        let t = tile_at(b, new_x, new_y);
        match t {
            '#' => return '#',
            'L' => return 'L',
            '?' => return '?',
            _ => {}
        }
        new_x += offset.0;
        new_y += offset.1;
    }
}

// get visible tiles in all directions
fn visible_tiles(b: &Board, x: isize, y: isize) -> Vec<char> {
    let mut res = Vec::new();
    for h in -1..2 {
        for w in -1..2 {
            if (h, w) != (0, 0) {
                let t = visible_tile(b, x, y, (w, h));
                if t != '?' {
                    res.push(t);
                }
            }
        }
    }
    res
}

// calculate each individual tile's next state
fn mutate_tile(b: &Board, x: isize, y: isize, r: &Rules) -> char {
    let t = tile_at(b, x, y);
    let tiles: Vec<char>;
    match *r {
        Rules::Part1 => tiles = adjacent(b, x, y),
        Rules::Part2 => tiles = visible_tiles(b, x, y),
    }
    match t {
        'L' => {
            // L -> # if no adjacent #
            for a in tiles {
                if a == '#' {
                    return 'L';
                }
            }
            '#'
        }
        '#' => {
            // # -> L if adjacent # >= 4
            let mut c = 0;
            for a in tiles {
                if a == '#' {
                    c += 1;
                }
            }
            match (r, c) {
                (Rules::Part1, 0..=3) | (Rules::Part2, 0..=4) => '#',
                _ => 'L',
            }
        }
        '.' => '.',
        _ => '?',
    }
}

// return the board's next state after rules are applied
fn mutate(state: Board, r: &Rules) -> Board {
    let mut next = Board::new();
    for (y, row) in state.iter().enumerate() {
        let mut next_row: Vec<char> = Vec::new();
        for (x, _) in row.iter().enumerate() {
            next_row.push(mutate_tile(&state, x as isize, y as isize, &r));
        }
        next.push(next_row);
    }
    next
}

fn simulate(mut board: Board, r: Rules) {
    // mutate boards until everything stops moving, i.e. the states are equal
    let mut last_board = board.clone();
    board = mutate(last_board.clone(), &r);
    while *last_board != *board {
        last_board = board.clone();
        board = mutate(last_board.clone(), &r);
    }
    let mut o = 0;
    for row in board {
        for tile in row {
            if tile == '#' {
                o += 1;
            }
        }
    }
    println!(
        "part {} answer: {}",
        match r {
            Rules::Part1 => 1,
            Rules::Part2 => 2,
        },
        o
    );
}

pub fn answer(f: std::fs::File) {
    // parse board from file
    let mut board: Board = Board::new();
    for line in BufReader::new(f).lines() {
        let mut row: Vec<char> = Vec::new();
        for chr in line.unwrap().chars() {
            row.push(chr);
        }
        board.push(row);
    }

    simulate(board.clone(), Rules::Part1);
    simulate(board, Rules::Part2);
}
