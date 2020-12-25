use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

type Bag = Vec<(String, usize)>;

type Rules = HashMap<String, Bag>;

pub fn a() -> String {
    let rules = rules();
    let mut contains_gold_bag: HashMap<String, bool> = HashMap::new();
    rules
        .keys()
        .fold(0, |acc, name| {
            if can_contain_gold_bag(name, &rules, &mut contains_gold_bag) {
                acc + 1
            } else {
                acc
            }
        })
        .to_string()
}

fn can_contain_gold_bag(
    bag: &str,
    rules: &Rules,
    contains_gold_bag: &mut HashMap<String, bool>,
) -> bool {
    match contains_gold_bag.get(bag) {
        Some(true) => true,
        Some(false) => false,
        None => {
            let bags = &rules.get(bag).unwrap();
            let can = bags.iter().any(|x| x.0 == "shiny gold")
                || bags
                    .iter()
                    .any(|x| can_contain_gold_bag(&x.0, rules, contains_gold_bag));
            contains_gold_bag.insert(bag.to_string(), can);
            can
        }
    }
}

fn rules() -> Rules {
    let mut rules = HashMap::new();
    fs::read_to_string("../input/day7")
        .expect("error reading file")
        .trim()
        .lines()
        .map(parse_rule)
        .for_each(|x| {
            rules.insert(x.0, x.1);
        });
    rules
}

fn parse_rule(rule: &str) -> (String, Bag) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d) (.+?) bag").unwrap();
    }
    let mut rule_iter = rule.split(" bags contain ");
    let name: String = rule_iter.next().unwrap().to_string();
    let contains: &str = rule_iter.next().unwrap();

    let contains: Vec<(String, usize)> = RE
        .captures_iter(contains)
        .map(|cap| (cap[2].to_string(), cap[1].parse::<usize>().unwrap()))
        .collect();
    (name, contains)
}

pub fn b() -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule() {
        let rule = "dotted salmon bags contain 2 dark lavender bags, 1 muted red bag, 1 vibrant magenta bag.";

        let (name, bag) = parse_rule(rule);
        assert_eq!(name, "dotted salmon".to_string());
        assert_eq!(
            bag.contains,
            vec![
                ("dark lavender".to_string(), 2usize),
                ("muted red".to_string(), 1usize),
                ("vibrant magenta".to_string(), 1usize)
            ]
        );
        //assert_eq!(bag.can_contain_gold_bag, None);
    }
}
