use std::collections::HashMap;

use crate::day_output::DayOutput;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Stripe {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Stripe {
    fn parse(c: char) -> Stripe {
        match c {
            'w' => Stripe::White,
            'u' => Stripe::Blue,
            'b' => Stripe::Black,
            'r' => Stripe::Red,
            'g' => Stripe::Green,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Towel {
    stripes: Vec<Stripe>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Design {
    stripes: Vec<Stripe>,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut iter_lines = input.lines();
    let mut towels = Vec::<Towel>::new();
    for towel in iter_lines.next().unwrap().split(", ") {
        towels.push(Towel {
            stripes: towel.chars().map(Stripe::parse).collect(),
        });
    }

    iter_lines.next();

    let mut designs = Vec::<Design>::new();
    for design in iter_lines {
        designs.push(Design {
            stripes: design.chars().map(Stripe::parse).collect(),
        });
    }
    let towels = towels;
    let designs = designs;

    fn recurse<'a>(
        memo: &mut HashMap<&'a [Stripe], usize>,
        remaining_stripes: &'a [Stripe],
        all_towels: &Vec<Towel>,
    ) -> usize {
        if remaining_stripes.is_empty() {
            return 1;
        }
        if let Some(res) = memo.get(remaining_stripes) {
            return *res;
        }
        let mut num_ways = 0usize;
        for t in all_towels.iter() {
            if remaining_stripes.starts_with(&t.stripes) {
                num_ways += recurse(memo, &remaining_stripes[t.stripes.len()..], all_towels);
            }
        }
        memo.insert(remaining_stripes, num_ways);
        num_ways
    }

    let mut part1 = 0usize;
    let mut part2 = 0usize;

    for d in designs.iter() {
        let mut memo = HashMap::<&[Stripe], usize>::new();
        let num_ways = recurse(&mut memo, &d.stripes, &towels);
        if num_ways > 0 {
            part1 += 1;
        }
        part2 += num_ways;
    }
    output.part1(part1.to_string());
    output.part2(part2.to_string());
}
