use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;

enum Rule {
    Literal(Vec<String>),
    Single(Vec<usize>),
    Double((Vec<usize>, Vec<usize>)),
}

struct ValidMessage(HashMap<usize, HashSet<String>>);

type Messages = Vec<String>;

impl ValidMessage {
    fn from_str(s: &str) -> Self {
        let mut rules = parse_rules(s);
        let mut res = ValidMessage(HashMap::new());
        while !rules.is_empty() {
            for k in rules.keys().map(|x| *x).collect::<Vec<usize>>() {
                let v = rules.get(&k).unwrap();
                match v {
                    Rule::Literal(v) => {
                        res.0.insert(k, HashSet::from_iter(v.clone()));
                        rules.remove(&k);
                    }
                    Rule::Single(v) => {
                        if v.iter().all(|x| res.0.contains_key(x)) {
                            res.add(k, &v);
                            rules.remove(&k);
                        }
                    }
                    Rule::Double((v1, v2)) => {
                        if v1.iter().all(|x| res.0.contains_key(x))
                            && v2.iter().all(|x| res.0.contains_key(x))
                        {
                            res.add_double(k, &v1, &v2);
                            rules.remove(&k);
                        }
                    }
                }
            }
        }
        res
    }

    fn add(&mut self, key: usize, value: &Vec<usize>) {
        let v = value.iter().fold(vec!["".to_string()], |acc, k| {
            combinations(&acc, self.0.get(k).unwrap())
        });
        self.0.insert(key, HashSet::from_iter(v));
    }

    fn add_double(&mut self, key: usize, v1: &Vec<usize>, v2: &Vec<usize>) {
        let mut v1 = v1.iter().fold(vec!["".to_string()], |acc, k| {
            combinations(&acc, self.0.get(k).unwrap())
        });
        let v2 = v2.iter().fold(vec!["".to_string()], |acc, k| {
            combinations(&acc, self.0.get(k).unwrap())
        });
        v1.extend(v2);
        self.0.insert(key, HashSet::from_iter(v1));
    }
}

fn combinations(v1: &Vec<String>, v2: &HashSet<String>) -> Vec<String> {
    let mut res = Vec::new();
    for x in v1 {
        for y in v2 {
            res.push(x.clone() + y);
        }
    }
    res
}

fn nums_from(s: &str) -> Vec<usize> {
    s.split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}

fn parse_rules(s: &str) -> HashMap<usize, Rule> {
    let mut h = HashMap::new();
    for rule in s.lines() {
        let mut kv = rule.split(": ");
        let k: usize = kv.next().unwrap().parse().unwrap();
        let v: &str = kv.next().unwrap();
        let v0: char = v.chars().next().unwrap();
        let v = match v0 {
            '"' => Rule::Literal(vec![v[1..v.len() - 1].to_string()]),
            _ => match v.find(|x| x == '|') {
                Some(idx) => Rule::Double((
                    nums_from(&v[..idx - 1]),
                    nums_from(&v[idx + 2..]),
                )),
                None => Rule::Single(nums_from(v)),
            },
        };
        h.insert(k, v);
    }
    h
}

fn parse_input(s: &str) -> (ValidMessage, Messages) {
    let mut data = s.split("\n\n");
    (
        ValidMessage::from_str(data.next().unwrap()),
        data.next()
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect(),
    )
}

pub fn a() -> String {
    let data = fs::read_to_string("../input/day19").expect("error reading file");
    let (rules, msg) = parse_input(&data);
    let valid: &HashSet<String> = rules.0.get(&0).unwrap();
    msg.iter()
        .filter(|m| valid.contains(*m))
        .count()
        .to_string()
}

// new 8 -> "8: 42 | 42 8"
// new 11 -> "11: 42 31 | 42 11 31"
pub fn b() -> String {
    let data = fs::read_to_string("../input/day19").expect("error reading file");
    let (rules, msg) = parse_input(&data);
    let valid: &HashSet<String> = rules.0.get(&0).unwrap();

    let r42 = rules.0.get(&42).unwrap();
    let r31 = rules.0.get(&31).unwrap();
    let len = r42.iter().next().unwrap().len();

    assert!(
        r42.iter().all(|x| x.len() == len) && r31.iter().all(|x| x.len() == len)
    );

    msg.iter()
        .filter(|m| valid.contains(*m) || is_valid_by_new_rules(*m, &r42, &r31, len))
        .count()
        .to_string()
}

fn is_valid_by_new_rules(
    m: &str,
    r42: &HashSet<String>,
    r31: &HashSet<String>,
    sublen: usize,
) -> bool {
    let head = head_matches(m, r42, sublen);
    let tail = tail_matches(m, r31, sublen);
    m.len() % sublen == 0
        && head >= 2
        && tail >= 1
        && head > tail
        && head + tail >= m.len() / sublen
}

fn head_matches(m: &str, ss: &HashSet<String>, sublen: usize) -> usize {
    (0..(m.len() / sublen))
        .take_while(|i| ss.contains(&m[i * sublen..(i + 1) * sublen]))
        .count()
}

fn tail_matches(m: &str, ss: &HashSet<String>, sublen: usize) -> usize {
    (0..(m.len() / sublen))
        .rev()
        .take_while(|i| ss.contains(&m[i * sublen..(i + 1) * sublen]))
        .count()
}
