use advent_of_code_2025::shared::grids::grid::Grid;
use advent_of_code_2025::shared::grids::{GridIter as _, Neighbors as _};
use advent_of_code_2025::shared::{PartSolution, Parts};
use hashbrown::HashSet;

advent_of_code_2025::solution!(1527_u64, 8690_u64);

enum Space {
    Empty,
    Paper,
}

fn parse_input(input: &str) -> Grid<Space> {
    let data = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    '@' => Space::Paper,
                    _ => panic!("Wrong input"),
                })
                .collect()
        })
        .collect::<Vec<Vec<Space>>>();

    Grid::new(data)
}

fn remove(parsed: &mut Grid<Space>) -> usize {
    // use hashset to avoid counting Paper rolls multiple times
    let mut to_remove = HashSet::new();

    'outer: for ((row_index, column_index), space) in parsed.row_column_index_value_iter() {
        if !matches!(space, Space::Paper) {
            continue;
        }

        let mut count = 0;

        for ((row_index, column_index), _) in parsed.hvd_neighbors(row_index, column_index) {
            if matches!(parsed[row_index][column_index], Space::Paper) {
                count += 1;

                if count > 3 {
                    continue 'outer;
                }
            }
        }

        to_remove.insert((row_index, column_index));
    }

    for &(row_index, column_index) in &to_remove {
        parsed[row_index][column_index] = Space::Empty;
    }

    to_remove.len()
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let mut grid = parse_input(input);

        let moveable = remove(&mut grid);

        PartSolution::USize(moveable)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let mut grid = parse_input(input);

        let mut removed_papers = 0;

        loop {
            let removed = remove(&mut grid);

            if removed == 0 {
                break;
            }

            removed_papers += removed;
        }

        PartSolution::USize(removed_papers)
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2025::{test_example_part_1, test_part_1};

        use crate::DAY;

        #[test]
        fn outcome() {
            test_part_1!(1527_u64);
        }

        #[test]
        fn example() {
            test_example_part_1!(13_u64);
        }
    }

    mod part_2 {
        use advent_of_code_2025::{test_example_part_2, test_part_2};

        use crate::DAY;

        #[test]
        fn outcome() {
            test_part_2!(8690_u64);
        }

        #[test]
        fn example() {
            test_example_part_2!(43_u64);
        }
    }
}
