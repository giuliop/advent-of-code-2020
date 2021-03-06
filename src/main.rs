use advent2020::day1;
use advent2020::day10;
use advent2020::day11;
use advent2020::day12;
use advent2020::day13;
use advent2020::day14;
use advent2020::day15;
use advent2020::day16;
use advent2020::day17;
use advent2020::day18;
use advent2020::day19;
use advent2020::day20;
use advent2020::day21;
use advent2020::day22;
use advent2020::day23;
use advent2020::day24;
use advent2020::day25;
use advent2020::day2;
use advent2020::day3;
use advent2020::day4;
use advent2020::day5;
use advent2020::day6;
use advent2020::day7;
use advent2020::day8;
use advent2020::day9;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let problem = args.get(1).map(|x| x.as_str()).unwrap_or("None");
    let result = match problem {
        "1a" => day1::a(),
        "1b" => day1::b(),
        "2a" => day2::a(),
        "2b" => day2::b(),
        "3a" => day3::a(),
        "3b" => day3::b(),
        "4a" => day4::a(),
        "4b" => day4::b(),
        "5a" => day5::a(),
        "5b" => day5::b(),
        "6a" => day6::a(),
        "6b" => day6::b(),
        "7a" => day7::a(),
        "7b" => day7::b(),
        "8a" => day8::a(),
        "8b" => day8::b(),
        "9a" => day9::a(),
        "9b" => day9::b(),
        "10a" => day10::a(),
        "10b" => day10::b(),
        "11a" => day11::a(),
        "11b" => day11::b(),
        "12a" => day12::a(),
        "12b" => day12::b(),
        "13a" => day13::a(),
        "13b" => day13::b(),
        "14a" => day14::a(),
        "14b" => day14::b(),
        "15a" => day15::a(),
        "15b" => day15::b(),
        "16a" => day16::a(),
        "16b" => day16::b(),
        "17a" => day17::a(),
        "17b" => day17::b(),
        "18a" => day18::a(),
        "18b" => day18::b(),
        "19a" => day19::a(),
        "19b" => day19::b(),
        "20a" => day20::a(),
        "20b" => day20::b(),
        "21a" => day21::a(),
        "21b" => day21::b(),
        "22a" => day22::a(),
        "22b" => day22::b(),
        "23a" => day23::a(),
        "23b" => day23::b(),
        "24a" => day24::a(),
        "24b" => day24::b(),
        "25a" => day25::a(),
        "25b" => day25::b(),
        "None" => "Please supply a problem".to_string(),
        _ => "Not solved yet".to_string(),
    };

    println!("\n{}", result);
}
