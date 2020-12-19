use regex::Regex;

use std::cmp::Eq;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const CHAR_REGEX: &str = r#"^(\d+): "(\w)"$"#;
const RULE_REGEX: &str = r#"^(\d+): (.+)$"#;

#[derive(Clone, Debug, Hash, PartialEq)]
enum Rule {
    Char(usize, char),
    Subrules(usize, Vec<usize>),
    Pipe(usize, Vec<usize>, Vec<usize>),
}

impl FromStr for Rule {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_regex = Regex::new(CHAR_REGEX).unwrap();
        let rule_regex = Regex::new(RULE_REGEX).unwrap();

        if let Some(c) = char_regex.captures(s) {
            return Ok(Rule::Char(c[1].parse()?, c[2].chars().next().unwrap()));
        }
        if let Some(c) = rule_regex.captures(s) {
            let id = c[1].parse()?;
            if c[2].contains("|") {
                let pipe: Vec<&str> = c[2].split(" | ").collect();
                let mut r1 = vec![];
                let mut r2 = vec![];
                for x in pipe[0].split(" ") {
                    r1.push(x.parse()?);
                }
                for x in pipe[1].split(" ") {
                    r2.push(x.parse()?);
                }
                return Ok(Rule::Pipe(id, r1, r2));
            } else {
                let mut r = vec![];
                for x in c[2].split(" ") {
                    r.push(x.parse()?);
                }
                return Ok(Rule::Subrules(id, r));
            }
        }
        panic!("Could not match {}", s);
    }
}

impl Rule {
    fn as_regex(&self, rules: &Rules) -> Regex {
        Regex::new(format!("^{}$", self.get_pattern(rules)).as_str()).unwrap()
    }
    fn get_pattern(&self, rules: &Rules) -> String {
        let mut res = String::new();
        match self {
            Rule::Char(_, c) => res.push(*c),
            Rule::Subrules(_, sr) => {
                for i in sr {
                    res += &get(rules, *i).get_pattern(rules);
                }
            }
            Rule::Pipe(11, sr1, sr2) if sr1 == &vec![42, 31] && sr2 == &vec![42, 11, 31] => {
                // part 2
                let p42 = get(rules, 42).get_pattern(rules);
                let p31 = get(rules, 31).get_pattern(rules);
                let mut gen = vec![];
                for i in 1..8 {
                    // regex was a mistake
                    gen.push(format!("({}{{{}}}{}{{{}}})", p42, i, p31, i));
                }
                res += &format!("({})", gen.join("|"));
            }
            Rule::Pipe(8, sr1, sr2) if sr1 == &vec![42] && sr2 == &vec![42, 8] => {
                // part 2 (again)
                let p42 = get(rules, 42).get_pattern(rules);
                res += &format!("{}+", p42);
            }
            Rule::Pipe(_, sr1, sr2) => {
                let mut p1 = String::new();
                for i in sr1 {
                    p1 += &get(rules, *i).get_pattern(rules);
                }
                let mut p2 = String::new();
                for i in sr2 {
                    p2 += &get(rules, *i).get_pattern(rules);
                }
                if p1.len() + p2.len() == 2 {
                    res += &format!("[{}{}]", p1, p2);
                } else {
                    res += &format!("({}|{})", p1, p2);
                }
            }
        }
        res
    }
}

impl Eq for Rule {}

type Rules = HashSet<Rule>;

fn get(r: &Rules, i: usize) -> &Rule {
    for rule in r {
        match rule {
            Rule::Char(x, _) => {
                if *x == i {
                    return rule;
                }
            }
            Rule::Subrules(x, _) => {
                if *x == i {
                    return rule;
                }
            }
            Rule::Pipe(x, _, _) => {
                if *x == i {
                    return rule;
                }
            }
        }
    }
    unreachable!()
}

fn part_1(rules: &Rules, msgs: &Vec<String>) {
    let re = get(&rules, 0).as_regex(&rules);
    let mut res = 0;
    for m in msgs {
        if re.is_match(m) {
            res += 1;
        }
    }
    println!("part 1 answer: {}", res);
}

fn part_2(old_rules: &Rules, msgs: &Vec<String>) {
    let mut new_rules = Rules::new();
    for rule in old_rules {
        match rule {
            Rule::Subrules(8, _) => {
                new_rules.insert("8: 42 | 42 8".parse().unwrap());
            }
            Rule::Subrules(11, _) => {
                new_rules.insert("11: 42 31 | 42 11 31".parse().unwrap());
            }
            _ => {
                new_rules.insert(rule.clone());
            }
        }
    }
    let re = get(&new_rules, 0).as_regex(&new_rules);
    let mut res = 0;
    for m in msgs {
        if re.is_match(m) {
            res += 1;
        }
    }
    println!("part 2 answer: {}", res);
}

pub fn answer(f: File) {
    let mut rules = Rules::new();
    let mut msgs = vec![];
    let mut has_rules = false;
    for line_ in BufReader::new(f).lines() {
        let line = line_.unwrap();
        if !has_rules {
            if line == "" {
                has_rules = true;
            } else {
                rules.insert(line.parse().unwrap());
            }
        } else {
            msgs.push(line);
        }
    }
    part_1(&rules, &msgs);
    part_2(&rules, &msgs);
}
