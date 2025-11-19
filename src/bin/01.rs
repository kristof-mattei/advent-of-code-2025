use std::str::FromStr;

use advent_of_code_2025::shared::{Quest, QuestSolution};

advent_of_code_2025::solution!("Dalkryth");

enum Direction {
    Left(usize),
    Right(usize),
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Direction::Left(l) => write!(f, "L{}", l),
            &Direction::Right(r) => write!(f, "R{}", r),
        }
    }
}

fn map_direction(input: &str) -> Direction {
    let number: usize = FromStr::from_str(&input[1..]).unwrap();

    match input.chars().next().unwrap() {
        'R' => Direction::Right(number),
        'L' => Direction::Left(number),
        _ => {
            panic!()
        },
    }
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<Direction>) {
    // input is 3 lines

    let mut iter = input.lines();

    let names = iter.next().unwrap().split(',').collect::<Vec<&str>>();
    let _empty = iter.next();
    let directions = iter
        .next()
        .unwrap()
        .split(',')
        .map(map_direction)
        .collect::<Vec<Direction>>();

    (names, directions)
}

impl Quest for Solution {
    fn part_1(&self, input: &str) -> QuestSolution {
        let (names, directions) = parse_input(input);

        let mut index = 0;

        for direction in directions {
            match direction {
                Direction::Left(left) => {
                    if index < left {
                        index = 0;
                    } else {
                        index -= left;
                    }
                },
                Direction::Right(right) => index = (index + right).min(names.len() - 1),
            }
        }

        QuestSolution::String(names[index].to_owned())
    }

    fn part_2(&self, input: &str) -> QuestSolution {
        let (names, directions) = parse_input(input);

        let mut index = 0;

        for direction in directions {
            match direction {
                Direction::Left(left) => {
                    let rem = left % names.len();

                    if index < rem {
                        let diff = rem - index;
                        index = names.len() - diff;
                    } else {
                        index -= rem;
                    }
                },
                Direction::Right(right) => {
                    index = (index + right) % names.len();
                },
            }
        }

        QuestSolution::String(names[index].to_owned())
    }

    fn part_3(&self, input: &str) -> QuestSolution {
        let (mut names, directions) = parse_input(input);

        for direction in directions {
            match direction {
                Direction::Left(left) => {
                    let rem = left % names.len();

                    let swap = names.len() - rem;
                    if swap == names.len() {
                    } else {
                        names.swap(0, swap);
                    }
                },
                Direction::Right(right) => {
                    let swap = right % names.len();

                    names.swap(0, swap);
                },
            }
        }

        QuestSolution::String(names[0].to_owned())
    }
}

#[cfg(test)]
mod test {

    mod part_1 {
        use advent_of_code_2025::shared::QuestSolution;
        use advent_of_code_2025::{test_example_part_1, test_part_1};

        use crate::QUEST;

        #[test]
        fn outcome() {
            test_part_1!(QuestSolution::String("Dalkryth".into()));
        }

        #[test]
        fn example() {
            test_example_part_1!(QuestSolution::String("Fyrryn".into()));
        }
    }

    mod part_2 {
        use advent_of_code_2025::shared::QuestSolution;
        use advent_of_code_2025::{test_example_part_2, test_part_2};

        use crate::QUEST;

        #[test]
        fn outcome() {
            test_part_2!(QuestSolution::String("Mornfeth".into()));
        }

        #[test]
        fn example() {
            test_example_part_2!(QuestSolution::String("Elarzris".into()));
        }
    }

    mod part_3 {
        use advent_of_code_2025::shared::QuestSolution;
        use advent_of_code_2025::{test_example_part_3, test_part_3};

        use crate::QUEST;

        #[test]
        fn outcome() {
            test_part_3!(QuestSolution::String("Tharnlorath".into()));
        }

        #[test]
        fn example() {
            test_example_part_3!(QuestSolution::String("Drakzyph".into()), 3);
        }
    }
}
