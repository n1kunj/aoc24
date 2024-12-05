use crate::day_output::DayOutput;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Letter {
    X,
    M,
    A,
    S,
}

struct Row {
    letters: Vec<Letter>,
}

struct WordSearch {
    rows: Vec<Row>,
}

impl WordSearch {
    fn at(&self, (x, y): (isize, isize)) -> Option<Letter> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        let row = self.rows.get(y)?;
        row.letters.get(x).copied()
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn go((x, y): (isize, isize), d: Direction) -> (isize, isize) {
        match d {
            Direction::N => (x, y - 1),
            Direction::NE => (x + 1, y - 1),
            Direction::E => (x + 1, y),
            Direction::SE => (x + 1, y + 1),
            Direction::S => (x, y + 1),
            Direction::SW => (x - 1, y + 1),
            Direction::W => (x - 1, y),
            Direction::NW => (x - 1, y - 1),
        }
    }
}

const DIRS: &[Direction] = &[
    Direction::N,
    Direction::NE,
    Direction::E,
    Direction::SE,
    Direction::S,
    Direction::SW,
    Direction::W,
    Direction::NW,
];

pub fn main(input: &str, output: &mut DayOutput) {
    let mut rows = Vec::<Row>::new();
    for row in input.lines() {
        let mut letters = Vec::<Letter>::new();
        for c in row.chars() {
            let l = match c {
                'X' => Letter::X,
                'M' => Letter::M,
                'A' => Letter::A,
                'S' => Letter::S,
                _ => panic!(),
            };
            letters.push(l);
        }
        rows.push(Row { letters });
    }
    let ws = WordSearch { rows };

    let mut part1 = 0i64;
    for (y, row) in ws.rows.iter().enumerate() {
        for (x, letter) in row.letters.iter().enumerate() {
            if *letter != Letter::X {
                continue;
            }
            for d in DIRS.iter() {
                let mut pos = (x as isize, y as isize);
                let mut matched = true;
                for expected in [Letter::M, Letter::A, Letter::S] {
                    pos = Direction::go(pos, *d);
                    if Some(expected) == ws.at(pos) {
                        continue;
                    }
                    matched = false;
                    break;
                }
                if matched {
                    part1 += 1;
                }
            }
        }
    }
    output.part1(part1);

    let mut part2 = 0i64;
    for (y, row) in ws.rows.iter().enumerate() {
        for (x, letter) in row.letters.iter().enumerate() {
            if *letter != Letter::A {
                continue;
            }
            let pos = (x as isize, y as isize);
            let nesw = [
                ws.at(Direction::go(pos, Direction::NE)),
                ws.at(Direction::go(pos, Direction::SW)),
            ];
            let nwse = [
                ws.at(Direction::go(pos, Direction::NW)),
                ws.at(Direction::go(pos, Direction::SE)),
            ];
            fn is_mas(letters: &[Option<Letter>; 2]) -> bool {
                letters.contains(&Some(Letter::M)) && letters.contains(&Some(Letter::S))
            }
            if is_mas(&nesw) && is_mas(&nwse) {
                part2 += 1;
            }
        }
    }
    output.part2(part2)
}
