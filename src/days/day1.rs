use crate::days::day;
use crate::days::day::Input;
use crate::util::to_int;

pub struct Day1 {}

impl day::Day for Day1 {

    fn part_one(&self, input: &Input) -> String {
        let totals = sorted_calorie_totals(input);
        return totals[0].to_string();
    }

    fn part_two(&self, input: &Input) -> String {
        let totals = sorted_calorie_totals(input);
        return (&totals[..3]).iter().sum::<i32>().to_string();
    }
}

fn sorted_calorie_totals(input: &Input) -> Vec<i32> {
    let mut totals = Vec::<i32>::new();
    for cals in input.as_list.to_vec() {
        if cals.is_empty() {
            totals.push(0);
            continue;
        }
        let total = totals.pop().unwrap_or(0);
        totals.push(total + to_int(&cals));
    }
    totals.sort_by(|a, b| b.cmp(a));
    return totals;
}
