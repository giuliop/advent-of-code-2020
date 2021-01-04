use std::collections::HashMap;

pub fn count(total_turns: usize) -> usize {
    let mut starting_numbers = vec![7, 14, 0, 17, 11, 1, 2];
    let next_turn = starting_numbers.len();
    let mut last_number = starting_numbers.pop().unwrap();

    let mut numbers: HashMap<usize, usize> = starting_numbers
        .iter()
        .enumerate()
        .map(|(i, x)| (*x, i + 1))
        .collect();

    for current_turn in next_turn..total_turns {
        last_number = current_turn
            - numbers
                .insert(last_number, current_turn)
                .unwrap_or(current_turn);
    }

    last_number
}

pub fn a() -> String {
    count(2020).to_string()
}

pub fn b() -> String {
    count(30000000).to_string()
}
