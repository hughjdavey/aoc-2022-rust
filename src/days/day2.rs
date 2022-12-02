use std::collections::HashMap;

use crate::days::day;
use crate::days::day::Input;

pub struct Day2 {}

impl day::Day for Day2 {

    fn part_one(&self, input: &Input) -> String {
        let mut total = 0;
        for game in input.as_list.iter() {
            let (them, us) = get_columns(game);
            total += get_result(them, us) + us;
        }
        return total.to_string();
    }

    fn part_two(&self, input: &Input) -> String {
        let mut total = 0;
        for game in input.as_list.iter() {
            let (them, result) = get_columns(game);
            let us = get_us(them, result);
            total += get_result(them, us) + us;
        }
        return total.to_string();
    }
}

fn get_columns(game: &String) -> (i32, i32) {
    let mappings = get_mappings();
    let split = game.split(" ").collect::<Vec<&str>>();
    return (*mappings.get(split[0]).unwrap(), *mappings.get(split[1]).unwrap());
}

fn get_mappings() -> HashMap<&'static str, i32> {
    return HashMap::from([
        ("A", 1), ("X", 1), ("B", 2), ("Y", 2), ("C", 3), ("Z", 3),
    ]);
}

// 1 = rock, 2 = paper, 3 = scissors
fn get_result(them: i32, us: i32) -> i32 {
    return if them == 3 && us == 1 { 6 }
        else if them == 2 && us == 3 { 6 }
        else if them == 1 && us == 2 { 6 }
        else if them == us { 3 }
        else { 0 }
}

// 1 = rock, 2 = paper, 3 = scissors, 1 = lose, 2 = draw, 3 = win
fn get_us(them: i32, result: i32) -> i32 {
    return if result == 1 && them == 1 { 3 }
        else if result == 1 && them == 2 { 1 }
        else if result == 1 && them == 3 { 2 }
        else if result == 3 && them == 1 { 2 }
        else if result == 3 && them == 2 { 3 }
        else if result == 3 && them == 3 { 1 }
        else { them }
}
