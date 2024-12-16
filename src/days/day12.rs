use std::collections::HashSet;

use crate::{
    day_output::DayOutput,
    direction::{Direction, DIRECTIONS},
    map::{Map, Row},
};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Plant {
    c: char,
}

#[derive(Debug, Clone)]
struct Region {
    #[allow(dead_code)]
    plant: Plant,
    locs: HashSet<(isize, isize)>,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut rows = Vec::<Row<Plant>>::new();
    for line in input.lines() {
        let mut plants = Vec::<Plant>::new();
        for c in line.chars() {
            plants.push(Plant { c });
        }
        rows.push(Row { tiles: plants });
    }
    let map = Map { rows };

    let mut touched = HashSet::<(isize, isize)>::new();

    let mut regions = Vec::<Region>::new();

    for (y, row) in map.rows.iter().enumerate() {
        for (x, plant) in row.tiles.iter().enumerate() {
            let start_pos = (x as isize, y as isize);
            if touched.contains(&start_pos) {
                continue;
            }
            // Search to find touching plants.
            let mut region = HashSet::<(isize, isize)>::new();
            let mut next_to_search = HashSet::<(isize, isize)>::new();

            next_to_search.insert(start_pos);
            region.insert(start_pos);

            let mut next_next = HashSet::<(isize, isize)>::new();

            while !next_to_search.is_empty() {
                for p in next_to_search.drain() {
                    for d in DIRECTIONS {
                        let next_pos = Direction::go(p, d);
                        if region.contains(&next_pos) {
                            continue;
                        }

                        if let Some(next_plant) = map.at(next_pos) {
                            if next_plant == *plant {
                                region.insert(next_pos);
                                next_next.insert(next_pos);
                            }
                        }
                    }
                }
                next_to_search.extend(next_next.drain());
            }
            touched.extend(region.iter());
            regions.push(Region {
                plant: *plant,
                locs: region,
            });
        }
    }
    drop(touched);
    let regions = regions;

    let mut part1 = 0usize;
    let mut part2 = 0usize;

    for region in regions.iter() {
        let mut fences = HashSet::<((isize, isize), Direction)>::new();
        for loc in region.locs.iter() {
            for d in [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
            ] {
                let adj = Direction::go(*loc, d);
                if !region.locs.contains(&adj) {
                    fences.insert((adj, d));
                }
            }
        }
        part1 += fences.len() * region.locs.len();

        let mut seen_fences = HashSet::<((isize, isize), Direction)>::new();
        let mut sides = 0usize;
        for (start_pos, d) in fences.iter() {
            if seen_fences.contains(&(*start_pos, *d)) {
                continue;
            }

            // Find fences with the same orientation and adjacent positions
            let mut found = HashSet::<(isize, isize)>::new();
            let mut next_to_search = HashSet::<(isize, isize)>::new();

            found.insert(*start_pos);
            next_to_search.insert(*start_pos);

            let mut next_next = HashSet::<(isize, isize)>::new();
            let adj_ds = match d {
                Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
                Direction::Right | Direction::Left => [Direction::Up, Direction::Down],
            };

            while !next_to_search.is_empty() {
                for pos in next_to_search.drain() {
                    for adj_d in adj_ds {
                        let adj_pos = Direction::go(pos, adj_d);
                        if fences.contains(&(adj_pos, *d)) && !found.contains(&adj_pos) {
                            found.insert(adj_pos);
                            next_next.insert(adj_pos);
                        }
                    }
                }
                next_to_search.extend(next_next.drain());
            }
            for found in found.iter() {
                seen_fences.insert((*found, *d));
            }
            sides += 1;
        }
        part2 += sides * region.locs.len();
    }

    output.part1(part1 as i64);
    output.part2(part2 as i64);
}
