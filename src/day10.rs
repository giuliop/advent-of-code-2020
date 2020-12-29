use crate::read_input;
use std::collections::HashMap;

fn adapters() -> Vec<usize> {
    let mut adapters = read_input::<usize>("../input/day10");
    adapters.insert(0, 0);
    adapters.sort_unstable();
    adapters.push(adapters[adapters.len() - 1] + 3);
    adapters
}

pub fn a() -> String {
    let adapters = adapters();

    let (ones, threes) =
        adapters
            .windows(2)
            .fold((0, 0), |(ones, threes), v| match v[1] - v[0] {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => unreachable!(),
            });
    (ones * threes).to_string()
}

pub fn b() -> String {
    let adapters = adapters();
    let mut memo = HashMap::new();
    memo.insert(adapters.len() - 1, 1);

    count_combinations(0, &adapters, &mut memo).to_string()
}

// return the number of combinations that can be created starting from
// index 'idx' of the 'adapters' vector, using 'memo' for memoization
fn count_combinations(
    idx: usize,
    adapters: &Vec<usize>,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    next_items(adapters, idx)
        .iter()
        .map(|&x| match memo.get(&x) {
            Some(&y) => y,
            None => {
                let fx = count_combinations(x, adapters, memo);
                memo.insert(x, fx);
                fx
            }
        })
        .sum()
}

// take an index 'idx' of the 'adapters' vector and return a vector with the
// next indexes  of the combinations that can be created starting from it
fn next_items(adapters: &Vec<usize>, idx: usize) -> Vec<usize> {
    let n: usize = adapters[idx];
    adapters[idx + 1..]
        .iter()
        .enumerate()
        .take_while(|(_, &x)| (x - n) <= 3)
        .map(|x| x.0 + idx + 1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_items() {
        fn try_n(n: usize) -> Vec<usize> {
            let adapters = vec![0, 1, 3, 5, 6, 9, 10];
            next_items(&adapters, n)
        }

        assert_eq!(try_n(0), vec![1, 2]);
        assert_eq!(try_n(3), vec![4]);
        assert_eq!(try_n(6), vec![]);
    }

    #[test]
    fn test_count_combinations() {
        let adapters = vec![0, 1, 2, 3, 4, 5];
        let mut memo = HashMap::new();
        memo.insert(adapters.len() - 1, 1);

        let res = count_combinations(0, &adapters, &mut memo);
        assert_eq!(res, 13);
    }
}
