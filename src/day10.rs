use std::iter;

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
    // This is a naive approach which doesn't actually complete in reasonable time. use z3?
    input
        .lines()
        .map(|line| {
            println!("{line}");
            let machine = parse_line(line);
            let start: JoltageVec = iter::repeat(0).take(machine.joltage.len()).collect();
            min_distance_to(
                start,
                |v, _| v == &machine.joltage,
                |v| {
                    machine
                        .buttons
                        .iter()
                        .map(|indexes| {
                            let mut next = v.clone();
                            for index in indexes {
                                next[*index] += 1;
                            }
                            (next, 1)
                        })
                        .filter(|(next, _)| {
                            next.iter()
                                .zip(&machine.joltage)
                                .all(|(next_joltage, goal_joltage)| next_joltage <= goal_joltage)
                        })
                        .collect()
                },
            )
            .unwrap()
        })
        .sum::<u64>() as usize
}
