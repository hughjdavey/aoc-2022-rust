use std::ops::Range;

use crate::days::day;
use crate::days::day::Input;
use crate::util::to_int;
use crate::util::split_to_vec;

pub struct Day4 {}

impl day::Day for Day4 {

    fn part_one(&self, input: &Input) -> String {
        return get_assignment_pairs(input.as_list.clone()).iter()
            .filter(|(range1, range2)| fully_contains(range1, range2))
            .count()
            .to_string();
    }

    fn part_two(&self, input: &Input) -> String {
        return get_assignment_pairs(input.as_list.clone()).iter()
            .filter(|(range1, range2)| overlaps(range1, range2))
            .count()
            .to_string();
    }
}

fn fully_contains(r1: &Range<i32>, r2: &Range<i32>) -> bool {
    return r1.clone().all(|i| r2.contains(&i)) || r2.clone().all(|i| r1.contains(&i));
}

fn overlaps(r1: &Range<i32>, r2: &Range<i32>) -> bool {
    return r1.clone().any(|i| r2.contains(&i)) || r2.clone().any(|i| r1.contains(&i));
}

fn get_assignment_pairs(input_list: Vec<String>) -> Vec<(Range<i32>, Range<i32>)> {
    return input_list.iter()
        .map(|assignments| split_to_vec(assignments, ","))
        .map(|pair| (split_to_vec(&pair[0], "-"), split_to_vec(&pair[1], "-")))
        .map(|(assignment1, assignment2)| (
            Range { start: to_int(&assignment1[0]), end: to_int(&assignment1[1]) + 1 },
            Range { start: to_int(&assignment2[0]), end: to_int(&assignment2[1]) + 1 }
        )).collect::<Vec<(Range<i32>, Range<i32>)>>();
}
