use crate::day_output::DayOutput;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

type DayFn = fn(&str, &mut DayOutput) -> ();

pub const DAYS: &[(&str, DayFn)] = &[
    ("day01", day01::main),
    ("day02", day02::main),
    ("day03", day03::main),
    ("day04", day04::main),
    ("day05", day05::main),
    ("day06", day06::main),
    ("day07", day07::main),
    ("day08", day08::main),
];
