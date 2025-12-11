#![feature(hash_set_entry)]
#![feature(let_chains)]
#![feature(ascii_char)]
use std::{fmt::Display, path::Path};

use clap::Parser;

pub mod prelude {
    pub use super::graph;
    pub use super::grid::*;
    pub use bitvec;
    pub use euclid::{default::*, point2, vec2};
    pub use hashbag::HashBag;
    pub use itertools::Itertools;
    pub use num::{Float, Integer};
    pub use rayon::prelude::*;
    pub use regex;
    pub use regex::Regex;
    pub use std::collections::HashMap;
}

pub mod graph;
pub mod grid;

// Inspired by https://git.sr.ht/~gadanidis/aoc2024/tree/main/item/src/main.rs

type DayFn = Box<dyn Fn(&str, bool) -> String + Send + Sync + 'static>;

struct Runner {
    days: Vec<(String, (DayFn, DayFn))>,
}
impl Runner {
    fn new() -> Self {
        Self { days: Vec::new() }
    }

    fn register_day<T1, T2, F1, F2>(&mut self, name: &str, part1: F1, part2: F2)
    where
        F1: Fn(&str, bool) -> T1,
        F1: Send + Sync + 'static,
        T1: Display,
        F2: Fn(&str, bool) -> T2,
        F2: Send + Sync + 'static,
        T2: Display,
    {
        self.days.push((
            name.to_string(),
            (
                Box::new(move |input, sample| part1(input, sample).to_string()),
                Box::new(move |input, sample| part2(input, sample).to_string()),
            ),
        ));
    }

    fn run(&self, day: &str, part: usize, sample: bool) {
        let (part1, part2) = &self
            .days
            .iter()
            .find(|(name, _)| name == day)
            .expect("Day not found")
            .1;

        let contents = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("input")
                .join(if sample {
                    format!("{}-sample", day)
                } else {
                    day.to_string()
                })
                .with_extension("txt"),
        )
        .expect("Failed to read input");
        let input = contents.trim();

        let start = std::time::Instant::now();
        let result = match part {
            1 => part1(input, sample),
            2 => part2(input, sample),
            _ => panic!("Invalid part {}", part),
        };
        let elapsed = start.elapsed();
        println!(
            "{} part {}: {}\t({}µs)",
            day,
            part,
            result,
            elapsed.as_micros()
        );
    }

    fn run_all(&self) {
        for (day, _) in &self.days {
            self.run(day, 1, false);
            self.run(day, 2, false);
        }
    }
}

#[derive(Parser)]
struct Args {
    /// Day to run (default all)
    day: Option<String>,

    /// Part to run (1 or 2) (default both)
    #[clap(long, short)]
    part: Option<usize>,

    /// Use sample data
    #[clap(long, short)]
    sample: bool,
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

pub fn main() {
    env_logger::init();

    let mut runner = Runner::new();
    runner.register_day("day01", day01::part1, day01::part2);
    runner.register_day("day02", day02::part1, day02::part2);
    runner.register_day("day03", day03::part1, day03::part2);
    runner.register_day("day04", day04::part1, day04::part2);
    runner.register_day("day05", day05::part1, day05::part2);
    runner.register_day("day06", day06::part1, day06::part2);
    runner.register_day("day07", day07::part1, day07::part2);
    runner.register_day("day08", day08::part1, day08::part2);
    runner.register_day("day09", day09::part1, day09::part2);
    runner.register_day("day10", day10::part1, day10::part2);

    let args = Args::parse();

    if args.sample && args.day.is_none() {
        eprintln!("--sample requires --day");
        std::process::exit(1);
    }

    match args.day {
        Some(day) => match args.part {
            Some(part) => runner.run(&day, part, args.sample),
            None => {
                runner.run(&day, 1, args.sample);
                runner.run(&day, 2, args.sample);
            }
        },
        None => runner.run_all(),
    }
}
