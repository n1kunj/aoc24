use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use crate::day_output::DayOutput;

#[derive(Debug)]
struct Rule {
    first: usize,
    second: usize,
}

#[derive(Debug)]
struct Update {
    pages: Vec<usize>,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut rules = Vec::<Rule>::new();
    let mut updates = Vec::<Update>::new();

    let mut is_rules = true;
    for line in input.lines() {
        if is_rules {
            if line.is_empty() {
                is_rules = false;
                continue;
            }
            let mut rule_it = line.split('|');
            let first = rule_it.next().unwrap().parse::<usize>().unwrap();
            let second = rule_it.next().unwrap().parse::<usize>().unwrap();
            rules.push(Rule { first, second });
        } else {
            updates.push(Update {
                pages: line
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect(),
            });
        }
    }

    let mut forward_rules = HashMap::<usize, HashSet<usize>>::new();
    for rule in rules {
        forward_rules
            .entry(rule.first)
            .or_default()
            .insert(rule.second);
    }

    let pages_sorted = |a: usize, b: usize| match forward_rules.get(&a) {
        Some(afters) => afters.contains(&b),
        None => false,
    };

    let pages_comparator = |a: usize, b: usize| match pages_sorted(a, b) {
        true => Ordering::Less,
        false => Ordering::Greater,
    };

    fn middle_page(u: &Update) -> usize {
        u.pages[u.pages.len() / 2]
    }

    let part1 = updates
        .iter()
        .filter(|u| u.pages.is_sorted_by(|a, b| pages_sorted(*a, *b)))
        .map(middle_page)
        .sum::<usize>() as i64;

    output.part1(part1.to_string());

    let part2 = updates
        .iter()
        .filter(|u| !u.pages.is_sorted_by(|a, b| pages_sorted(*a, *b)))
        .map(|u| {
            let mut pages = u.pages.clone();
            pages.sort_by(|a, b| pages_comparator(*a, *b));
            Update { pages }
        })
        .map(|u| middle_page(&u))
        .sum::<usize>() as i64;

    output.part2(part2.to_string());
}
