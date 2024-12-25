use crate::day_output::DayOutput;

#[derive(Debug, Clone)]
struct Key {
    heights: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Lock {
    heights: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Elem {
    Empty,
    Metal,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut keys = Vec::<Key>::new();
    let mut locks = Vec::<Lock>::new();

    let mut add = |buf: &[Vec<Elem>]| {
        if buf.first().unwrap().iter().all(|e| *e == Elem::Empty) {
            // Key
            let mut heights = Vec::<usize>::new();
            for x in 0..buf.first().unwrap().len() {
                let cur_height = buf
                    .iter()
                    .take(buf.len() - 1)
                    .filter(|r| r[x] == Elem::Metal)
                    .count();
                heights.push(cur_height);
            }
            keys.push(Key { heights });
        } else {
            assert!(buf.first().unwrap().iter().all(|e| *e == Elem::Metal));
            // Lock
            let mut heights = Vec::<usize>::new();
            for x in 0..buf.last().unwrap().len() {
                let cur_height = buf.iter().skip(1).filter(|r| r[x] == Elem::Metal).count();
                heights.push(cur_height);
            }
            locks.push(Lock { heights });
        }
    };

    let mut buf = Vec::<Vec<Elem>>::new();

    for line in input.lines() {
        if line.is_empty() {
            add(&buf);
            buf.clear();
        } else {
            let mut rows = Vec::<Elem>::new();
            for char in line.chars() {
                let e = match char {
                    '.' => Elem::Empty,
                    '#' => Elem::Metal,
                    _ => panic!(),
                };
                rows.push(e);
            }
            buf.push(rows);
        }
    }
    if !buf.is_empty() {
        add(&buf);
    }

    let mut part1 = 0usize;
    for k in keys.iter() {
        for l in locks.iter() {
            let mut any_dont_fit = false;
            for (lh, kh) in l.heights.iter().zip(k.heights.iter()) {
                if *lh + *kh > 5 {
                    any_dont_fit = true;
                    break;
                }
            }
            if !any_dont_fit {
                part1 += 1;
            }
        }
    }
    output.part1(part1.to_string());
}
