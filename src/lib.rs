use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

fn read_input<T: FromStr>(filename: &str) -> Vec<T> {
    fs::read_to_string(filename)
        .expect("error reading file")
        .trim()
        .split('\n')
        .map(|x| x.parse::<T>().ok().unwrap())
        //.filter_map(|x| x.parse::<T>().ok())
        .collect()
}

fn read_input_world<'a, T: Copy>(
    filename: &str,
    legend: &'a HashMap<char, T>,
) -> Vec<Vec<T>> {
    fs::read_to_string(filename)
        .expect("error reading file")
        .trim()
        .split('\n')
        .map(|x| {
            x.chars()
                .map(|c| legend.get(&c).unwrap().clone())
                .collect::<Vec<T>>()
        })
        .collect()
}
