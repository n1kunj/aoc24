use std::collections::{hash_map::Entry, HashMap};

use crate::{
    day_output::DayOutput,
    direction::{Direction, DIRECTIONS},
    map::{Map, Row},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumPad {
    _A,
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
}

#[derive(Debug, Clone)]
struct Input {
    cost: usize,
    nums: Vec<NumPad>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirPad {
    Up,
    Right,
    Down,
    Left,
    A,
}

impl DirPad {
    fn pos(self) -> (isize, isize) {
        match self {
            DirPad::Up => (1, 0),
            DirPad::Right => (2, 1),
            DirPad::Down => (1, 1),
            DirPad::Left => (0, 1),
            DirPad::A => (2, 0),
        }
    }

    fn at((x, y): (isize, isize)) -> Option<DirPad> {
        match (x, y) {
            (1, 0) => Some(DirPad::Up),
            (2, 1) => Some(DirPad::Right),
            (1, 1) => Some(DirPad::Down),
            (0, 1) => Some(DirPad::Left),
            (2, 0) => Some(DirPad::A),
            _ => None,
        }
    }

    fn convert(d: Direction) -> DirPad {
        match d {
            Direction::Up => DirPad::Up,
            Direction::Right => DirPad::Right,
            Direction::Down => DirPad::Down,
            Direction::Left => DirPad::Left,
        }
    }

    fn to_char(self) -> char {
        match self {
            DirPad::Up => '^',
            DirPad::Right => '>',
            DirPad::Down => 'v',
            DirPad::Left => '<',
            DirPad::A => 'A',
        }
    }
}

#[allow(dead_code)]
fn print_dirs(dirs: &[DirPad]) {
    for d in dirs {
        print!("{}", d.to_char());
    }
    println!();
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut inputs = Vec::<Input>::new();
    for line in input.lines() {
        let mut nums = Vec::<NumPad>::new();
        for char in line.chars() {
            let num = match char {
                'A' => NumPad::_A,
                '0' => NumPad::_0,
                '1' => NumPad::_1,
                '2' => NumPad::_2,
                '3' => NumPad::_3,
                '4' => NumPad::_4,
                '5' => NumPad::_5,
                '6' => NumPad::_6,
                '7' => NumPad::_7,
                '8' => NumPad::_8,
                '9' => NumPad::_9,
                _ => panic!(),
            };
            nums.push(num);
        }
        let cost = line.strip_suffix('A').unwrap().parse::<usize>().unwrap();
        inputs.push(Input { cost, nums });
    }
    let inputs = inputs;

    let rows = vec![
        Row {
            tiles: vec![Some(NumPad::_7), Some(NumPad::_8), Some(NumPad::_9)],
        },
        Row {
            tiles: vec![Some(NumPad::_4), Some(NumPad::_5), Some(NumPad::_6)],
        },
        Row {
            tiles: vec![Some(NumPad::_1), Some(NumPad::_2), Some(NumPad::_3)],
        },
        Row {
            tiles: vec![None, Some(NumPad::_0), Some(NumPad::_A)],
        },
    ];
    let numpad = Map { rows };

    fn dirs_to_pad_inputs(dirs: &[Direction]) -> Vec<DirPad> {
        let mut dirpath = dirs.iter().map(|d| DirPad::convert(*d)).collect::<Vec<_>>();
        dirpath.push(DirPad::A);
        dirpath
    }

    fn calc_best_dir_input_path(
        memo: &mut HashMap<(DirPad, DirPad, usize), usize>,
        src: DirPad,
        dst: DirPad,
        iters: usize,
    ) -> Vec<Direction> {
        if src == dst {
            return vec![];
        }

        let mut heads = Vec::<((isize, isize), Vec<Direction>)>::new();
        let mut visited = HashMap::<(isize, isize), Vec<Direction>>::new();
        heads.push((src.pos(), vec![]));
        visited.insert(heads[0].0, heads[0].1.clone());
        let mut next_heads = Vec::<((isize, isize), Vec<Direction>)>::new();
        while !heads.is_empty() {
            for (pos, path) in heads.drain(..) {
                for d in DIRECTIONS {
                    let npos = Direction::go(pos, d);
                    if DirPad::at(npos).is_some() {
                        let entry = visited.entry(npos);
                        let mut npath = path.clone();
                        npath.push(d);
                        let should_add = match &entry {
                            Entry::Occupied(e) => {
                                let mut cost = |ds: &Vec<Direction>| -> usize {
                                    let pad_inputs = dirs_to_pad_inputs(ds);
                                    calc_dir_inputs_cost(memo, &pad_inputs, iters)
                                };
                                cost(&npath) <= cost(e.get())
                            }
                            Entry::Vacant(_) => true,
                        };
                        if should_add {
                            entry.insert_entry(npath.clone());
                            next_heads.push((npos, npath));
                        }
                    }
                }
            }
            heads.append(&mut next_heads);
        }
        visited.remove(&DirPad::pos(dst)).unwrap()
    }

    fn calc_dir_input_cost(
        memo: &mut HashMap<(DirPad, DirPad, usize), usize>,
        src: DirPad,
        dst: DirPad,
        iters: usize,
    ) -> usize {
        if iters == 0 {
            return 1;
        }
        if let Some(memoized) = memo.get(&(src, dst, iters)) {
            return *memoized;
        }
        let mut cost = 0usize;
        let mut pos = DirPad::A;
        for d in calc_best_dir_input_path(memo, src, dst, iters - 1) {
            let npos = DirPad::convert(d);
            cost += calc_dir_input_cost(memo, pos, npos, iters - 1);
            pos = npos;
        }
        cost += calc_dir_input_cost(memo, pos, DirPad::A, iters - 1);
        memo.insert((src, dst, iters), cost);
        cost
    }

    fn calc_dir_inputs_cost(
        memo: &mut HashMap<(DirPad, DirPad, usize), usize>,
        ins: &[DirPad],
        iters: usize,
    ) -> usize {
        let mut cost = 0usize;
        let mut pos = DirPad::A;
        for dst in ins {
            cost += calc_dir_input_cost(memo, pos, *dst, iters);
            pos = *dst;
        }
        cost
    }

    fn calc_best_numpad_paths(
        numpad: &Map<Option<NumPad>>,
        iters: usize,
    ) -> HashMap<(NumPad, NumPad), Vec<Direction>> {
        let mut memo = HashMap::<(DirPad, DirPad, usize), usize>::new();
        let mut best_numpad_paths = HashMap::<(NumPad, NumPad), Vec<Direction>>::new();
        for (y, row) in numpad.rows.iter().enumerate() {
            for (x, num) in row.tiles.iter().enumerate() {
                let num = match num {
                    Some(num) => *num,
                    None => continue,
                };
                let mut heads = Vec::<((isize, isize), Vec<Direction>)>::new();
                let mut visited = HashMap::<(isize, isize), Vec<Direction>>::new();
                heads.push(((x as isize, y as isize), vec![]));
                visited.insert(heads[0].0, heads[0].1.clone());
                let mut next_heads = Vec::<((isize, isize), Vec<Direction>)>::new();
                while !heads.is_empty() {
                    for (pos, path) in heads.drain(..) {
                        for d in DIRECTIONS {
                            let npos = Direction::go(pos, d);
                            if numpad.at(npos).flatten().is_none() {
                                continue;
                            }
                            let entry = visited.entry(npos);
                            let mut npath = path.clone();
                            npath.push(d);
                            let should_add = match &entry {
                                Entry::Occupied(e) => {
                                    let mut cost = |ds: &Vec<Direction>| -> usize {
                                        let pad_inputs = dirs_to_pad_inputs(ds);
                                        calc_dir_inputs_cost(&mut memo, &pad_inputs, iters)
                                    };
                                    cost(&npath) <= cost(e.get())
                                }
                                Entry::Vacant(_) => true,
                            };
                            if should_add {
                                entry.insert_entry(npath.clone());
                                next_heads.push((npos, npath));
                            }
                        }
                    }
                    heads.append(&mut next_heads);
                }
                for (pos, path) in visited {
                    let src = num;
                    let dst = numpad.at(pos).unwrap().unwrap();
                    best_numpad_paths.insert((src, dst), path);
                }
            }
        }
        assert_eq!(best_numpad_paths.len(), 11 * 11);
        best_numpad_paths
    }

    let calc_complexity = |iters: usize| -> usize {
        let mut cost_memo = HashMap::<(DirPad, DirPad, usize), usize>::new();
        let best_numpad_paths = calc_best_numpad_paths(&numpad, iters);
        let mut cost = 0usize;
        for input in inputs.iter() {
            let mut numrobpos = NumPad::_A;
            let mut numrobpath = Vec::<DirPad>::new();
            for n in input.nums.iter() {
                let numpath = best_numpad_paths.get(&(numrobpos, *n)).unwrap();
                for d in numpath {
                    numrobpath.push(DirPad::convert(*d));
                }
                numrobpath.push(DirPad::A);
                numrobpos = *n;
            }

            cost += calc_dir_inputs_cost(&mut cost_memo, &numrobpath, iters) * input.cost;
        }
        cost
    };

    let part1 = calc_complexity(2);
    output.part1(part1.to_string());

    let part2 = calc_complexity(25);
    output.part2(part2.to_string());
}
