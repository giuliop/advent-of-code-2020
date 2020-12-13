use crate::read_input;

pub fn a() -> String {
    let nums = read_input::<String>("../input/day2");
    nums.into_iter()
        .filter(|x| is_valid_a(x))
        .count()
        .to_string()
}

fn is_valid_a(s: &str) -> bool {
    let s: Vec<&str> = s.split(' ').collect();

    let range: Vec<usize> = s[0]
        .split('-')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    let low = range[0];
    let high = range[1];

    let val = s[1].chars().next().unwrap();

    let pw = s[2];

    let actual = pw.matches(val).count();
    low <= actual && actual <= high
}

pub fn b() -> String {
    let nums = read_input::<String>("../input/day2");
    nums.into_iter()
        .filter(|x| is_valid_b(x))
        .count()
        .to_string()
}

fn is_valid_b(s: &str) -> bool {
    let s: Vec<&str> = s.split(' ').collect();

    let range: Vec<usize> = s[0]
        .split('-')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    let pos1 = range[0] - 1;
    let pos2 = range[1] - 1;

    let val = s[1].chars().next().unwrap();

    let pw = s[2];

    let positions: Vec<_> = pw.match_indices(val).map(|x| x.0).collect();

    let mut count = 0;
    for p in positions {
        if p == pos1 || p == pos2 {
            count += 1;
        }
    }

    count == 1
}
