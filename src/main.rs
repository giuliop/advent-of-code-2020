use advent2020::day1;
use advent2020::day2;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let problem = args.get(1).map(|x| x.as_str()).unwrap_or("None");
    let result = match problem {
        "1a" => day1::a(),
        "1b" => day1::b(),
        "2a" => day2::a(),
        "2b" => day2::b(),
        "None" => "Please supply a problem".to_string(),
        _ => "Not solved yet".to_string(),
    };

    println!("\n{}", result);
}