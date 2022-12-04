#![warn(clippy::pedantic)]
use aoc_2022::advent_of_code::AdventOfCodeInput;
use aoc_2022::solutions::{day_one, day_three};
use criterion::{criterion_group, criterion_main, Criterion};

fn load_inp(day: u32) -> AdventOfCodeInput {
    AdventOfCodeInput::get_input(day)
}

fn bench_day_one(c: &mut Criterion) {
    let aoc_input = load_inp(1);

    c.bench_function("d1", |b| b.iter(|| day_one::solve(&aoc_input.inp)));
}

fn bench_day_three(c: &mut Criterion) {
    let aoc_input = load_inp(1);

    c.bench_function("d1", |b| b.iter(|| day_three::solve(&aoc_input.inp)));
}

criterion_group!(day_1, bench_day_one, bench_day_three);
criterion_main!(day_1);
