use std::{collections::HashMap, iter::zip};

use crate::day_output::DayOutput;

pub fn main(input: &str, output: &mut DayOutput) {
    let mut lefts = Vec::<u64>::new();
    let mut rights = Vec::<u64>::new();

    for line in input.lines() {
        let mut tokens = line.split_ascii_whitespace();
        lefts.push(tokens.next().unwrap().parse::<u64>().unwrap());
        rights.push(tokens.next().unwrap().parse::<u64>().unwrap());
        assert!(tokens.next().is_none());
    }
    assert_eq!(lefts.len(), rights.len());

    lefts.sort();
    rights.sort();

    let sum_diffs = zip(&lefts, &rights)
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<u64>() as i64;

    output.part1(sum_diffs.to_string());

    let mut right_counts = HashMap::<u64, u64>::new();
    for r in &rights {
        right_counts.entry(*r).and_modify(|c| *c += 1).or_insert(1);
    }

    let sim_score = lefts
        .iter()
        .map(|l| match right_counts.get(l) {
            Some(c) => l * c,
            None => 0,
        })
        .sum::<u64>() as i64;

    output.part2(sim_score.to_string());
}
