use crate::day_output::DayOutput;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

impl Instruction {
    fn parse(i: u8) -> Instruction {
        match i {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!(),
        }
    }
}

fn exec(instr: u8, op: u8, regs: &mut Registers, pc: &mut usize, mut out: impl FnMut(u8)) {
    let instr = Instruction::parse(instr);
    let combo = || -> i64 {
        match op {
            0..=3 => op as i64,
            4 => regs.a,
            5 => regs.b,
            6 => regs.c,
            7 => panic!(),
            _ => panic!(),
        }
    };
    let literal = || -> i64 { op as i64 };
    match instr {
        Instruction::Adv => {
            let num = regs.a;
            let denom = 2i64.pow(combo().try_into().unwrap());
            regs.a = num / denom;
            *pc += 2;
        }
        Instruction::Bxl => {
            regs.b ^= literal();
            *pc += 2;
        }
        Instruction::Bst => {
            regs.b = combo() % 8;
            *pc += 2;
        }
        Instruction::Jnz => {
            if regs.a != 0 {
                *pc = literal().try_into().unwrap();
            } else {
                *pc += 2;
            }
        }
        Instruction::Bxc => {
            regs.b ^= regs.c;
            *pc += 2;
        }
        Instruction::Out => {
            out((combo() % 8).try_into().unwrap());
            *pc += 2;
        }
        Instruction::Bdv => {
            let num = regs.a;
            let denom = 2i64.pow(combo().try_into().unwrap());
            regs.b = num / denom;
            *pc += 2;
        }
        Instruction::Cdv => {
            let num = regs.a;
            let denom = 2i64.pow(combo().try_into().unwrap());
            regs.c = num / denom;
            *pc += 2;
        }
    }
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut lines_iter = input.lines();
    let reg_a = lines_iter.next().unwrap();
    let reg_b = lines_iter.next().unwrap();
    let reg_c = lines_iter.next().unwrap();
    lines_iter.next();
    let program = lines_iter.next().unwrap();
    assert!(lines_iter.next().is_none());

    let initial_regs = Registers {
        a: reg_a[reg_a.find(':').unwrap() + 2..].parse().unwrap(),
        b: reg_b[reg_b.find(':').unwrap() + 2..].parse().unwrap(),
        c: reg_c[reg_c.find(':').unwrap() + 2..].parse().unwrap(),
    };
    let program = program[program.find(':').unwrap() + 2..]
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    // Part 1
    {
        let mut registers = initial_regs;
        let mut pc = 0usize;
        let mut outs = Vec::<u8>::new();
        loop {
            let instr = program.get(pc);
            let operand = program.get(pc + 1);
            let mut out = |v: u8| outs.push(v);
            match (instr, operand) {
                (Some(instr), Some(operand)) => {
                    exec(*instr, *operand, &mut registers, &mut pc, &mut out)
                }
                _ => break,
            }
        }

        let part1 = outs
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");

        output.part1(part1);
    }

    // Part 2.
    // I couldn't figure out a generic solution that didn't rely on me coding up something
    // input-specific, so here's the brute force solution that will complete eventually.
    {
        let mut outs = Vec::<u8>::new();
        for reg_a in 0.. {
            let mut registers = Registers {
                a: reg_a,
                b: 0,
                c: 0,
            };
            outs.clear();
            let mut pc = 0usize;
            let mut out = |v: u8| outs.push(v);
            let mut end_of_program = false;
            let mut success = false;
            loop {
                let instr = program.get(pc);
                let operand = program.get(pc + 1);
                match (instr, operand) {
                    (Some(instr), Some(operand)) => {
                        exec(*instr, *operand, &mut registers, &mut pc, &mut out)
                    }
                    _ => end_of_program = true,
                }
                if end_of_program {
                    if outs == program {
                        success = true;
                    }
                    break;
                }
            }
            if success {
                output.part2(reg_a.to_string());
                break;
            }
        }
    }
}
