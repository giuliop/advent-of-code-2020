use std::collections::HashSet;
use std::fs;

pub fn a() -> String {
    fs::read_to_string("../input/day6")
        .expect("error reading file")
        .trim()
        .split("\n\n")
        .map(parse_data)
        .map(|x| x.len())
        .sum::<usize>()
        .to_string()
}

fn parse_data(group: &str) -> HashSet<char> {
    let mut h = HashSet::new();
    group.chars().for_each(|x| match x {
        'a'..='z' => {
            h.insert(x);
        }
        _ => (),
    });
    h
}

pub fn b() -> String {
    let h: HashSet<char> = ('a'..='z').collect();
    fs::read_to_string("../input/day6")
        .expect("error reading file")
        .trim()
        .split("\n\n")
        .map(|x| {
            x.split('\n').fold(h.clone(), |acc, x| {
                let x = x.chars().collect();
                acc.intersection(&x).copied().collect()
            })
        })
        .map(|x| x.len())
        .sum::<usize>()
        .to_string()
}
