use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::day_output::DayOutput;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ComputerName(Rc<String>);
struct Link(ComputerName, ComputerName);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct ComputerId(u16);

struct Computer {
    id: ComputerId,
    name: ComputerName,
    conns: HashSet<ComputerId>,
}

pub fn main(input: &str, output: &mut DayOutput) {
    let mut links = Vec::<Link>::new();
    for line in input.lines() {
        let mut iter_computers = line.split("-");
        let first = iter_computers.next().unwrap();
        let second = iter_computers.next().unwrap();
        assert!(iter_computers.next().is_none());
        links.push(Link(
            ComputerName(Rc::new(first.to_owned())),
            ComputerName(Rc::new(second.to_owned())),
        ));
    }
    let links = links;

    let mut direct_conns = HashMap::<ComputerName, HashSet<ComputerName>>::new();
    let mut add_direct_conn = |a: ComputerName, b: ComputerName| {
        direct_conns.entry(a).or_default().insert(b);
    };
    for link in links.iter() {
        add_direct_conn(link.0.clone(), link.1.clone());
        add_direct_conn(link.1.clone(), link.0.clone());
    }

    let mut all_computers = direct_conns.iter().collect::<Vec<_>>();
    all_computers.sort_by(|a, b| a.0 .0.cmp(&b.0 .0));
    let computer_name_to_id = all_computers
        .iter()
        .enumerate()
        .map(|(i, c)| (c.0, ComputerId(i as u16)))
        .collect::<HashMap<_, _>>();
    let computers = all_computers
        .iter()
        .enumerate()
        .map(|(idx, (name, conns))| Computer {
            id: ComputerId(idx as u16),
            name: (*name).clone(),
            conns: conns
                .iter()
                .map(|c| *computer_name_to_id.get(c).unwrap())
                .collect::<HashSet<_>>(),
        })
        .collect::<Vec<_>>();

    // Part 1
    {
        let mut triples = HashSet::<[ComputerId; 3]>::new();
        for c in computers.iter() {
            for a in c.conns.iter() {
                for b in c.conns.iter() {
                    if a == b {
                        continue;
                    }
                    if computers[a.0 as usize].conns.contains(b) {
                        let mut triple = [*a, *b, c.id];
                        triple.sort();
                        triples.insert(triple);
                    }
                }
            }
        }

        let mut part1 = 0usize;
        for triple in triples.iter() {
            let mut any_starts_with_t = false;
            for t in triple {
                if computers[t.0 as usize].name.0.starts_with("t") {
                    any_starts_with_t = true;
                    break;
                }
            }
            if any_starts_with_t {
                part1 += 1;
            }
        }
        output.part1(part1.to_string());
    }

    // Part 2
    {
        fn add_n_tuple(n_tuples: &mut HashSet<Vec<ComputerId>>, n_tuple: &[ComputerId]) -> bool {
            let mut n_tuple = n_tuple.to_owned();
            n_tuple.sort();
            n_tuples.insert(n_tuple)
        }
        fn recurse(
            computers: &Vec<Computer>,
            n_tuples: &mut HashSet<Vec<ComputerId>>,
            n_tuple: &[ComputerId],
            v: &HashSet<ComputerId>,
        ) {
            for next_c in v {
                let mut n_tuple = n_tuple.to_owned();
                n_tuple.push(*next_c);
                let added = add_n_tuple(n_tuples, &n_tuple);
                if added {
                    let next_v = v
                        .intersection(&computers[next_c.0 as usize].conns)
                        .copied()
                        .collect::<HashSet<_>>();
                    recurse(computers, n_tuples, &n_tuple, &next_v);
                }
            }
        }

        let all_computers_set = computers.iter().map(|c| c.id).collect::<HashSet<_>>();
        let mut n_tuples = HashSet::<Vec<ComputerId>>::new();
        recurse(&computers, &mut n_tuples, &[], &all_computers_set);

        let mut n_tuples = n_tuples.iter().collect::<Vec<_>>();
        n_tuples.sort_by_key(|t| t.len());
        let last = n_tuples
            .last()
            .unwrap()
            .iter()
            .map(|s| computers[s.0 as usize].name.0.to_string())
            .collect::<Vec<_>>();
        output.part2(last.join(","));
    }
}
