use std::collections::{HashMap, HashSet};

use crate::{
    day_output::DayOutput,
    direction::{Direction, DIRECTIONS},
    map::{Map, Row},
};

struct Byte {
    pos: (isize, isize),
    idx: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Byte(usize),
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut bytes = Vec::<Byte>::new();
    let mut is_bytes = true;
    let mut meta: Option<(isize, isize, usize)> = None;
    for line in input.lines() {
        if is_bytes {
            if line.is_empty() {
                is_bytes = false;
            } else {
                let mut elems = line.split(',');
                let x = elems.next().unwrap().parse::<isize>().unwrap();
                let y = elems.next().unwrap().parse::<isize>().unwrap();
                assert!(elems.next().is_none());
                bytes.push(Byte {
                    pos: (x, y),
                    idx: bytes.len(),
                });
            }
        } else {
            let mut elems = line.split_ascii_whitespace();
            let x = elems.next().unwrap().parse::<isize>().unwrap();
            let y = elems.next().unwrap().parse::<isize>().unwrap();
            let num_falling = elems.next().unwrap().parse::<usize>().unwrap();
            assert!(elems.next().is_none());
            meta = Some((x, y, num_falling));
        }
    }

    let (sx, sy, part1_falling) = meta.unwrap();
    let sp = (0isize, 0isize);
    let ep = (sx, sy);

    let bps = HashMap::<(isize, isize), usize>::from_iter(
        bytes.iter().enumerate().map(|(i, b)| (b.pos, i)),
    );

    let mut rows = Vec::<Row<Tile>>::new();
    for y in 0..=sy {
        rows.push(Row {
            tiles: Vec::from_iter((0..=sx).map(|x| match bps.get(&(x, y)) {
                Some(i) => Tile::Byte(*i),
                None => Tile::Empty,
            })),
        });
    }
    let map = Map { rows };

    #[allow(dead_code)]
    fn print_map(map: &Map<Tile>, max_fallen_idx: usize) {
        for row in map.rows.iter() {
            for t in row.tiles.iter() {
                let c = match t {
                    Tile::Empty => '.',
                    Tile::Byte(i) => {
                        if *i <= max_fallen_idx {
                            '#'
                        } else {
                            '.'
                        }
                    }
                };
                print!("{c}");
            }
            println!();
        }
    }

    let steps_to_exit = |max_fallen_idx: usize| -> Option<usize> {
        assert_eq!(map.at(sp).unwrap(), Tile::Empty);
        assert_eq!(map.at(ep).unwrap(), Tile::Empty);

        let mut visited = HashSet::<(isize, isize)>::new();
        let mut heads = HashSet::<(isize, isize)>::new();
        visited.insert(sp);
        heads.insert(sp);
        let mut next_heads = HashSet::<(isize, isize)>::new();
        let mut steps = 0usize;
        while !heads.is_empty() {
            steps += 1;
            for pos in heads.drain() {
                for d in DIRECTIONS {
                    let np = Direction::go(pos, d);
                    if np == ep {
                        return Some(steps);
                    }
                    let is_empty = match map.at(np) {
                        Some(t) => match t {
                            Tile::Empty => true,
                            Tile::Byte(i) => i > max_fallen_idx,
                        },
                        None => false,
                    };

                    if is_empty && !visited.contains(&np) {
                        visited.insert(np);
                        next_heads.insert(np);
                    }
                }
            }
            heads.extend(next_heads.drain());
        }
        None
    };

    let part1 = steps_to_exit(part1_falling - 1).unwrap();
    output.part1(part1.to_string());

    let first_failing_idx = bytes.partition_point(|b| steps_to_exit(b.idx).is_some());
    let first_byte = &bytes[first_failing_idx];
    output.part2(format!("{},{}", first_byte.pos.0, first_byte.pos.1));
}
