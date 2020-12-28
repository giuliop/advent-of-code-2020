use crate::read_input;
use itertools::Itertools;

const OFFSET: usize = 25;

fn combinations(nums: &[usize]) -> Vec<usize> {
    nums.iter()
        .combinations(2)
        .map(|x| x.iter().copied().sum())
        .collect()
}

fn invalid_number(nums: &Vec<usize>) -> usize {
    nums.windows(OFFSET + 1)
        .find(|window| {
            combinations(&window[..OFFSET])
                .iter()
                .all(|x| x != &window[OFFSET])
        })
        .unwrap()[OFFSET]
}

pub fn a() -> String {
    invalid_number(&read_input::<usize>("../input/day9")).to_string()
}

// return the start and end index od the contiguous set that sums to n in nums
fn contiguous_set(n: usize, nums: &Vec<usize>) -> Option<(usize, usize)> {
    for start in 0..nums.len() {
        for end in start + 1..nums.len() {
            match nums[start..end].iter().sum::<usize>() {
                x if x == n => return Some((start, end)),
                x if x > n => break,
                _ => (),
            }
        }
    }
    None
}

pub fn b() -> String {
    let nums = read_input::<usize>("../input/day9");
    let n = invalid_number(&nums);

    let (start, end) = contiguous_set(n, &nums).unwrap();
    let range = &nums[start..end];

    (range.iter().max().unwrap() + range.iter().min().unwrap()).to_string()
}
