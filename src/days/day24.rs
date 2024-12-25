use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::day_output::DayOutput;

#[derive(Debug, Clone)]
struct Input {
    name: Rc<String>,
    val: bool,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    And,
    Xor,
    Or,
}

impl Op {
    fn apply(self, i0: bool, i1: bool) -> bool {
        match self {
            Op::And => i0 && i1,
            Op::Xor => i0 ^ i1,
            Op::Or => i0 | i1,
        }
    }
}

#[derive(Debug, Clone)]
struct Gate {
    in0: Rc<String>,
    in1: Rc<String>,
    op: Op,
    out: Rc<String>,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut inputs = Vec::<Input>::new();
    let mut gates = Vec::<Gate>::new();
    let mut is_input = true;
    for line in input.lines() {
        if is_input {
            if line.is_empty() {
                is_input = false;
            } else {
                let spl = line.split_once(": ").unwrap();
                inputs.push(Input {
                    name: Rc::new(spl.0.to_owned()),
                    val: match spl.1 {
                        "0" => false,
                        "1" => true,
                        _ => panic!(),
                    },
                });
            }
        } else {
            let mut iter_token = line.split_ascii_whitespace();
            let in0 = Rc::new(iter_token.next().unwrap().to_owned());
            let op = match iter_token.next().unwrap() {
                "AND" => Op::And,
                "XOR" => Op::Xor,
                "OR" => Op::Or,
                _ => panic!(),
            };
            let in1 = Rc::new(iter_token.next().unwrap().to_owned());
            iter_token.next().unwrap();
            let out = Rc::new(iter_token.next().unwrap().to_owned());
            assert!(iter_token.next().is_none());
            gates.push(Gate { in0, in1, op, out });
        }
    }
    let inputs = inputs;
    let gates = gates;

    fn bool_to_bit(b: bool, bit: usize) -> usize {
        (match b {
            true => 1,
            false => 0,
        }) << bit
    }

    let make_inputs_to_val = |overrides: &[Input]| -> HashMap<Rc<String>, bool> {
        let mut input_to_val = HashMap::<Rc<String>, bool>::new();
        for input in inputs.iter() {
            input_to_val.insert(input.name.clone(), false);
        }
        for o in overrides {
            input_to_val.insert(o.name.clone(), o.val);
        }
        input_to_val
    };

    fn replace_gate(g: &Gate, gate_output_swaps: &[(Rc<String>, Rc<String>)]) -> Gate {
        let replace = |s: &Rc<String>| -> Rc<String> {
            for swap in gate_output_swaps {
                if swap.0 == *s {
                    return swap.1.clone();
                } else if swap.1 == *s {
                    return swap.0.clone();
                }
            }
            s.clone()
        };
        Gate {
            in0: g.in0.clone(),
            in1: g.in1.clone(),
            op: g.op,
            out: replace(&g.out),
        }
    }

    fn simulate(
        gates: &[Gate],
        gate_output_swaps: &[(Rc<String>, Rc<String>)],
        input_to_val: &mut HashMap<Rc<String>, bool>,
    ) -> usize {
        let mut uninit_gates = gates
            .iter()
            .map(|g| (replace_gate(g, gate_output_swaps), false))
            .collect::<Vec<_>>();

        loop {
            let mut processed_any = false;
            for (g, processed) in uninit_gates.iter_mut() {
                if *processed {
                    continue;
                }
                if let Some(&i0) = input_to_val.get(&g.in0) {
                    if let Some(&i1) = input_to_val.get(&g.in1) {
                        *processed = true;
                        processed_any = true;
                        let val = g.op.apply(i0, i1);
                        input_to_val.insert(g.out.clone(), val);
                    }
                }
            }
            if !processed_any {
                break;
            }
        }

        let mut z = 0usize;

        for (i, v) in input_to_val {
            if let Some(stripped) = i.strip_prefix("z") {
                let idx = stripped.parse::<usize>().unwrap();
                z |= bool_to_bit(*v, idx);
            }
        }
        z
    }

    // Part 1
    {
        let part1 = simulate(&gates, &[], &mut make_inputs_to_val(&inputs));
        output.part1(part1.to_string());
    }

    // Part 2
    {
        let input_gate_names = inputs
            .iter()
            .map(|g| g.name.clone())
            .collect::<HashSet<_>>();

        let other_gate_names = gates.iter().map(|g| g.out.clone()).collect::<HashSet<_>>();
        let mut other_gate_names = other_gate_names.iter().collect::<Vec<_>>();
        other_gate_names.sort();

        let mut fixing_swaps = HashSet::<(Rc<String>, Rc<String>)>::new();

        const MAX_TEST_BITS: usize = 3;

        for i in 0..64usize {
            let mut overrides = Vec::<Input>::new();
            for b in 0..MAX_TEST_BITS {
                overrides.push(Input {
                    name: Rc::new(format!("x{:02}", i + b)),
                    val: false,
                });
                overrides.push(Input {
                    name: Rc::new(format!("y{:02}", i + b)),
                    val: false,
                });
            }

            if overrides
                .iter()
                .any(|o| !input_gate_names.contains(&o.name))
            {
                break;
            }

            let mut run_tests = |test_bits: usize,
                                 gate_output_swaps: &[(Rc<String>, Rc<String>)]|
             -> (bool, HashSet<Rc<String>>) {
                assert!(test_bits <= MAX_TEST_BITS);
                let mut is_broken = false;
                let mut stimulated_inputs = HashSet::<Rc<String>>::new();
                for x in (0..(1 << test_bits)).rev() {
                    for y in (0..(1 << test_bits)).rev() {
                        for i in 0usize..test_bits {
                            overrides[2 * i].val = x & 1 << i != 0;
                            overrides[2 * i + 1].val = y & 1 << i != 0;
                        }

                        let z = x + y;
                        let mut inputs_to_val = make_inputs_to_val(&overrides[..2 * test_bits]);
                        let simulated = simulate(&gates, gate_output_swaps, &mut inputs_to_val);
                        let expected = z << i;
                        if simulated != expected {
                            for (name, v) in inputs_to_val.iter() {
                                if *v {
                                    stimulated_inputs.insert(name.clone());
                                }
                            }
                            is_broken = true;
                            break;
                        }
                    }
                    if is_broken {
                        break;
                    }
                }
                (is_broken, stimulated_inputs)
            };

            let (is_broken, stimulated) = run_tests(
                MAX_TEST_BITS,
                &fixing_swaps.iter().cloned().collect::<Vec<_>>(),
            );
            if is_broken {
                let mut stimulated = stimulated
                    .iter()
                    .filter(|s| !input_gate_names.contains(*s))
                    .collect::<Vec<_>>();
                stimulated.sort();
                let mut swaps: Vec<(Rc<String>, Rc<String>)> =
                    Vec::<(Rc<String>, Rc<String>)>::new();
                let mut possible_fixes = HashSet::<(Rc<String>, Rc<String>)>::new();
                for &x in stimulated.iter() {
                    for &y in other_gate_names.iter() {
                        if x == y {
                            continue;
                        }
                        swaps.clear();
                        for swap in fixing_swaps.iter() {
                            swaps.push(swap.clone());
                        }
                        swaps.push((x.clone(), y.clone()));
                        let (is_still_broken, _) = run_tests(MAX_TEST_BITS, &swaps);
                        if !is_still_broken {
                            possible_fixes.insert((x.clone(), y.clone()));
                        }
                    }
                }
                if possible_fixes.is_empty() {
                    println!("No fix found from bit {i}");
                    break;
                } else if possible_fixes.len() == 1 {
                    fixing_swaps.insert(possible_fixes.iter().next().unwrap().clone());
                }
            }
        }
        println!("Swaps: {fixing_swaps:?}");

        let mut part2 = Vec::<String>::new();
        for f in fixing_swaps.iter() {
            part2.push(f.0.to_string());
            part2.push(f.1.to_string());
        }
        part2.sort();
        output.part2(part2.join(","));
    }
}
