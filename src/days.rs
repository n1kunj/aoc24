use crate::day_output::DayOutput;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

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
    ("day09", day09::main),
    ("day10", day10::main),
    ("day11", day11::main),
    ("day12", day12::main),
    ("day13", day13::main),
    ("day14", day14::main),
    ("day15", day15::main),
    ("day16", day16::main),
    ("day17", day17::main),
    ("day18", day18::main),
    ("day19", day19::main),
    ("day20", day20::main),
    ("day21", day21::main),
    ("day22", day22::main),
    ("day23", day23::main),
    ("day24", day24::main),
    ("day25", day25::main),
];
