use crate::day_output::DayOutput;

mod day01;
mod day02;
mod day03;

type DayFn = fn(&str, &mut DayOutput) -> ();

pub const DAYS: &[(&str, DayFn)] = &[
    ("day01", day01::main),
    ("day02", day02::main),
    ("day03", day03::main),
];
