use advent2020::day1;
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
        "None" => "Please supply a problem".to_string(),
        _ => "Not solved yet".to_string(),
    };

    println!("\n{}", result);
}
