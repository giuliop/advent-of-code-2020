use std::fs;
use std::str::FromStr;

pub mod day1;
pub mod day2;

fn read_input<T: FromStr>(path: &str) -> Vec<T> {
    fs::read_to_string(path)
        .expect("error reading file")
        .trim()
        .split('\n')
        .map(|x| x.parse::<T>().ok().unwrap())
        //.filter_map(|x| x.parse::<T>().ok())
        .collect()
}
