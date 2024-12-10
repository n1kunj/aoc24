use std::collections::HashMap;

use crate::day_output::DayOutput;

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
}

struct Row {
    tiles: Vec<u32>,
}

struct Map {
    rows: Vec<Row>,
}

impl Map {
    fn at(&self, (x, y): (isize, isize)) -> Option<u32> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        let row = self.rows.get(y)?;
        row.tiles.get(x).copied()
    }
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut rows: Vec<Row> = Vec::<Row>::new();
    let mut heads = Vec::<(usize, usize)>::new();
    for (y, line) in input.lines().enumerate() {
        let mut tiles = Vec::<u32>::new();
        for (x, c) in line.chars().enumerate() {
            let height = c.to_digit(10).unwrap();
            tiles.push(height);
            if height == 0 {
                heads.push((x, y));
            }
        }
        rows.push(Row { tiles });
    }
    let map = Map { rows };

    let mut part1 = 0usize;
    let mut part2 = 0usize;
    for head in heads.iter() {
        let mut to_be_visited = HashMap::<(isize, isize), usize>::new();
        let mut next_to_be_visited = Vec::<((isize, isize), usize)>::new();

        let mut cur_height = 0;
        to_be_visited.insert((head.0 as isize, head.1 as isize), 1);

        while !to_be_visited.is_empty() {
            next_to_be_visited.clear();
            for (pos, trails) in to_be_visited.drain() {
                for dir in &[
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                ] {
                    let next_pos = Direction::go(pos, *dir);
                    if let Some(h) = map.at(next_pos) {
                        if h == cur_height + 1 {
                            next_to_be_visited.push((next_pos, trails));
                        }
                    }
                }
            }
            cur_height += 1;
            for (next_pos, trails) in next_to_be_visited.drain(..) {
                *to_be_visited.entry(next_pos).or_default() += trails;
            }

            if cur_height == 9 {
                part1 += to_be_visited.len();
                part2 += to_be_visited.values().sum::<usize>();
                break;
            }
        }
    }
    output.part1(part1 as i64);
    output.part2(part2 as i64);
}
