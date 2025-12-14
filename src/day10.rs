use std::{collections::HashSet, iter};

use smallvec::SmallVec;

use crate::graph::min_distance_to;
#[allow(unused)]
use crate::prelude::*;

type JoltageVec = SmallVec<[u16; 10]>;

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: JoltageVec,
}

fn parse_line(line: &str) -> Machine {
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

    let mut pieces = line.split(' ');
    let lights = pieces
        .next()
        .unwrap()
        .trim_matches(&['[', ']'])
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("unexpected char {c}"),
        })
        .collect_vec();

    let buttons = pieces
        .take_while_ref(|p| p.starts_with('('))
        .map(|button_str| {
            button_str
                .trim_matches(&['(', ')'])
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let joltage = pieces
        .next()
        .unwrap()
        .trim_matches(&['{', '}'])
        .split(',')
        .map(|num| num.parse::<u16>().unwrap())
        .collect();
    Machine {
        lights,
        buttons,
        joltage,
    }
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    input
        .lines()
        .map(|line| {
            let machine = parse_line(line);
            let start = iter::repeat(false).take(machine.lights.len()).collect_vec();
            min_distance_to(
                start,
                |v, _| v == &machine.lights,
                |v| {
                    machine
                        .buttons
                        .iter()
                        .map(|indexes| {
                            let mut next = v.clone();
                            for index in indexes {
                                next[*index] = !next[*index];
                            }
                            (next, 1)
                        })
                        .collect()
                },
            )
            .unwrap()
        })
        .sum::<u64>() as usize
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    input
        .lines()
        .map(|line| {
            let machine = parse_line(line);

            let solver = z3::Optimize::new();

            let mut joltage_sums = vec![z3::ast::Int::from_i64(0); machine.joltage.len()];

            let button_counts = machine
                .buttons
                .iter()
                .enumerate()
                .map(|(index, button)| {
                    let count = z3::ast::Int::new_const(format!("button{index}"));
                    solver.assert(&count.ge(0));
                    for index in button {
                        joltage_sums[*index] = joltage_sums[*index].clone() + count.clone();
                    }
                    count
                })
                .collect_vec();

            for (sum, value) in joltage_sums.iter().zip(machine.joltage) {
                solver.assert(&sum.eq(z3::ast::Int::from_i64(value as i64)));
            }

            let total_presses = button_counts.into_iter().reduce(|a, b| a + b).unwrap();

            solver.minimize(&total_presses);

            assert!(solver.check(&[]) == z3::SatResult::Sat);

            let model = solver.get_model().unwrap();
            model.eval(&total_presses, true).unwrap().as_i64().unwrap() as usize
        })
        .sum()
}
