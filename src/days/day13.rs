use regex::{Captures, Regex};

use crate::day_output::DayOutput;

#[derive(Debug)]
struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    p: (isize, isize),
}

pub fn main(input: &str, output: &mut DayOutput) {
    let a_re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let b_re = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let p_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut machines = Vec::<Machine>::new();
    let mut line_iter = input.lines().peekable();
    while line_iter.peek().is_some() {
        let a = line_iter.next().unwrap();
        let b = line_iter.next().unwrap();
        let p = line_iter.next().unwrap();

        let a = a_re.captures(a).unwrap();
        let b = b_re.captures(b).unwrap();
        let p = p_re.captures(p).unwrap();

        let parse = |c: &Captures, i: usize| c.get(i).unwrap().as_str().parse::<isize>().unwrap();

        machines.push(Machine {
            a: (parse(&a, 1), parse(&a, 2)),
            b: (parse(&b, 1), parse(&b, 2)),
            p: (parse(&p, 1), parse(&p, 2)),
        });

        line_iter.next();
    }
    let machines = machines;
    let machines2 = machines
        .iter()
        .map(|m| Machine {
            a: m.a,
            b: m.b,
            p: (m.p.0 + 10000000000000, m.p.1 + 10000000000000),
        })
        .collect::<Vec<_>>();

    fn calc_cost(m: &Machine) -> Option<isize> {
        let (ax, ay) = m.a;
        let (bx, by) = m.b;
        let (px, py) = m.p;

        // n * ax + m * bx = px
        // n * ay + m * by = py

        // Factor out n
        // n * ax * ay + m * bx * ay = px * ay
        // n * ay * ax + m * by * ax = py * ax
        // n * ax * ay = px * ay - m * bx * ay
        // n * ay * ax = py * ax - m * by * ax
        // px * ay - m * bx * ay = py * ax - m * by * ax
        // m * by * ax - m * bx * ay = py * ax - px * ay

        // m = (by * ax - bx * ay) = py * ax - px * ay
        // m = (py * ax - px * ay) / (by * ax - bx * ay)

        let m_num = py * ax - px * ay;
        let m_denom = by * ax - bx * ay;
        if m_num % m_denom != 0 {
            return None;
        }
        let m = m_num / m_denom;

        let n_num = px - m * bx;
        let n_denom = ax;
        if n_num % n_denom != 0 {
            return None;
        }
        let n = n_num / n_denom;

        fn cost((n, m): &(isize, isize)) -> isize {
            3 * n + m
        }
        Some(cost(&(n, m)))
    }

    let part1 = machines.iter().filter_map(calc_cost).sum::<isize>();
    output.part1(part1.to_string());

    let part2 = machines2.iter().filter_map(calc_cost).sum::<isize>();
    output.part2(part2.to_string());
}
