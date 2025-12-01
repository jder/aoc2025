use std::iter;

#[allow(unused)]
use crate::prelude::*;

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let mut now: isize = 50;
    let sequence = input.lines().map(|line| {
        let (letter, num_st) = line.split_at(1);
        let dir = match letter {
            "L" => -1,
            "R" => 1,
            _ => panic!("unknown direction {letter}"),
        };
        now = (now + num_st.parse::<isize>().unwrap() * dir) % 100;
        now
    });

    sequence.filter(|&x| x == 0).count()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    todo!()
}
