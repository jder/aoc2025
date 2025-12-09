#[allow(unused)]
use crate::prelude::*;

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let points = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse().unwrap())
                .collect_tuple::<(isize, isize)>()
                .unwrap()
        })
        .collect_vec();
    points
        .iter()
        .cartesian_product(points.iter())
        .map(|(a, b)| ((a.0 - b.0 + 1) * (a.1 - b.1 + 1)).abs())
        .max()
        .unwrap() as usize
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    todo!()
}
