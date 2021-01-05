use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Input {
    rules: HashMap<String, (RangeInclusive<usize>, RangeInclusive<usize>)>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl Input {
    fn valid_field(&self, n: &usize) -> bool {
        self.rules
            .values()
            .any(|(r1, r2)| r1.contains(n) || r2.contains(n))
    }

    fn valid_ticket(&self, t: &Vec<usize>) -> bool {
        t.iter().all(|f| self.valid_field(f))
    }
}

fn parse_rule(s: &str) -> (String, (RangeInclusive<usize>, RangeInclusive<usize>)) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(.+?): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }
    let caps = RE.captures(s).unwrap();
    (
        caps[1].to_string(),
        (
            RangeInclusive::new(caps[2].parse().unwrap(), caps[3].parse().unwrap()),
            RangeInclusive::new(caps[4].parse().unwrap(), caps[5].parse().unwrap()),
        ),
    )
}

fn read_input() -> Input {
    let input = fs::read_to_string("../input/day16").expect("error reading file");
    let mut split = input.split("\n\n");
    Input {
        rules: split
            .next()
            .unwrap()
            .lines()
            .map(|s| parse_rule(s))
            .collect(),
        my_ticket: split
            .next()
            .unwrap()
            .split(":\n")
            .nth(1)
            .unwrap()
            .split(',')
            .filter_map(|x| x.parse().ok())
            .collect(),
        nearby_tickets: split
            .next()
            .unwrap()
            .split(":\n")
            .nth(1)
            .unwrap()
            .lines()
            .map(|line| line.split(',').filter_map(|x| x.parse().ok()).collect())
            .collect(),
    }
}

pub fn a() -> String {
    let input = read_input();
    input
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|n| (!input.valid_field(n)))
        .sum::<usize>()
        .to_string()
}

pub fn b() -> String {
    let input = read_input();

    let tickets = input
        .nearby_tickets
        .iter()
        .filter(|x| input.valid_ticket(x))
        .collect();

    let mut matches: Vec<(usize, Vec<&str>)> = (0..input.my_ticket.len())
        .map(|i| (i, possible_fields(&input.rules, &tickets, i)))
        .collect();

    let mut finalized: Vec<(&str, usize)> = Vec::new();

    while !matches.is_empty() {
        let (mut new_matches, new_finalized): (
            Vec<(usize, Vec<&str>)>,
            Vec<(usize, Vec<&str>)>,
        ) = matches.iter().cloned().partition(|(_, v)| v.len() > 1);

        new_matches.iter_mut().for_each(|(_, v)| {
            v.retain(|&x| new_finalized.iter().all(|(_, v)| x != v[0]))
        });

        matches = new_matches;
        finalized.extend(
            new_finalized
                .iter()
                .map(|(i, v)| (v[0], *i))
                .collect::<Vec<(&str, usize)>>(),
        );
    }
    finalized
        .iter()
        .filter(|(s, _)| s.len() >= 9 && &s[..9] == "departure")
        .map(|(_, n)| input.my_ticket[*n])
        .product::<usize>()
        .to_string()
}

fn possible_fields<'a>(
    rules: &'a HashMap<String, (RangeInclusive<usize>, RangeInclusive<usize>)>,
    tickets: &Vec<&Vec<usize>>,
    pos: usize,
) -> Vec<&'a str> {
    let nums: Vec<usize> = tickets.iter().map(|t| t[pos]).collect();
    rules
        .iter()
        .filter(|(_, (r1, r2))| {
            nums.iter().all(|n| r1.contains(n) || r2.contains(n))
        })
        .map(|(k, _)| k.as_str())
        .collect()
}
