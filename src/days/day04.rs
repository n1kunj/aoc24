use crate::{
    day_output::DayOutput,
    facing::{Facing, FACINGS},
};

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
            for d in FACINGS.iter() {
                let mut pos = (x as isize, y as isize);
                let mut matched = true;
                for expected in [Letter::M, Letter::A, Letter::S] {
                    pos = Facing::go(pos, *d);
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
                ws.at(Facing::go(pos, Facing::NE)),
                ws.at(Facing::go(pos, Facing::SW)),
            ];
            let nwse = [
                ws.at(Facing::go(pos, Facing::NW)),
                ws.at(Facing::go(pos, Facing::SE)),
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
