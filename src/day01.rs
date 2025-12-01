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
        now = (now + num_st.parse::<isize>().unwrap() * dir).rem_euclid(100);
        now
    });

    sequence.filter(|&x| x == 0).count()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let mut now: isize = 50;
    let zeros = input.lines().map(|line| {
        let (letter, num_st) = line.split_at(1);
        let dir = match letter {
            "L" => -1,
            "R" => 1,
            _ => panic!("unknown direction {letter}"),
        };
        let distance = num_st.parse::<isize>().unwrap();
        let loops = distance / 100;
        let next = (now + distance * dir).rem_euclid(100);

        let extras = if (dir.signum() != (next - now).signum() || next == 0) && now != 0 {
            1
        } else {
            0
        };

        now = next;
        loops + extras
    });

    zeros.sum::<isize>() as usize
}
