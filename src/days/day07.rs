use crate::day_output::DayOutput;

struct Equation {
    tv: usize,
    nums: Vec<usize>,
}

enum Operator {
    Add,
    Mul,
    Concat,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut equations = Vec::<Equation>::new();
    for line in input.lines() {
        let (tv, nums) = line.split_once(':').unwrap();
        let parse = |v: &str| v.parse::<usize>().unwrap();
        equations.push(Equation {
            tv: parse(tv),
            nums: nums.split_ascii_whitespace().map(parse).collect(),
        });
    }
    let equations = equations;

    fn equation_is_true<const N: usize>(eq: &Equation, operators: &[Operator; N]) -> bool {
        let tv = eq.tv;
        let nums = &eq.nums;
        let accum = nums[0];
        let rest = &nums[1..];

        fn recurse<const N: usize>(
            tv: usize,
            operators: &[Operator; N],
            accum: usize,
            rest: &[usize],
        ) -> bool {
            if accum > tv {
                return false;
            }
            if rest.is_empty() {
                return accum == tv;
            }
            let operand = rest[0];
            let rest = &rest[1..];
            for op in operators {
                let new_accum = match op {
                    Operator::Add => accum + operand,
                    Operator::Mul => accum * operand,
                    Operator::Concat => {
                        let mut cur = 10usize;
                        while operand >= cur {
                            cur *= 10;
                        }
                        accum * cur + operand
                    }
                };
                if recurse(tv, operators, new_accum, rest) {
                    return true;
                }
            }
            false
        }

        recurse(tv, operators, accum, rest)
    }

    let part1 = equations
        .iter()
        .filter_map(
            |eq| match equation_is_true(eq, &[Operator::Add, Operator::Mul]) {
                true => Some(eq.tv),
                false => None,
            },
        )
        .sum::<usize>();
    output.part1(part1.to_string());

    let part2 = equations
        .iter()
        .filter_map(|eq| {
            match equation_is_true(eq, &[Operator::Add, Operator::Mul, Operator::Concat]) {
                true => Some(eq.tv),
                false => None,
            }
        })
        .sum::<usize>();
    output.part2(part2.to_string());
}
