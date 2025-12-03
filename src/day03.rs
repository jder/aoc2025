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

fn largest_joltage(haystack: &[Char], batteries: usize) -> usize {
    if batteries == 0 {
        0
    } else {
        let biggest_index = largest_index(&haystack[..haystack.len() - batteries + 1]);
        let rest = largest_joltage(&haystack[biggest_index + 1..], batteries - 1);

        digit_to_val(haystack[biggest_index]) * 10usize.pow(batteries as u32 - 1) + rest
    }
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    input
        .lines()
        .map(|line| largest_joltage(line.as_ascii().unwrap(), 2))
        .sum()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    input
        .lines()
        .map(|line| largest_joltage(line.as_ascii().unwrap(), 12))
        .sum()
}
