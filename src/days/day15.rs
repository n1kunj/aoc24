use crate::{
    day_output::DayOutput,
    direction::Direction,
    map::{Map, Row},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Box,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile2 {
    Empty,
    BoxL,
    BoxR,
    Wall,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut is_map = true;

    let mut rows = Vec::<Row<Tile>>::new();
    let mut rows2 = Vec::<Row<Tile2>>::new();
    let mut commands = Vec::<Direction>::new();
    let mut start: Option<(usize, usize)> = None;
    for (y, line) in input.lines().enumerate() {
        if is_map {
            if line.is_empty() {
                is_map = false;
                continue;
            }
            let mut tiles = Vec::<Tile>::new();
            let mut tiles2 = Vec::<Tile2>::new();
            for (x, c) in line.chars().enumerate() {
                let tile = match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'O' => Tile::Box,
                    '@' => {
                        assert!(start.is_none());
                        start = Some((x, y));
                        Tile::Empty
                    }
                    _ => panic!(),
                };
                tiles.push(tile);
                let tile2s = match tile {
                    Tile::Empty => [Tile2::Empty, Tile2::Empty],
                    Tile::Box => [Tile2::BoxL, Tile2::BoxR],
                    Tile::Wall => [Tile2::Wall, Tile2::Wall],
                };
                tiles2.extend(tile2s);
            }
            rows.push(Row { tiles });
            rows2.push(Row { tiles: tiles2 });
        } else {
            for c in line.chars() {
                commands.push(match c {
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    _ => panic!(),
                });
            }
        }
    }

    let map = Map { rows };
    let map2 = Map { rows: rows2 };

    let start = start.unwrap();

    // Part 1
    {
        let mut map = map.clone();
        let mut pos = (start.0 as isize, start.1 as isize);

        for c in commands.iter() {
            let npos = Direction::go(pos, *c);
            let can_move: bool;
            let mut cur_tile_pos = npos;
            loop {
                match map.at(cur_tile_pos).unwrap() {
                    Tile::Empty => {
                        can_move = true;
                        break;
                    }
                    Tile::Box => {}
                    Tile::Wall => {
                        can_move = false;
                        break;
                    }
                }
                cur_tile_pos = Direction::go(cur_tile_pos, *c);
            }
            if can_move {
                pos = npos;
                let at_npos = map.at(npos).unwrap();
                if at_npos == Tile::Box {
                    *map.at_mut(npos).unwrap() = Tile::Empty;
                    *map.at_mut(cur_tile_pos).unwrap() = Tile::Box;
                }
            }
        }

        let mut part1 = 0usize;
        for (y, row) in map.rows.iter().enumerate() {
            for (x, tile) in row.tiles.iter().enumerate() {
                if *tile == Tile::Box {
                    part1 += 100 * y + x;
                }
            }
        }
        output.part1(part1 as i64);
    }

    #[allow(dead_code)]
    fn print_map2(map: &Map<Tile2>, pos: (isize, isize)) {
        for (y, row) in map.rows.iter().enumerate() {
            for (x, tile) in row.tiles.iter().enumerate() {
                let mut c = match tile {
                    Tile2::Empty => '.',
                    Tile2::BoxL => '[',
                    Tile2::BoxR => ']',
                    Tile2::Wall => '#',
                };
                if (x as isize, y as isize) == pos {
                    c = '@';
                }
                print!("{c}");
            }
            println!();
        }
        println!();
    }

    // Part 2
    {
        let mut map = map2.clone();
        let mut pos = (start.0 as isize * 2, start.1 as isize);

        let mut wavefront = Vec::<(isize, isize)>::new();
        let mut next_wavefront = Vec::<(isize, isize)>::new();
        let mut moves_to_apply = Vec::<((isize, isize), Tile2)>::new();
        for c in commands.iter() {
            wavefront.clear();
            next_wavefront.clear();
            moves_to_apply.clear();

            let mut can_move = true;
            wavefront.push(pos);
            let up_or_down = [Direction::Up, Direction::Down].contains(c);
            loop {
                for wf in wavefront.drain(..) {
                    let cur_tile_pos = Direction::go(wf, *c);
                    match map.at(cur_tile_pos).unwrap() {
                        Tile2::Empty => {}
                        Tile2::BoxL => {
                            next_wavefront.push(cur_tile_pos);
                            if up_or_down {
                                next_wavefront.push((cur_tile_pos.0 + 1, cur_tile_pos.1));
                            }
                        }
                        Tile2::BoxR => {
                            next_wavefront.push(cur_tile_pos);
                            if up_or_down {
                                next_wavefront.push((cur_tile_pos.0 - 1, cur_tile_pos.1));
                            }
                        }
                        Tile2::Wall => {
                            can_move = false;
                            break;
                        }
                    }
                }
                if !can_move {
                    break;
                }
                wavefront.append(&mut next_wavefront);
                if wavefront.is_empty() {
                    break;
                }
                for wf in wavefront.iter() {
                    moves_to_apply.push((*wf, map.at(*wf).unwrap()));
                }
            }
            if !can_move {
                continue;
            }

            // Now we've confirmed we can move, apply the move.
            for (cur_tile_pos, tile) in moves_to_apply.iter().rev() {
                let npos = Direction::go(*cur_tile_pos, *c);
                *map.at_mut(npos).unwrap() = *tile;
                *map.at_mut(*cur_tile_pos).unwrap() = Tile2::Empty;
            }
            pos = Direction::go(pos, *c);
        }

        let mut part2 = 0usize;
        for (y, row) in map.rows.iter().enumerate() {
            for (x, tile) in row.tiles.iter().enumerate() {
                if *tile == Tile2::BoxL {
                    part2 += 100 * y + x;
                }
            }
        }
        output.part2(part2 as i64);
    }
}
