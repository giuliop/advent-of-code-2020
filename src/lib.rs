use std::fs;

pub mod day1;

pub fn input_uint(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .expect("error reading file")
        .trim()
        .split('\n')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}
