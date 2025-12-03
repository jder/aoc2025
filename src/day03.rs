use std::{ascii::Char, cmp::Ordering};

#[allow(unused)]
use crate::prelude::*;

fn largest_index(haystack: &[Char]) -> usize {
    // first largest, not last (which is what position_max does)
    haystack
        .iter()
        .position_max_by(|a, b| a.cmp(b).then(Ordering::Less))
        .unwrap()
}

fn digit_to_val(digit: Char) -> usize {
    digit.to_char().to_digit(10).unwrap() as usize
}

fn largest_joltage(haystack: &[Char]) -> usize {
    let biggest_index = largest_index(&haystack[..haystack.len() - 1]);
    let next_index = largest_index(&haystack[biggest_index + 1..]) + biggest_index + 1;

    digit_to_val(haystack[biggest_index]) * 10 + digit_to_val(haystack[next_index])
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    input
        .lines()
        .map(|line| largest_joltage(line.as_ascii().unwrap()))
        .sum()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    todo!()
}
