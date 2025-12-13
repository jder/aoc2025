#[allow(unused)]
use crate::prelude::*;

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let sections = input.split("\n\n").collect_vec();

    let piece_counts = sections[..sections.len() - 1]
        .iter()
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| line.chars().filter(|char| *char == '#').count())
                .sum::<usize>()
        })
        .collect_vec();

    let requests = sections[sections.len() - 1].lines().map(|request| {
        let (dimensions, needs) = request.split(':').collect_tuple().unwrap();
        let dimensions: (usize, usize) = dimensions
            .split('x')
            .map(|d| d.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let needs = needs
            .trim()
            .split_whitespace()
            .map(|d| d.parse::<usize>().unwrap())
            .collect_vec();
        (dimensions, needs)
    });

    requests
        .map(|(dimensions, needs)| {
            let total_filled = needs
                .iter()
                .zip(piece_counts.iter())
                .map(|(need, count)| need * count)
                .sum::<usize>();
            let total_space = needs.iter().map(|need| need * 9).sum::<usize>();

            let available_space = dimensions.0 * dimensions.1;

            if total_filled > available_space {
                0
            } else if total_space <= available_space {
                1
            } else {
                panic!("unknown")
            }
        })
        .sum()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    todo!()
}
