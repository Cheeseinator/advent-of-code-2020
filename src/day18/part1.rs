fn parse(e: &String) -> u64 {
    let mut res;
    let mut a = e.split(" ");
    let x: u64 = a.next().unwrap().parse().unwrap();
    let op = match a.next() {
        Some(x) => x,
        None => return e.parse().unwrap(),
    };
    let y: u64 = a.next().unwrap().parse().unwrap();
    match op {
        "+" => res = x + y,
        "*" => res = x * y,
        _ => unreachable!(),
    }
    loop {
        let op = a.next();
        let num = a.next();
        match op {
            Some("+") => {
                res += num.unwrap().parse::<u64>().unwrap();
            }
            Some("*") => {
                res *= num.unwrap().parse::<u64>().unwrap();
            }
            _ => break,
        }
    }
    res
}

// turn 3 * (4 + 2) into 3 * 6
fn parse_paren(s: &String) -> String {
    let mut arr = vec![String::new()];

    if !s.contains('(') {
        return parse(&s).to_string();
    }

    for c in s.chars() {
        let i = arr.len() - 1;
        match c {
            '(' => {
                arr.push(String::new());
            }
            ')' => {
                let p = parse_paren(&arr[i].to_string());
                arr[i - 1] += &p;
                arr.pop();
            }
            _ => {
                arr[i].push(c);
            }
        }
    }
    arr[0].to_string()
}

pub fn answer(v: &Vec<String>) -> u64 {
    let mut res = 0;
    for e in v {
        res += parse(&parse_paren(&e));
    }
    res
}
