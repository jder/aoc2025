use std::ops::{Add, Mul};

#[allow(unused)]
use crate::prelude::*;

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let mut lines = input.lines();
    let numbers = lines
        .take_while_ref(|line| !line.starts_with(&['*', '+']))
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let ops = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| match c {
            "+" => <usize as Add>::add,
            "*" => <usize as Mul>::mul,
            _ => panic!("unknown op {c}"),
        });

    ops.enumerate()
        .map(|(col, op)| numbers.iter().map(|row| row[col]).reduce(op).unwrap())
        .sum()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    todo!()
}
