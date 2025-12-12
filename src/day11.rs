use crate::graph::all_paths;
#[allow(unused)]
use crate::prelude::*;

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let edges: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (source, edges) = line.split(':').collect_tuple().unwrap();
            let edges = edges.trim().split(' ').collect_vec();
            (source, edges)
        })
        .collect();

    all_paths("you", "out", |node| {
        edges.get(node).cloned().unwrap_or(vec![])
    })
    .count()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    todo!()
}
