use std::collections::HashMap;

use crate::day_output::DayOutput;

pub fn main(input: &str, output: &mut DayOutput) {
    let mut stones = Vec::<usize>::new();
    for num in input.split_ascii_whitespace() {
        stones.push(num.parse::<usize>().unwrap());
    }
    let stones = stones;

    let get_stone_counts_after_iters = |iters: usize| -> usize {
        let mut cur_stone_counts = HashMap::<usize, usize>::new();
        let mut next_stone_counts = HashMap::<usize, usize>::new();
        let mut memo = HashMap::<usize, (usize, Option<usize>)>::new();
        for stone in stones.iter() {
            *cur_stone_counts.entry(*stone).or_default() += 1;
        }
        for _ in 0..iters {
            for (s, count) in cur_stone_counts.drain() {
                let mut add_next = |s| *next_stone_counts.entry(s).or_default() += count;
                if s == 0 {
                    add_next(1);
                } else {
                    match memo.get(&s) {
                        Some((l, r)) => {
                            add_next(*l);
                            if let Some(r) = r {
                                add_next(*r);
                            }
                        }
                        None => {
                            let s_str = format!("{s}");
                            let num_digits = s_str.len();
                            if num_digits % 2 == 0 {
                                let left = s_str[..num_digits / 2].parse::<usize>().unwrap();
                                let right = s_str[num_digits / 2..].parse::<usize>().unwrap();
                                memo.insert(s, (left, Some(right)));
                                add_next(left);
                                add_next(right);
                            } else {
                                let left = s * 2024;
                                memo.insert(s, (left, None));
                                add_next(left);
                            }
                        }
                    }
                }
            }
            cur_stone_counts.extend(next_stone_counts.drain());
        }
        cur_stone_counts.values().sum()
    };

    output.part1(get_stone_counts_after_iters(25) as i64);
    output.part2(get_stone_counts_after_iters(75) as i64);
}
