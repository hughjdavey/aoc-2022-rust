use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Result, Write};
use std::process::exit;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Day number must be specified");
        println!("Example usage: `cargo start-day 4`");
        exit(1);
    }

    let day = &args[1].parse::<i32>().unwrap_or(-1);
    if day == &-1 {
        println!("Day must be a number ('{}' was provided)", &args[1]);
        println!("Example usage: `cargo start-day 4`");
        exit(1);
    }

    File::create(format!("inputs/day{}.txt", day))?;
    File::create(format!("example-inputs/day{}.txt", day))?;
    let mut day_file = File::create(format!("src/days/day{}.rs", day))?;
    day_file.write_all(get_day_file(*day).as_bytes())?;

    let mut main_file_ro = File::open("src/main.rs")?;
    let mut contents = String::new();
    main_file_ro.read_to_string(&mut contents)?;

    //"4" => Box::new(days::day4::Day4 {}),
    let mut main_file = OpenOptions::new().write(true).truncate(true).open("src/main.rs")?;
    let new_contents = contents
        .replace(&format!("pub mod day{};", day - 1), &format!("pub mod day{};\n    pub mod day{};", day - 1, day))
        .replace(&format!("Day{} {{}}),", day - 1), &format!("Day{} {{}}),\n        \"{}\" => Box::new(days::day{}::Day{} {{}}),", day - 1, day, day, day));
    main_file.write_all(new_contents.as_bytes())?;
    main_file.flush()?;
    Ok(())
}

fn get_day_file(day: i32) -> String {
    return
"use crate::days::day;
use crate::days::day::Input;

pub struct DayX {}

impl day::Day for DayX {

    fn part_one(&self, input: &Input) -> String {
        return \"p1\".to_string();
    }

    fn part_two(&self, input: &Input) -> String {
        return \"p2\".to_string();
    }
}
".replace("X", &*day.to_string());
}
