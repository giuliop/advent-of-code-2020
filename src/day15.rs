use std::collections::HashMap;

pub fn a() -> String {
    let mut starting_nums = vec![7, 14, 0, 17, 11, 1, 2];
    let total_turns = 2020;

    let mut last: usize = starting_nums.pop().unwrap();

    let mut nums: HashMap<usize, usize> = starting_nums
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i))
        .collect();

    let turns_so_far = nums.keys().count();

    for current_turn in turns_so_far..total_turns - 1 {
        last = match nums.get(&last).cloned() {
            Some(turn) => {
                nums.insert(last, current_turn);
                current_turn - turn
            }
            None => {
                nums.insert(last, current_turn);
                0
            }
        };
        //println!("turn: {}, spoken: {}", current_turn + 1, last);
    }
    last.to_string()
}

pub fn b() -> String {
    "".to_string()
}
