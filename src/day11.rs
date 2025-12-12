use crate::graph::count_paths;
#[allow(unused)]
use crate::prelude::*;

fn find_paths<'a>(input: &'a str, start: &'a str, end: &'a str) -> usize {
    let edges: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|line| {
            let (source, edges) = line.split(':').collect_tuple().unwrap();
            let edges = edges.trim().split(' ').collect_vec();
            (source, edges)
        })
        .collect();

    count_paths(start, end, move |node| {
        edges.get(node).cloned().unwrap_or(vec![])
    })
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    find_paths(input, "you", "out")
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let s_to_fft = find_paths(input, "svr", "fft");
    let s_to_dac = find_paths(input, "svr", "dac");

    let fft_to_dac = find_paths(input, "fft", "dac");
    let dac_to_fft = find_paths(input, "dac", "fft");

    let fft_to_end = find_paths(input, "fft", "out");
    let dac_to_end = find_paths(input, "dac", "out");

    s_to_fft * fft_to_dac * dac_to_end + s_to_dac * dac_to_fft * fft_to_end
}
