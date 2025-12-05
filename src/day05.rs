use std::ops::RangeInclusive;

use chumsky::{Parser, prelude::*, text};

#[allow(unused)]
use crate::prelude::*;

type Ranges = Vec<RangeInclusive<usize>>;

fn parser<'s>() -> impl Parser<'s, &'s str, (Ranges, Vec<usize>), extra::Err<Rich<'s, char>>> {
    let number = text::int(10).map(|s: &str| s.parse::<usize>().unwrap());

    let range = number
        .then_ignore(just('-'))
        .then(number)
        .map(|(first, last)| first..=last);

    let ranges = range
        .separated_by(just("\n"))
        .allow_trailing()
        .collect::<Vec<_>>();
    let numbers = number.separated_by(just("\n")).collect();

    ranges.then_ignore(just("\n")).then(numbers)
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let (ranges, candidates) = parser().parse(input).unwrap();

    candidates
        .iter()
        .filter(|c| ranges.iter().any(|r| r.contains(c)))
        .count()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    todo!()
}
