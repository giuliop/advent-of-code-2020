use crate::read_input;

const MAX_ID: usize = 127 * 8 + 7;

fn id_from_code(code: &str) -> usize {
    let binary: String = code
        .chars()
        .map(|x| match x {
            'B' | 'R' => '1',
            'F' | 'L' => '0',
            _ => unreachable!(),
        })
        .collect();

    let row = usize::from_str_radix(&binary[..7], 2).unwrap();
    let col = usize::from_str_radix(&binary[7..], 2).unwrap();
    row * 8 + col
}

pub fn a() -> String {
    read_input::<String>("../input/day5")
        .iter()
        .map(|x| id_from_code(x))
        .max()
        .unwrap()
        .to_string()
}

pub fn b() -> String {
    available_ids(
        read_input::<String>("../input/day5")
            .iter()
            .map(|x| id_from_code(x))
            .collect(),
    )[0]
    .to_string()
}

fn available_ids(ids: Vec<usize>) -> Vec<usize> {
    (0..MAX_ID)
        .filter(|x| {
            !ids.contains(x) && ids.contains(&(*x + 1)) && ids.contains(&(*x - 1))
        })
        .collect()
}
