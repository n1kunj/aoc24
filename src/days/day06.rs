use std::collections::{HashMap, HashSet};

use crate::day_output::DayOutput;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Space,
    Obstruction,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn go((x, y): (isize, isize), d: Direction) -> (isize, isize) {
        match d {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }

    fn turn_right(d: Direction) -> Direction {
        match d {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Row {
    tiles: Vec<Tile>,
}

struct Map {
    rows: Vec<Row>,
}

impl Map {
    fn at(&self, (x, y): (isize, isize)) -> Option<Tile> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        let row = self.rows.get(y)?;
        row.tiles.get(x).copied()
    }
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut rows = Vec::<Row>::new();
    let mut start_pos: Option<(usize, usize)> = None;
    for (y, line) in input.lines().enumerate() {
        let mut tiles = Vec::<Tile>::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => tiles.push(Tile::Space),
                '#' => tiles.push(Tile::Obstruction),
                '^' => {
                    tiles.push(Tile::Space);
                    assert!(start_pos.is_none());
                    start_pos = Some((x, y));
                }
                _ => panic!(),
            }
        }
        rows.push(Row { tiles });
    }

    let map = Map { rows };

    let mut dir = Direction::Up;
    let start_pos = match start_pos {
        Some(pos) => (pos.0 as isize, pos.1 as isize),
        None => panic!(),
    };
    let mut pos = start_pos;
    let mut path_history = Vec::<((isize, isize), Direction)>::new();

    loop {
        path_history.push((pos, dir));

        let next_pos = Direction::go(pos, dir);
        match map.at(next_pos) {
            Some(tile) => match tile {
                Tile::Space => {
                    pos = next_pos;
                }
                Tile::Obstruction => {
                    dir = Direction::turn_right(dir);
                }
            },
            None => break,
        }
    }

    let unique_positions = path_history.iter().map(|pd| pd.0).collect::<HashSet<_>>();
    output.part1(unique_positions.len() as i64);

    let posdir_index = path_history
        .iter()
        .enumerate()
        .map(|(i, pd)| ((pd.0, pd.1), i))
        .collect::<HashMap<_, _>>();

    let mut loop_count = 0i64;
    let mut considered_obst_pos = HashSet::<(isize, isize)>::new();

    let mut new_visited = HashSet::<((isize, isize), Direction)>::new();
    for (idx, (orig_pos, orig_dir)) in path_history.iter().enumerate() {
        let (obst_pos, _next_dir) = match path_history.get(idx + 1) {
            Some(h) => h,
            None => continue,
        };
        if obst_pos == orig_pos {
            // Hit an obstacle already and turned, skip this.
            continue;
        }
        if *obst_pos == start_pos {
            // Obstacle pos is where we started, disallowed.
            continue;
        }
        if considered_obst_pos.contains(obst_pos) {
            // Already considered an obstacle here with an earlier starting position.
            continue;
        }
        considered_obst_pos.insert(*obst_pos);

        new_visited.clear();
        let mut pos = *orig_pos;
        let mut dir = *orig_dir;
        loop {
            let next_pos = Direction::go(pos, dir);
            let next_tile = if next_pos == *obst_pos {
                Tile::Obstruction
            } else {
                match map.at(next_pos) {
                    Some(tile) => tile,
                    None => break,
                }
            };
            match next_tile {
                Tile::Space => {
                    pos = next_pos;
                }
                Tile::Obstruction => {
                    dir = Direction::turn_right(dir);
                }
            }
            if let Some(i) = posdir_index.get(&(pos, dir)) {
                if *i <= idx {
                    loop_count += 1;
                    break;
                }
            }
            let is_new_visited = new_visited.insert((pos, dir));
            if !is_new_visited {
                loop_count += 1;
                break;
            }
        }
    }

    output.part2(loop_count);
}
