use std::collections::HashMap;

use regex::Regex;

use crate::day_output::DayOutput;

#[derive(Debug, Copy, Clone)]
struct Robot {
    p: (isize, isize),
    v: (isize, isize),
}

#[derive(Debug, Copy, Clone)]
struct Quadrant {
    x: (isize, isize),
    y: (isize, isize),
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut iter_lines = input.lines();
    let mut sz_ln = iter_lines.next().unwrap().split_whitespace();
    let sz_x = sz_ln.next().unwrap().parse::<isize>().unwrap();
    let sz_y = sz_ln.next().unwrap().parse::<isize>().unwrap();
    assert!(sz_ln.next().is_none());

    let robot_re = Regex::new(r"p=(.+),(.+) v=(.+),(.+)").unwrap();

    let mut robots = Vec::<Robot>::new();
    for line in iter_lines {
        let c = robot_re.captures(line).unwrap();
        let parse = |i: usize| c.get(i).unwrap().as_str().parse::<isize>().unwrap();
        robots.push(Robot {
            p: (parse(1), parse(2)),
            v: (parse(3), parse(4)),
        });
    }

    let sz = (sz_x, sz_y);
    let robots = robots;

    let mut p1_robots = robots.clone();

    fn simulate(sz: (isize, isize), iters: usize, robots: &mut [Robot]) {
        let szx = sz.0;
        let szy = sz.1;
        for r in robots.iter_mut() {
            let px = &mut r.p.0;
            let py = &mut r.p.1;
            let vx = r.v.0;
            let vy = r.v.1;
            *px += vx * iters as isize;
            *py += vy * iters as isize;

            *px = px.rem_euclid(szx);
            *py = py.rem_euclid(szy);
        }
    }

    simulate(sz, 100, &mut p1_robots);

    let qszx = sz_x / 2;
    let qszy = sz_y / 2;
    let quadrants = [
        Quadrant {
            x: (0, qszx),
            y: (0, qszy),
        },
        Quadrant {
            x: (qszx + 1, qszx * 2 + 1),
            y: (0, qszy),
        },
        Quadrant {
            x: (0, qszx),
            y: (qszy + 1, qszy * 2 + 1),
        },
        Quadrant {
            x: (qszx + 1, qszx * 2 + 1),
            y: (qszy + 1, qszy * 2 + 1),
        },
    ];

    let mut part1 = 1usize;
    for q in quadrants.iter() {
        let mut r_count = 0usize;
        for r in p1_robots.iter() {
            let px = r.p.0;
            let py = r.p.1;
            if q.x.0 <= px && q.x.1 > px && q.y.0 <= py && q.y.1 > py {
                r_count += 1;
            }
        }
        part1 *= r_count;
    }
    output.part1(part1 as i64);

    let mut p2_robots = robots.clone();

    let mut locs = HashMap::<(isize, isize), usize>::new();
    let mut iter_count = 0;
    let iter_jump = 1usize;
    loop {
        locs.clear();
        let mut any_non_unique = false;
        for r in p2_robots.iter() {
            let e = locs.entry(r.p).or_default();
            *e += 1;
            if *e > 1 {
                any_non_unique = true;
            }
        }
        if !any_non_unique {
            for y in 0..sz_y {
                for x in 0..sz_x {
                    match locs.get(&(x, y)) {
                        Some(c) => print!("{c}"),
                        None => print!("."),
                    }
                }
                println!();
            }
            println!("{iter_count}");
            break;
        }
        simulate(sz, iter_jump, &mut p2_robots);
        iter_count += iter_jump;
    }
    output.part2(iter_count as i64);
}
