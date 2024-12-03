mod days;
mod part_result;

use days::DAYS;

use clap::Parser;
use part_result::PartResult;
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

    let mut f: Option<&fn(&str, output: &mut PartResult) -> ()> = None;

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
            let mut has_example = false;
            let mut has_real = false;
            let mut dir_names = Vec::<String>::new();
            for dir in std::fs::read_dir(Path::new(&all_inputs_dir)).unwrap() {
                let dir = dir.unwrap();
                if dir.file_type().unwrap().is_dir() {
                    match dir.file_name().to_str().unwrap() {
                        "example" => has_example = true,
                        "real" => has_real = true,
                        other => dir_names.push(other.to_owned()),
                    }
                }
            }
            // Run example first
            if has_example {
                load_and_run("example");
            }
            // Then everything except real next
            for dir_name in dir_names {
                load_and_run(&dir_name);
            }
            // Then finally real
            if has_real {
                load_and_run("real");
            }
        }
    };

    Ok(())
}

fn run(
    f: &fn(&str, output: &mut PartResult),
    name: &str,
    input: &str,
    part1: Option<i64>,
    part2: Option<i64>,
) {
    println!("[{name}] Running...");

    let mut res = PartResult::new();
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
