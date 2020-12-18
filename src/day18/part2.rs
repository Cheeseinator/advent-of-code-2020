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

pub fn parse_add(a: &String) -> String {
    let mut tokens: Vec<String> = a.split(" ").map(|x| x.to_string()).collect();
    for i in 1..tokens.len() - 1 {
        let token = &tokens[i];
        if token == "+" {
            tokens[i - 1] = String::from("(") + &tokens[i - 1];
            tokens[i + 1].push(')');
        }
    }
    tokens.join(" ")
}

// turn 3 * (4 + 2) into 3 * 6
fn parse_paren(mut s: &String, deep: bool) -> String {
    let mut arr = vec![String::new()];

    if !s.contains('(') {
        let mut p: String = s.to_string();
        if deep {
            let added = parse_add(s);
            p = parse_paren(&added, false);
        }
        return parse(&p).to_string();
    }

    for c in s.chars() {
        let i = arr.len() - 1;
        match c {
            '(' => {
                arr.push(String::new());
            }
            ')' => {
                let p = parse_paren(&arr[i].to_string(), deep);
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
        // amazingly, this worked on the first try
        let mut s = parse_paren(&e, true);
        s = parse_add(&s);
        s = parse_paren(&s, false);
        res += parse(&s);
    }
    res
}
