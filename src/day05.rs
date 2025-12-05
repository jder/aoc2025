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

fn union(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> Option<RangeInclusive<usize>> {
    (a.end() >= b.start() && a.start() <= b.end())
        .then(|| *a.start().min(b.start())..=*(a.end().max(b.end())))
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let (ranges, _candidates) = parser().parse(input).unwrap();

    let disjoint_ranges = ranges.into_iter().fold(vec![], |accum, mut range| {
        // The newly-added range may overlap with multiple existing ranges,
        // so we remove any which overlap and continue to expand this one to cover
        // all overlaps.
        let accum = accum.into_iter().filter(|maybe_overlapping| {
            if let Some(union) = union(&maybe_overlapping, &range) {
                range = union;
                false
            } else {
                true
            }
        });

        let mut result = accum.collect_vec();
        result.push(range);
        result
    });

    disjoint_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}
