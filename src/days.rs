use crate::part_result::PartResult;

mod day01;
mod day02;

type DayFn = fn(&str, &mut PartResult) -> ();

pub const DAYS: &[(&str, DayFn)] = &[("day01", day01::main), ("day02", day02::main)];
