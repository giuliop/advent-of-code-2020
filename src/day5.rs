use crate::read_input;

struct Seat {
    row: usize,
    col: usize,
    id: usize,
}

const MAX_ID: usize = 127 * 8 + 7;

//struct SeatParseError;

impl Seat {
    fn from_code(code: &str) -> Seat {
        let binary: String = code
            .chars()
            .map(|x| match x {
                'B' | 'R' => '1',
                'F' | 'L' => '0',
                _ => '!', // will panic later
            })
            .collect();

        let row = usize::from_str_radix(&binary[..7], 2).unwrap();
        let col = usize::from_str_radix(&binary[7..], 2).unwrap();
        let id = row * 8 + col;

        Seat { row, col, id }
    }
}

pub fn a() -> String {
    read_input::<String>("../input/day5")
        .iter()
        .map(|x| Seat::from_code(x).id)
        .max()
        .unwrap()
        .to_string()
}

pub fn b() -> String {
    available_ids(
        read_input::<String>("../input/day5")
            .iter()
            .map(|x| Seat::from_code(x).id)
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
