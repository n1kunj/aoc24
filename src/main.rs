mod day_output;
mod days;
pub mod direction;
pub mod facing;
pub mod map;

use days::DAYS;

use clap::Parser;
use day_output::DayOutput;
use std::{
    path::{Path, PathBuf},
    time::Instant,
};

#[derive(Parser, Debug)]
struct Args {
    day: String,
    input: Option<String>,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let mut f: Option<&fn(&str, output: &mut DayOutput) -> ()> = None;

    for day in DAYS {
        if day.0 == args.day {
            f = Some(&day.1);
            break;
        }
    }

    let f = f.ok_or(format!("Unknown name {}", args.day))?;
    let all_inputs_dir = format!("./inputs/{}", args.day);

    let load_and_run = |input: &str| {
        let path = |file_name: &str| -> PathBuf {
            let path_name = format!("{}/{}/{}.txt", all_inputs_dir, input, file_name);
            Path::new(&path_name).to_owned()
        };

        let parse_part = |res: std::io::Result<String>| -> Option<i64> {
            match res {
                Ok(s) => Some(s.parse::<i64>().unwrap()),
                Err(_) => None,
            }
        };

        let input_str = std::fs::read_to_string(path("input")).unwrap();
        let part1 = parse_part(std::fs::read_to_string(path("part1")));
        let part2 = parse_part(std::fs::read_to_string(path("part2")));

        run(f, input, &input_str, part1, part2);
    };

    match &args.input {
        Some(input) => load_and_run(input),
        None => {
            let mut dir_names = std::fs::read_dir(Path::new(&all_inputs_dir))
                .unwrap()
                .filter_map(|dir| {
                    let dir = dir.unwrap();
                    match dir.file_type().unwrap().is_dir() {
                        true => Some(dir.file_name().to_str().unwrap().to_owned()),
                        false => None,
                    }
                })
                .collect::<Vec<_>>();

            dir_names.sort();
            // Run examples first
            dir_names.retain_mut(|dir_name| {
                if dir_name.starts_with("example") {
                    load_and_run(dir_name);
                    return false;
                }
                true
            });

            // Run everything except reals second
            dir_names.retain_mut(|dir_name| {
                if !dir_name.starts_with("real") {
                    load_and_run(dir_name);
                    return false;
                }
                true
            });

            // Run everything remaining
            for dir_name in dir_names.iter() {
                load_and_run(dir_name);
            }
        }
    };

    Ok(())
}

fn run(
    f: &fn(&str, output: &mut DayOutput),
    name: &str,
    input: &str,
    part1: Option<i64>,
    part2: Option<i64>,
) {
    println!("[{name}] Running...");

    let mut res = DayOutput::new();
    let before = Instant::now();
    f(input, &mut res);
    let after = Instant::now();
    let dur = after - before;
    println!("    [{name}] Took {dur:#?}");

    compare_result(name, 1, part1, res.get_part1());
    compare_result(name, 2, part2, res.get_part2());
}

fn compare_result(name: &str, part: i64, expected: Option<i64>, actual: Option<i64>) {
    match (actual, expected) {
        (None, None) => {
            println!("    [{name}] Part {part}: No result nor expected result")
        }
        (None, Some(e)) => {
            println!("    [{name}] Part {part}: No result, but expected {e}")
        }
        (Some(a), None) => {
            println!("    [{name}] Part {part}: Result was {a}")
        }
        (Some(a), Some(e)) => {
            if a == e {
                println!("    [{name}] Part {part}: PASS Result was {a} as expected")
            } else {
                println!("    [{name}] Part {part}: FAIL Result was {a} but expected {e}")
            }
        }
    }
}
