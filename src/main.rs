use std::{env, fs};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::days::day::Input;

mod days {
    pub mod day;
    pub mod day1;
    pub mod day2;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // this will be true if we run `cargo run-examples 1` which maps to `cargo run ex 1` (see .cargo/config)
        if &args[1] == "ex" {
            run_day(&args[2], true)
        }
        else {
            run_day(&args[1], false);
        }
        return;
    }

    for n in 1..25 {
        run_day(&n.to_string(), false);
    }
}

fn run_day(n: &str, ex: bool) {
    let day = get_day(n);
    let input = get_input(n, ex);

    println!("=== DAY {} ===", n);
    println!("Part 1: {}", day.part_one(&input));
    println!("Part 2: {}", day.part_two(&input));
    println!();
}

fn get_day(n: &str) -> Box<dyn days::day::Day> {
    match n {
        "1" => Box::new(days::day1::Day1 {}),
        "2" => Box::new(days::day2::Day2 {}),
        _ => panic!("Unknown day: {}", n),
    }
}

fn get_input(n: &str, ex: bool) -> days::day::Input {
    let directory = if ex { "example-inputs" } else { "inputs" };
    let path = format!("{}\\day{}.txt", directory, n);
    let str = fs::read_to_string(&path).expect("Can't parse file");
    let list = read_lines(&path);
    return Input { as_string: str, as_list: list }
}

// see https://stackoverflow.com/a/35820003
fn read_lines(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Can't open file");
    return BufReader::new(file).lines()
        .map(|l| l.expect("Can't parse line"))
        .collect()
}
