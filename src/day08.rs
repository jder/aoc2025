use std::ops::Mul;

use euclid::point3;

#[allow(unused)]
use crate::prelude::*;

fn join(a: usize, b: usize, subgraph_roots: &mut Vec<usize>) {
    let (new, old) = if a < b { (a, b) } else { (b, a) };
    for root in subgraph_roots {
        if *root == old {
            *root = new
        }
    }
}

pub fn part1(input: &str, is_sample: bool) -> usize {
    let num_joins = if is_sample { 10 } else { 1000 };
    let points: Vec<Point3D<isize>> = input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(",")
                .map(|coord| coord.parse().unwrap())
                .collect_tuple()
                .unwrap();
            point3(x, y, z)
        })
        .collect_vec();

    let distances = points
        .iter()
        .enumerate()
        .cartesian_product(points.iter().enumerate())
        .flat_map(|((i0, p0), (i1, p1))| (i0 < i1).then(|| ((*p0 - *p1).square_length(), i0, i1)))
        .sorted()
        .collect_vec();

    let mut subgraph_roots = (0..points.len()).collect_vec();
    for (_dist, a, b) in distances.into_iter().take(num_joins) {
        let root_a = subgraph_roots[a];
        let root_b = subgraph_roots[b];
        if root_a != root_b {
            join(root_a, root_b, &mut subgraph_roots);
        }
    }

    subgraph_roots
        .iter()
        .counts()
        .into_values()
        .sorted()
        .rev()
        .take(3)
        .reduce(<usize as Mul>::mul)
        .unwrap()
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    todo!()
}
