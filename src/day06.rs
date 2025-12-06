use std::{
    iter,
    ops::{Add, Mul},
};

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

fn transpose<'l>(lines: impl Iterator<Item = &'l str>) -> impl Iterator<Item = String> {
    let lines = lines.map(|l| l.as_bytes()).collect_vec();
    (0..lines.first().unwrap().len()).map(move |index| {
        String::from_utf8(
            lines
                .iter()
                .map(|line| line[index])
                .chain(iter::once(' ' as u8))
                .collect_vec(),
        )
        .unwrap()
    })
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let mut lines = input.lines();
    let numbers = transpose(lines.take_while_ref(|line| !line.starts_with(&['*', '+'])))
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    println!("{numbers:?}");

    let ops = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|c| match c {
            "+" => <usize as Add>::add,
            "*" => <usize as Mul>::mul,
            _ => panic!("unknown op {c}"),
        });

    ops.zip_eq(numbers)
        .map(|(op, nums)| nums.into_iter().reduce(op).unwrap())
        .sum()
}
