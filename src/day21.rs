use std::collections::{HashMap, HashSet};
use std::fs;

fn read_input(f: &str) -> (Vec<String>, HashMap<String, Vec<HashSet<String>>>) {
    let data: Vec<(Vec<String>, HashSet<String>)> = fs::read_to_string(f)
        .expect("error reading file")
        .lines()
        .map(|s| {
            let raw = s.split(" (contains ").collect::<Vec<&str>>();
            let (ingredients, allergens) = (raw[0], raw[1]);
            (
                allergens[..allergens.len() - 1]
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect(),
                ingredients
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect(),
            )
        })
        .collect();

    let mut all_ingredients = Vec::new();
    let mut list = HashMap::new();

    for (allergens, ingredients) in data {
        for allergen in allergens {
            list.entry(allergen)
                .and_modify(|e: &mut Vec<HashSet<String>>| {
                    e.push(ingredients.clone());
                })
                .or_insert(vec![ingredients.clone()]);
        }
        for ingredient in ingredients {
            all_ingredients.push(ingredient);
        }
    }
    (all_ingredients, list)
}

pub fn solution() -> (usize, Vec<String>) {
    let (ingredients, list) = read_input("../input/day21");

    let mut excluded_foods = list
        .iter()
        .map(|(k, v)| {
            let (set, others) = v.split_at(1);
            (
                &k[..],
                set[0]
                    .iter()
                    .filter(|e| others.iter().all(|s| s.contains(*e)))
                    .map(|s| &s[..])
                    .collect(),
            )
        })
        .collect::<HashMap<&str, HashSet<&str>>>();

    while excluded_foods.values().any(|set| set.len() > 1) {
        let finalized: HashSet<&str> = excluded_foods
            .values()
            .filter(|set| set.len() == 1)
            .map(|set| *set.iter().next().unwrap())
            .collect();
        for set in excluded_foods.values_mut().filter(|set| set.len() > 1) {
            set.retain(|e| !finalized.contains(e));
        }
    }

    let mut excluded_foods = excluded_foods
        .iter()
        .map(|(k, v)| (*k, &v.iter().next().unwrap()[..]))
        .collect::<Vec<(&str, &str)>>();

    excluded_foods.sort_unstable_by_key(|x| x.0);

    let excluded_foods = excluded_foods
        .iter()
        .map(|x| x.1.to_string())
        .collect::<Vec<String>>();

    (
        ingredients
            .iter()
            .filter(|e| {
                !excluded_foods
                    .iter()
                    .map(|x| x.as_str())
                    .collect::<HashSet<&str>>()
                    .contains(e.as_str())
            })
            .count(),
        excluded_foods,
    )
}

pub fn a() -> String {
    solution().0.to_string()
}

pub fn b() -> String {
    let mut res = String::new();
    for s in solution().1 {
        res.push_str(&s);
        res.push(',');
    }
    res.pop();
    res
}
