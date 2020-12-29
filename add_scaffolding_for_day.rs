// Do the following for day X
// 1. add the following line to lib.rs
//      pub mod dayX;
// 2. create a dayX.rs file with empty a() and b() functions
// 3. add the following lines to main.rs
//      use advent2020::dayX;
//      "Xa" => dayX::a(),
//      "Xb" => dayX::b(),

use std::fs;

fn parse_day(args: &Vec<String>) -> Option<usize> {
    args.get(1)
        .map(|x| x.as_str())
        .map(|x| x.parse::<usize>())
        .transpose()
        .unwrap_or(Option::None)
}

fn insert_in_file_after(filename: &str, after: &str, insert: &str) {
    let content = fs::read_to_string(filename)
        .expect(&format!("error reading file {}", filename));
    if content.contains(insert) {
        println!("{} -> already in file {}\n", insert, filename);
        return;
    }
    let pos = content
        .find(after)
        .expect(&format!("did not find {} in file {}", after, filename))
        + after.len();
    let content = [&content[..pos], insert, &content[pos..]].concat();
    fs::write(filename, content).expect(&format!("error writing file {}", filename));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(0).unwrap();
    let day = parse_day(&args);
    if let None = day {
        println!("Usage: {} <day number>", cmd);
        println!("E.g., {} 9", cmd);
        return;
    }

    let day: usize = day.unwrap();

    // step 1
    let filename = "src/lib.rs";
    let insert = format!("pub mod day{};\n", day);
    let after = format!("pub mod day{};\n", day - 1);
    insert_in_file_after(filename, &after, &insert);

    // step 2
    let filename = &format!("src/day{}.rs", day);
    if !fs::metadata(filename).is_ok() {
        let content = [
            "pub fn a() -> String {\"\".to_string()}\n",
            "\n",
            "pub fn b() -> String {\"\".to_string()}\n",
        ]
        .concat();
        fs::write(filename, content)
            .expect(&format!("error writing file {}", filename));
    } else {
        println!("{} already exists\n", filename);
    }

    // step 3
    let filename = "src/main.rs";
    let insert = [
        "        ",
        &format!("\"{0}a\" => day{0}::a(),", day),
        "\n        ",
        &format!("\"{0}b\" => day{0}::b(),\n", day),
    ]
    .concat();
    let after = format!("\"{0}b\" => day{0}::b(),\n", day - 1);
    insert_in_file_after(filename, &after, &insert);

    let insert = format!("use advent2020::day{};\n", day);
    let after = format!("use advent2020::day{};\n", day - 1);
    insert_in_file_after(filename, &after, &insert);
}
