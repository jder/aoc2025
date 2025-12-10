use std::iter;

use crate::graph::flood_fill_from;
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

fn walk_between_inclusive(a: Location, b: Location) -> impl Iterator<Item = Location> {
    let dx = (b.x - a.x).signum();
    let dy = (b.y - a.y).signum();

    assert!(dx == 0 || dy == 0);

    let mut next = Some(a);
    iter::from_fn(move || {
        let to_return = next;
        match to_return {
            None => None,
            Some(n) if n == b => {
                next = None;
                to_return
            }
            Some(n) => {
                next = Some(n + vec2(dx, dy));
                to_return
            }
        }
    })
}

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Border,
    Inside,
    Outside,
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let points = input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect_tuple::<(isize, isize)>()
                .unwrap();
            point2(x, y)
        })
        .collect_vec();

    let mut tiles = Grid::new(
        Tile::Inside,
        points.iter().map(|p| p.x).max().unwrap() as usize + 1,
        points.iter().map(|p| p.y).max().unwrap() as usize + 1,
    );

    let pairs = points.iter().circular_tuple_windows();
    for (&a, &b) in pairs {
        for loc in walk_between_inclusive(a, b) {
            tiles.set(loc, Tile::Border);
        }
    }

    println!("set borders");

    let grid_corners = [
        point2(0, 0),
        point2(tiles.width() as isize - 1, 0),
        point2(tiles.width() as isize - 1, tiles.height() as isize - 1),
        point2(0, tiles.height() as isize - 1),
    ];
    let mut to_visit = grid_corners
        .into_iter()
        .circular_tuple_windows()
        .flat_map(|(corner_a, corner_b)| walk_between_inclusive(corner_a, corner_b))
        .collect_vec();

    while let Some(next) = to_visit.pop() {
        if let Some(cell) = tiles.cell(next) {
            if *cell.contents() == Tile::Inside {
                to_visit.extend(cardinal_neighbors(next));
                tiles.set(next, Tile::Outside);
            }
        }
    }

    println!("done fill");

    fn rect_size((a, b): &(Point2D<isize>, Point2D<isize>)) -> isize {
        ((a.x - b.x + 1) * (a.y - b.y + 1)).abs()
    }

    fn is_valid(a: Point2D<isize>, b: Point2D<isize>, tiles: &Grid<Tile>) -> bool {
        let corners = [a, point2(a.x, b.y), b, point2(b.x, a.y)];
        let valid = corners
            .into_iter()
            .circular_tuple_windows()
            .flat_map(|(start, end)| walk_between_inclusive(start, end))
            .all(|loc| *tiles.cell(loc).unwrap().contents() != Tile::Outside);

        valid
    }

    let sorted_candidates = points
        .iter()
        .copied()
        .cartesian_product(points.iter().copied())
        .sorted_by_key(rect_size)
        .rev()
        .collect_vec();

    println!("have candidates");

    sorted_candidates
        .iter()
        .find(|(a, b)| is_valid(*a, *b, &tiles))
        .map(rect_size)
        .unwrap() as usize
}
