#![warn(clippy::pedantic)]
use aoc_2022::advent_of_code::AdventOfCodeInput;
use aoc_2022::solutions::{day_one, day_two};

use std::time::Instant;
fn main() {
    let start = Instant::now();
    for i in 14..=14 {
        let input = AdventOfCodeInput::get_input(i).inp;
        match i {
            1 => println!("{}", day_one::solve(&input)),
            2 => println!("{}", day_two::solve(&input)),

            _ => unimplemented!(),
        }
    }
    let end = Instant::now();
    println!("Total runtime {:?}", end - start);
}
