use crate::read_input;

pub fn a() -> String {
    let nums = read_input::<usize>("../input/day1");
    let mut res = 0;
    'search: for (i, n1) in nums.iter().enumerate() {
        for n2 in &nums[i + 1..] {
            if n1 + n2 == 2020 {
                res = n1 * n2;
                break 'search;
            }
        }
    }
    return res.to_string();
}

pub fn b() -> String {
    let nums = read_input::<usize>("../input/day1");
    let mut res = 0;
    'search: for (i, n1) in nums.iter().enumerate() {
        for (i2, n2) in nums[i + 1..].iter().enumerate() {
            let n12 = n1 + n2;
            if n12 < 2020 {
                for n3 in &nums[i2 + 1..] {
                    if n12 + n3 == 2020 {
                        res = n1 * n2 * n3;
                        break 'search;
                    }
                }
            }
        }
    }
    return res.to_string();
}
