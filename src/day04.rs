#[allow(unused)]
use crate::prelude::*;

fn is_roll(cell: &Cell<char>) -> bool {
    *cell.contents() == '@'
}

fn accessible(grid: &Grid<char>) -> impl Iterator<Item = Cell<char>> {
    grid.cells()
        .filter(|cell| is_roll(cell) && cell.neighbors().filter(is_roll).count() < 4)
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    accessible(&Grid::new_with_lines(input.lines())).count()
}

fn remove_accessible(mut grid: Grid<char>) -> usize {
    let to_take = accessible(&grid).map(|cell| cell.location()).collect_vec();
    if to_take.is_empty() {
        0
    } else {
        for loc in &to_take {
            grid.set(*loc, ' ')
        }
        to_take.len() + remove_accessible(grid)
    }
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    remove_accessible(Grid::new_with_lines(input.lines()))
}
