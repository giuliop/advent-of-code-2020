mod day1;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let problem = args.get(1).map(|x| x.as_str()).unwrap_or("None");
    let result = match problem {
        "1a" => day1::part1(),
        "1b" => day1::part2(),
        _ => String::from("Not solved yet"),
    };

    println!("\n{}", result);
}
