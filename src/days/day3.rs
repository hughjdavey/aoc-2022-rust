use std::collections::HashMap;

use crate::days::day;
use crate::days::day::Input;

pub struct Day3 {}

impl day::Day for Day3 {

    fn part_one(&self, input: &Input) -> String {
        let priorities = get_priorities();
        return input.as_list.iter()
            .map(|sack| sack.split_at(sack.len() / 2))
            .map(|(compartment1, compartment2)| compartment1.chars()
                .filter(|c| compartment2.contains(*c))
                .collect::<Vec<char>>().pop().unwrap()
            )
            .map(|item| priorities.get(&item).unwrap())
            .sum::<i32>()
            .to_string();
    }

    fn part_two(&self, input: &Input) -> String {
        let priorities = get_priorities();
        return input.as_list.chunks(3)
            .map(|group| group.iter().collect::<Vec<&String>>())
            .map(|group| (group[0], group[1], group[2]))
            .map(|(sack1, sack2, sack3)| sack1.chars()
                .filter(|c| sack2.contains(*c) && sack3.contains(*c))
                .collect::<Vec<char>>().pop().unwrap()
            )
            .map(|item| priorities.get(&item).unwrap())
            .sum::<i32>()
            .to_string();
    }
}

fn get_priorities() -> HashMap<char, i32> {
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let numbers: Vec<i32> = (1..53).collect();
    let r = letters.iter().zip(numbers.iter());
    let mut priorities = HashMap::new();
    for (l, n) in r {
        priorities.insert(*l, *n);
    }
    return priorities;
}
