use std::collections::{hash_map::Entry, HashMap};

use crate::{
    day_output::DayOutput,
    direction::{Direction, DIRECTIONS},
    map::{Map, Row},
};

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut iter_lines = input.lines();
    let threshold = iter_lines.next().unwrap().parse::<usize>().unwrap();
    iter_lines.next();
    let mut rows = Vec::<Row<Tile>>::new();
    let mut sp: Option<(usize, usize)> = None;
    let mut ep: Option<(usize, usize)> = None;
    for (y, line) in iter_lines.enumerate() {
        let mut tiles = Vec::<Tile>::new();
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'S' => {
                    assert!(sp.is_none());
                    sp = Some((x, y));
                    Tile::Empty
                }
                'E' => {
                    assert!(ep.is_none());
                    ep = Some((x, y));
                    Tile::Empty
                }
                _ => panic!(),
            };
            tiles.push(tile);
        }
        rows.push(Row { tiles });
    }
    let map = Map { rows };
    let sp = sp.unwrap();
    let ep = ep.unwrap();

    let mut heads = Vec::<(isize, isize)>::new();
    let mut visited = HashMap::<(isize, isize), usize>::new();
    let mut picos = 0usize;
    heads.push((sp.0 as isize, sp.1 as isize));
    visited.insert(heads[0], picos);
    let mut next_heads = Vec::<(isize, isize)>::new();
    while !heads.is_empty() {
        picos += 1;
        for pos in heads.drain(..) {
            for d in DIRECTIONS {
                let npos = Direction::go(pos, d);
                if map.at(npos).unwrap() == Tile::Empty {
                    if let Entry::Vacant(e) = visited.entry(npos) {
                        e.insert(picos);
                        next_heads.push(npos);
                    }
                }
            }
        }
        heads.append(&mut next_heads);
    }
    assert!(visited.contains_key(&(ep.0 as isize, ep.1 as isize)));

    let mut vrows = Vec::<Row<Option<usize>>>::new();
    for y in 0..map.rows.len() {
        let mut steps_to = Vec::<Option<usize>>::new();
        for x in 0..map.rows[0].tiles.len() {
            steps_to.push(visited.get(&(x as isize, y as isize)).copied());
        }
        vrows.push(Row { tiles: steps_to });
    }
    let vmap = Map { rows: vrows };

    // Part 1
    {
        let mut part1_savings = HashMap::<usize, usize>::new();

        for (pos, steps) in visited.iter() {
            for d in DIRECTIONS {
                let npos = Direction::go_n(*pos, d, 2);
                if let Some(nsteps) = vmap.at(npos).flatten() {
                    if nsteps > *steps + 2 {
                        let saved = nsteps - steps - 2;
                        *part1_savings.entry(saved).or_insert(0) += 1;
                    }
                }
            }
        }

        let mut part1 = 0usize;
        for (saving, count) in part1_savings.iter() {
            if *saving >= threshold {
                part1 += *count;
            }
        }
        output.part1(part1.to_string());
    }

    // Part 2
    {
        let mut part2_savings = HashMap::<usize, usize>::new();

        for (pos, steps) in visited.iter() {
            for x in -20..=20isize {
                for y in -20..=20isize {
                    let cheat_dur = (x.abs() + y.abs()) as usize;
                    if (2..=20).contains(&cheat_dur) {
                        let npos = (pos.0 + x, pos.1 + y);
                        if let Some(nsteps) = vmap.at(npos).flatten() {
                            if nsteps > *steps + cheat_dur {
                                let saved = nsteps - steps - cheat_dur;
                                *part2_savings.entry(saved).or_insert(0) += 1;
                            }
                        }
                    }
                }
            }
        }

        let mut part2 = 0usize;
        for (saving, count) in part2_savings.iter() {
            if *saving >= threshold {
                part2 += *count;
            }
        }
        output.part2(part2.to_string());
    }
}
