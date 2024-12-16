use std::collections::{HashMap, HashSet};

use crate::{
    day_output::DayOutput,
    direction::{Direction, DIRECTIONS},
    map::{Map, Row},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut rows = Vec::<Row<Tile>>::new();
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;
    for (y, line) in input.lines().enumerate() {
        let mut tiles = Vec::<Tile>::new();
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'S' => {
                    assert!(start.is_none());
                    start = Some((x, y));
                    Tile::Empty
                }
                'E' => {
                    assert!(end.is_none());
                    end = Some((x, y));
                    Tile::Empty
                }
                _ => panic!(),
            };
            tiles.push(tile);
        }
        rows.push(Row { tiles });
    }
    let map = Map { rows };
    let start = start.unwrap();
    let end = end.unwrap();

    let move_forwards = |pos: (isize, isize),
                         d: Direction,
                         score: isize|
     -> Option<((isize, isize), Direction, isize)> {
        let npos = Direction::go(pos, d);
        let ntile = map.at(npos)?;
        match ntile {
            Tile::Empty => Some((npos, d, score + 1)),
            Tile::Wall => None,
        }
    };
    let turn_right = |pos: (isize, isize),
                      d: Direction,
                      score: isize|
     -> Option<((isize, isize), Direction, isize)> {
        let nd = Direction::turn_right(d);
        Some((pos, nd, score + 1000))
    };
    let turn_left = |pos: (isize, isize),
                     d: Direction,
                     score: isize|
     -> Option<((isize, isize), Direction, isize)> {
        let nd = Direction::turn_left(d);
        Some((pos, nd, score + 1000))
    };

    // Part 1
    let mut heads = HashMap::<((isize, isize), Direction), isize>::new();
    heads.insert(((start.0 as isize, start.1 as isize), Direction::Right), 0);
    let mut visited = heads.clone();
    let mut next_heads = Vec::<((isize, isize), Direction, isize)>::new();
    while !heads.is_empty() {
        for ((pos, d), score) in heads.drain() {
            for (npos, nd, nscore) in [
                move_forwards(pos, d, score),
                turn_right(pos, d, score),
                turn_left(pos, d, score),
            ]
            .into_iter()
            .flatten()
            {
                let prev_visited = visited.get(&(npos, nd));
                let is_better = match prev_visited {
                    Some(score) => nscore < *score,
                    None => true,
                };
                if is_better {
                    visited.insert((npos, nd), nscore);
                    next_heads.push((npos, nd, nscore));
                }
            }
        }
        for (npos, nd, nscore) in next_heads.drain(..) {
            heads.insert((npos, nd), nscore);
        }
    }

    let part1 = *DIRECTIONS
        .iter()
        .filter_map(|d| visited.get(&((end.0 as isize, end.1 as isize), *d)))
        .min()
        .unwrap();
    output.part1(part1 as i64);

    // Part 2
    let mut heads = HashMap::<((isize, isize), Direction), isize>::new();
    let pos = (end.0 as isize, end.1 as isize);
    for d in DIRECTIONS.iter() {
        if let Some(score) = visited.get(&(pos, *d)) {
            if *score == part1 {
                heads.insert((pos, *d), *score);
            }
        }
    }
    let mut on_best_path = HashSet::<(isize, isize)>::new();
    on_best_path.insert(pos);
    let mut next_heads = Vec::<((isize, isize), Direction, isize)>::new();
    while !heads.is_empty() {
        for ((pos, d), score) in heads.drain() {
            for (npos, nd, nscore) in [
                move_forwards(pos, Direction::turn_around(d), score),
                turn_right(pos, d, score),
                turn_left(pos, d, score),
            ]
            .into_iter()
            .flatten()
            {
                let nscore = score - (nscore - score);
                let nd = Direction::turn_around(nd);
                let prev_visited = visited.get(&(npos, nd));
                let is_best_path = match prev_visited {
                    Some(score) => nscore == *score,
                    None => false,
                };
                if is_best_path {
                    on_best_path.insert(npos);
                    next_heads.push((npos, nd, nscore));
                }
            }
        }
        for (npos, nd, nscore) in next_heads.drain(..) {
            heads.insert((npos, nd), nscore);
        }
    }

    output.part2(on_best_path.len() as i64);
}
