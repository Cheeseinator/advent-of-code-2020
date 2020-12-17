use std::collections::HashSet;
use std::ops::Range;

type Cell = (i64, i64, i64); // x, y, z

fn neighbors(cube: &HashSet<Cell>, c: Cell) -> u64 {
    let mut res = 0;
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                if (x != 0 || y != 0 || z != 0) && cube.contains(&(c.0 + x, c.1 + y, c.2 + z)) {
                    res += 1;
                }
            }
        }
    }
    res
}

fn dimensions(cube: &HashSet<Cell>) -> (Range<i64>, Range<i64>, Range<i64>) {
    let mut x = (0, 0);
    let mut y = (0, 0);
    let mut z = (0, 0);
    for cell in cube {
        if cell.0 < x.0 {
            x.0 = cell.0
        }
        if cell.0 > x.1 {
            x.1 = cell.0
        }
        if cell.1 < y.0 {
            y.0 = cell.1
        }
        if cell.1 > y.1 {
            y.1 = cell.1
        }
        if cell.2 < z.0 {
            z.0 = cell.2
        }
        if cell.2 > z.1 {
            z.1 = cell.2
        }
    }
    // pad all values by 1, endings are exclusive so they get padded again
    (x.0 - 1..x.1 + 2, y.0 - 1..y.1 + 2, z.0 - 1..z.1 + 2)
}

pub fn next(current: HashSet<Cell>) -> HashSet<Cell> {
    let mut state = HashSet::new();
    let dim = dimensions(&current);
    for x in dim.0 {
        for y in dim.1.clone() {
            for z in dim.2.clone() {
                let cell = (x, y, z);
                let n = neighbors(&current, cell);
                if current.contains(&cell) {
                    if n == 2 || n == 3 {
                        state.insert(cell);
                    }
                } else if n == 3 {
                    state.insert(cell);
                }
            }
        }
    }
    state
}
