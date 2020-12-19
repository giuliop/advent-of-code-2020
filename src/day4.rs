use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn read_input(filename: &str) -> Vec<Data> {
    fs::read_to_string(filename)
        .expect("error reading file")
        .trim()
        .split("\n\n")
        .map(parse_data)
        .collect()
}

type Data = HashMap<String, String>;

fn parse_data(data: &str) -> Data {
    let mut d: Data = HashMap::new();
    data.split_whitespace().for_each(|x| {
        let y = x.split(':').collect::<Vec<&str>>();
        d.insert(y[0].to_string(), y[1].to_string());
    });
    d
}

fn has_all(d: &Data, fields: &[&str]) -> bool {
    fields.iter().all(|&x| d.contains_key(x))
}

pub fn a() -> String {
    let data = read_input("../input/day4");
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    data.iter()
        .filter(|x| has_all(x, &fields[..]))
        .count()
        .to_string()
}

pub fn b() -> String {
    let data = read_input("../input/day4");
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    data.iter()
        .filter(|x| {
            fields
                .iter()
                .all(|&f| x.contains_key(f) && is_valid(f, x.get(f).unwrap()))
        })
        .count()
        .to_string()
}

fn is_valid(key: &str, value: &str) -> bool {
    match key {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        "byr" => {
            let v = value.parse::<usize>().unwrap_or(0);
            v >= 1920 && v <= 2002
        }
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        "iyr" => {
            let v = value.parse::<usize>().unwrap_or(0);
            v >= 2010 && v <= 2020
        }
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        "eyr" => {
            let v = value.parse::<usize>().unwrap_or(0);
            v >= 2020 && v <= 2030
        }
        // hgt (Height) - a number followed by either cm or in:
        // - If cm, the number must be at least 150 and at most 193.
        // - If in, the number must be at least 59 and at most 76.
        "hgt" => {
            let re = Regex::new(r"^(?P<val>\d*)(?P<unit>cm|in)$").unwrap();
            if let Some(caps) = re.captures(value) {
                if let Some(val) = caps.name("val") {
                    if let Some(unit) = caps.name("unit") {
                        let val = val.as_str().parse::<usize>().unwrap_or(0);
                        match unit.as_str() {
                            "cm" => return val >= 150 && val <= 193,
                            "in" => return val >= 59 && val <= 76,
                            _ => return false,
                        }
                    }
                }
            }
            false
        }
        //hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        "hcl" => {
            let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            re.is_match(value)
        }
        //ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        "ecl" => {
            let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
            re.is_match(value)
        }
        //pid (Passport ID) - a nine-digit number, including leading zeroes.
        "pid" => {
            let re = Regex::new(r"^\d{9}$").unwrap();
            re.is_match(value)
        }
        //cid (Country ID) - ignored, missing or not.
        _ => true,
    }
}
