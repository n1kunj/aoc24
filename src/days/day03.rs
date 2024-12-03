use crate::day_output::DayOutput;
use regex::{Captures, Regex};

pub fn main(input: &str, output: &mut DayOutput) {
    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    fn mul(c: &Captures) -> i64 {
        let get = |i: usize| c.get(i).unwrap().as_str().parse::<i64>().unwrap();
        get(1) * get(2)
    }
    let part1 = mul_re.captures_iter(input).map(|c| mul(&c)).sum();
    output.part1(part1);

    #[derive(Debug)]
    enum Command {
        Add(i64),
        Do,
        Dont,
    }

    let mut commands = Vec::<(usize, Command)>::new();

    mul_re.captures_iter(input).for_each(|c| {
        commands.push((c.get(0).unwrap().start(), Command::Add(mul(&c))));
    });
    do_re.find_iter(input).for_each(|m| {
        commands.push((m.start(), Command::Do));
    });
    dont_re.find_iter(input).for_each(|m| {
        commands.push((m.start(), Command::Dont));
    });

    commands.sort_by_key(|c| c.0);

    let mut part2 = 0i64;
    let mut do_add = true;
    for c in commands {
        match c.1 {
            Command::Add(v) => {
                if do_add {
                    part2 += v
                }
            }
            Command::Do => do_add = true,
            Command::Dont => do_add = false,
        }
    }
    output.part2(part2);
}
