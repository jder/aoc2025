use std::collections::HashSet;

#[allow(unused)]
use crate::prelude::*;

fn count_splits(previous_row: HashSet<Cell<char>>) -> usize {
    if previous_row.is_empty() {
        return 0;
    }

    let mut count = 0;

    let unsplit = previous_row.iter().flat_map(|one| one.offset(0, 1));
    let split = unsplit
        .flat_map(|this_row| {
            if *this_row.contents() == '^' {
                count += 1;
                vec![
                    this_row.offset(-1, 0).unwrap(),
                    this_row.offset(1, 0).unwrap(),
                ]
            } else {
                vec![this_row]
            }
        })
        .collect();

    count + count_splits(split)
}

pub fn part1(input: &str, _is_sample: bool) -> usize {
    let grid = Grid::new_with_lines(input.lines());

    let start = grid
        .cells()
        .find(|cell| *cell.contents() == 'S')
        .into_iter()
        .collect();
    count_splits(start)
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    todo!()
}
