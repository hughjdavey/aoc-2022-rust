# aoc-2022-rust

Attempt at solving some [Advent of Code 2022](https://adventofcode.com/2022) puzzles using [Rust](https://www.rust-lang.org) which I have never used before.

Thanks to [MetroWind/advent2019](https://github.com/MetroWind/advent2019) and [fspoettel/advent-of-code-rust](https://github.com/fspoettel/advent-of-code-rust) for inspiration on how to structure the project.

### Adding a day (todo automate!)

_Let us say you are adding day 4_

* Add files `inputs/day4.txt` and `example_inputs/day4.txt` and add AOC input
* Add `pub mod day4;` into the `mod days {...}` block in `main.rs`
* Add `"4" => Box::new(days::day4::Day4 {}),` into the `match n {...}` block in `main.rs`
* Create file `src/days/day4.rs` and implement the `Day` trait like so:
```rust
use crate::days::day;
use crate::days::day::Input;

pub struct Day4 {}

impl day::Day for Day4 {

    fn part_one(&self, input: &Input) -> String {
        return "p1".to_string();
    }

    fn part_two(&self, input: &Input) -> String {
        return "p2".to_string();
    }
}
```
* The type `Input` passed into `part_one` and `part_two` is defined in `day.rs`, and has
two useful methods on it - `as_string` and `as_list`. These will get you the relevant day's input
parsed as either a `String` or a `Vec<String>`. See below for how to use real or example input.

### Running

##### Run a day with example input

`cargo run-examples 1`

##### Run a day with real input

`cargo run 1`

##### Run all days with real input

`cargo run`
