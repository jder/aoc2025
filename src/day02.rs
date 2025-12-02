#[allow(unused)]
use crate::prelude::*;

fn two_repeated(value: &usize) -> bool {
    let digits = value.to_string();
    digits[..digits.len() / 2] == digits[digits.len() / 2..]
}

fn any_repeated(value: &usize) -> bool {
    let string = value.to_string();
    let digits = string.as_bytes();
    (1..=digits.len() / 2).any(|length| digits.chunks(length).all_equal())
}

fn sum_invalid(input: &str, invalid_fn: fn(&usize) -> bool) -> usize {
    let ranges = input.split(",");
    ranges
        .map(|range_str| {
            let (low, high) = range_str.split("-").collect_tuple().unwrap();
            let low = low.parse().unwrap();
            let high = high.parse().unwrap();

            (low..=high).filter(invalid_fn).sum::<usize>()
        })
        .sum()
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    sum_invalid(input, two_repeated)
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    sum_invalid(input, any_repeated)
}
