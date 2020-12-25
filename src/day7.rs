use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

type Bags = Vec<(String, usize)>;

type Rules = HashMap<String, Bags>;

pub fn a() -> String {
    let rules = rules();
    let end_bag = "shiny gold";
    rules
        .keys()
        .filter(|x| can_ultimately_contain(x, &rules, end_bag))
        .count()
        .to_string()
}

fn can_ultimately_contain(bag: &str, rules: &Rules, end_bag: &str) -> bool {
    rules
        .get(bag)
        .unwrap()
        .iter()
        .any(|x| x.0 == end_bag || can_ultimately_contain(&x.0, rules, end_bag))
}

pub fn b() -> String {
    let rules = rules();
    let start_bag = "shiny gold";
    let maybe_bags = rules.get(start_bag);
    sum_bags_contained(maybe_bags, &rules).to_string()
}

fn sum_bags_contained(maybe_bags: Option<&Bags>, rules: &Rules) -> usize {
    match maybe_bags {
        None => 0,
        Some(bags) => bags
            .iter()
            .map(|x| x.1 + x.1 * sum_bags_contained(rules.get(&x.0), &rules))
            .sum(),
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

fn parse_rule(rule: &str) -> (String, Bags) {
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
