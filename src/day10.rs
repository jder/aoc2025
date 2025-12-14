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
    // This is a naive approach which doesn't actually complete in reasonable time. use z3?
    input
        .lines()
        .map(|line| {
            println!("{line}");
            let machine = parse_line(line);
            let ordered_buttons = machine
                .buttons
                .into_iter()
                .sorted_by_key(|button| button.len())
                .rev()
                .collect_vec();
            let button_bits = ordered_buttons
                .iter()
                .map(|button| {
                    let mut bits = 0;
                    for index in button {
                        bits |= 1 << index
                    }
                    bits
                })
                .collect_vec();
            let mut reachable_by_index =
                button_bits
                    .iter()
                    .rev()
                    .fold(Vec::new(), |mut reachable, new_bits| {
                        let prev_reachable = reachable.last().copied().unwrap_or_default();
                        reachable.push(prev_reachable | *new_bits);
                        reachable
                    });
            reachable_by_index.reverse();
            let reachable_lsb_by_index = (0..button_bits.len())
                .map(|index| reachable_lsb(&button_bits[index..], &ordered_buttons[index..]))
                .collect_vec();
            let mut cache = (0..ordered_buttons.len()).map(|_| HashSet::new()).collect();

            // println!("{reachable_by_index:?}, {ordered_buttons:?}");
            best(
                0,
                &machine.joltage,
                &ordered_buttons,
                &reachable_by_index,
                &reachable_lsb_by_index,
                &mut cache,
            )
            .unwrap()
        })
        .sum()
}

fn reachable_lsb(button_bits: &[usize], buttons: &[Vec<usize>]) -> HashSet<u16> {
    let mut seen = HashSet::new();
    let mut queue = vec![0];
    while let Some(next) = queue.pop() {
        for bits in button_bits {
            let possible = next ^ bits;
            if seen.insert(possible as u16) {
                queue.push(possible);
            }
        }
    }
    println!(
        "{} are reachable from {} aka {:?}",
        seen.len(),
        button_bits.iter().map(|b| format!("{b:#b}")).join(", "),
        buttons
    );
    seen
}

fn lsb(needed: &JoltageVec) -> u16 {
    let mut result = 0;
    for (index, one_needed) in needed.iter().enumerate() {
        if one_needed & 0x1 == 1 {
            result |= 0x1 << index;
        }
    }
    result
}

fn best(
    index: usize,
    needed: &JoltageVec,
    ordered_buttons: &Vec<Vec<usize>>,
    reachable_by_index: &Vec<usize>,
    reachable_lsb_by_index: &Vec<HashSet<u16>>,
    unreachable: &mut Vec<HashSet<JoltageVec>>,
) -> Option<usize> {
    if index == 1 {
        println!("trying {needed:?}");
    }
    let needed_bits = button_bits_needed(needed);
    if needed.iter().all(|x| *x == 0) {
        Some(0)
    } else if index >= ordered_buttons.len() || !reachable_by_index[index] & needed_bits != 0 {
        // println!("fast reject {index}, {needed:?}");
        None
    } else if unreachable[index].contains(needed) {
        None
    } else if !reachable_lsb_by_index[index].contains(&lsb(needed)) {
        None
    } else {
        let button = &ordered_buttons[index];
        for take_count in (0..=max_takeable(button, needed)).rev() {
            let mut next = needed.clone();
            for index in button {
                next[*index] -= take_count
            }

            // println!("at {needed:?}: trying pushing {index} x {take_count}");
            if let Some(result) = best(
                index + 1,
                &next,
                ordered_buttons,
                reachable_by_index,
                reachable_lsb_by_index,
                unreachable,
            ) {
                return Some(result + take_count as usize);
            }
        }
        // println!("{index}, {needed:?} unreachable");
        unreachable[index].insert(needed.clone());
        None
    }
}

fn button_bits_needed(vec: &JoltageVec) -> usize {
    let mut bits = 0;
    for (index, amount) in vec.iter().enumerate() {
        if *amount > 0 {
            bits |= 1 << index;
        }
    }
    bits
}

fn max_takeable(button: &Vec<usize>, needed: &JoltageVec) -> u16 {
    button.iter().map(|index| needed[*index]).min().unwrap()
}
