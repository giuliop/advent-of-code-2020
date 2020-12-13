use std::fs;

pub fn input_uint(path: &str) -> Vec<u64> {
    fs::read_to_string(path)
        .expect("error reading file")
        .trim()
        .split('\n')
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}
