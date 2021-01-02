use modinverse::modinverse;
use std::fs;

fn read_input_part1(path: &str) -> (usize, Vec<usize>) {
    let data = fs::read_to_string(path).expect("error reading file");
    let split = data.find('\n').unwrap();
    let time = data[..split].parse().unwrap();
    let buses = data[split + 1..]
        .trim()
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse().unwrap())
        .collect();
    (time, buses)
}

pub fn a() -> String {
    let (time, buses) = read_input_part1("../input/day13");
    let times: Vec<usize> = buses
        .iter()
        .map(|x| {
            let rem = time % x;
            time + if rem == 0 { 0 } else { x - rem }
        })
        .collect();
    let (i, bus_time) = times
        .iter()
        .enumerate()
        .filter(|(_, &x)| x >= time)
        .min_by_key(|x| x.1)
        .unwrap();
    (buses[i] * (bus_time - time)).to_string()
}

pub fn b() -> String {
    b_helper("../input/day13")
}

pub fn b_helper(s: &str) -> String {
    let times: Vec<(usize, usize)> = read_input_part2(s);
    let prod: usize = times.iter().map(|(_, x)| x).product();

    (times
        .iter()
        .map(|(a, n)| {
            let b = prod / n;
            a * b * (modinverse(b as isize, *n as isize).unwrap() as usize)
        })
        .sum::<usize>()
        % prod)
        .to_string()
}

fn read_input_part2(path: &str) -> Vec<(usize, usize)> {
    let data = fs::read_to_string(path).expect("error reading file");
    let split = data.find('\n').unwrap();
    let data: Vec<(usize, usize)> = data[split + 1..]
        .trim()
        .split(',')
        .map(|x| match x {
            "x" => 0,
            _ => x.parse().unwrap(),
        })
        .enumerate()
        .collect();
    data.iter()
        .filter(|(_, x)| *x != 0)
        .map(|(i, x)| (((x - i % x) % x), *x))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    //use std::env;

    #[test]
    fn test_part2() {
        //let path = env::current_dir().unwrap();
        //println!("The current directory is {}", path.display());
        assert_eq!(b_helper("input/test2"), "3417");
        assert_eq!(b_helper("input/test"), "1261476");
    }
}
