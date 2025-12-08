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

fn count_timelines<'a>(
    start: Option<Cell<'a, char>>,
    cache: &mut HashMap<Cell<'a, char>, usize>,
) -> usize {
    let Some(start) = start else { return 1 };

    if let Some(result) = cache.get(&start) {
        *result
    } else {
        let result = if *start.contents() == '^' {
            count_timelines(start.offset(-1, 1), cache) + count_timelines(start.offset(1, 1), cache)
        } else {
            count_timelines(start.offset(0, 1), cache)
        };
        cache.insert(start, result);

        result
    }
}

pub fn part2(input: &str, _is_sample: bool) -> usize {
    let grid = Grid::new_with_lines(input.lines());

    let start = grid.cells().find(|cell| *cell.contents() == 'S');

    let mut cache = HashMap::new();
    count_timelines(start, &mut cache)
}
