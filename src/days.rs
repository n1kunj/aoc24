use crate::part_result::PartResult;

mod day01;

pub const DAYS: &[(
    &'static str,
    fn(&str, &mut PartResult) -> (),
)] = &[("day01", day01::main)];
