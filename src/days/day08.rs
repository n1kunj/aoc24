use std::collections::{HashMap, HashSet};

use crate::{
    day_output::DayOutput,
    map::{Map, Row},
};

#[derive(Copy, Clone)]
enum Tile {
    Nothing,
    Antenna(char),
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut rows = Vec::<Row<Tile>>::new();
    for line in input.lines() {
        let mut tiles = Vec::<Tile>::new();
        for c in line.chars() {
            let tile = match c {
                '.' => Tile::Nothing,
                a => Tile::Antenna(a),
            };
            tiles.push(tile);
        }
        rows.push(Row { tiles });
    }
    let map = Map { rows };

    let mut char_to_antennas = HashMap::<char, Vec<(isize, isize)>>::new();

    for (y, row) in map.rows.iter().enumerate() {
        for (x, tile) in row.tiles.iter().enumerate() {
            match tile {
                Tile::Nothing => (),
                Tile::Antenna(c) => char_to_antennas
                    .entry(*c)
                    .or_default()
                    .push((x as isize, y as isize)),
            }
        }
    }

    let mut antinodes = HashSet::<(isize, isize)>::new();
    let mut resonants = HashSet::<(isize, isize)>::new();

    for (_c, antennas) in char_to_antennas.iter() {
        for (i, a) in antennas.iter().enumerate() {
            for b in antennas[i + 1..].iter() {
                let (ax, ay) = *a;
                let (bx, by) = *b;
                let dx = ax - bx;
                let dy = ay - by;

                // Part 1 antinodes.
                let n0 = (ax + dx, ay + dy);
                let n1 = (bx - dx, by - dy);
                let mut add_antinode = |n: (isize, isize)| -> bool {
                    match map.at(n) {
                        Some(_) => {
                            antinodes.insert(n);
                            true
                        }
                        None => false,
                    }
                };
                add_antinode(n0);
                add_antinode(n1);

                // Part 2 resonants.
                let mut add_resonant = |n: (isize, isize)| -> bool {
                    match map.at(n) {
                        Some(_) => {
                            resonants.insert(n);
                            true
                        }
                        None => false,
                    }
                };
                let mut from_a = (ax, ay);
                while add_resonant(from_a) {
                    from_a = (from_a.0 + dx, from_a.1 + dy);
                }
                let mut from_b = (bx, by);
                while add_resonant(from_b) {
                    from_b = (from_b.0 - dx, from_b.1 - dy);
                }
            }
        }
    }
    output.part1(antinodes.len().to_string());
    output.part2(resonants.len().to_string());
}
