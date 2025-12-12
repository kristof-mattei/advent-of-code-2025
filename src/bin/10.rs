use std::collections::VecDeque;
use std::vec::Vec;

use advent_of_code_2025::shared::{PartSolution, Parts};
use hashbrown::HashSet;

advent_of_code_2025::solution!(479_usize, 1_476_550_548_usize);

type Indicators = Vec<bool>;
type Schematics = Vec<Vec<usize>>;
type Joltage = Vec<usize>;

fn parse_input(input: &str) -> Vec<(Indicators, Schematics, Joltage)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let split = line.split(' ').collect::<Vec<_>>();

            let indicators = {
                split[0]
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Bad input"),
                    })
                    .collect::<Vec<_>>()
            };

            let schematics = {
                let mut schematics = vec![];

                for schematic in &split[1..split.len() - 1] {
                    schematics.push(
                        schematic
                            .trim_start_matches('(')
                            .trim_end_matches(')')
                            .split(',')
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect::<Vec<_>>(),
                    );
                }

                schematics
            };

            let joltage = {
                split[split.len() - 1]
                    .trim_start_matches('{')
                    .trim_end_matches('}')
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            };

            (indicators, schematics, joltage)
        })
        .collect::<Vec<(Indicators, Schematics, Joltage)>>()
}

fn solve_single_machine(
    &(ref indicators, ref schematics, _): &(Vec<bool>, Vec<Vec<usize>>, Vec<usize>),
) -> usize {
    let wanted_state = {
        let mut wanted_state = 0;

        for (bit, &on) in indicators.iter().enumerate() {
            if on {
                wanted_state |= 1_u128 << bit;
            }
        }

        wanted_state
    };

    let wirings = {
        let mut wirings = Vec::with_capacity(schematics.len());

        for wiring in schematics {
            let mut w = 0;

            for &bit in wiring {
                if bit < indicators.len() {
                    w |= 1_u128 << bit;
                }
            }

            wirings.push(w);
        }

        wirings
    };

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0, 0));
    visited.insert(0);

    while let Some((state, presses)) = queue.pop_front() {
        for &w in &wirings {
            let next_state = state ^ w;
            let next_presses = presses + 1;

            if next_state == wanted_state {
                return next_presses;
            }

            if !visited.contains(&next_state) {
                visited.insert(next_state);
                queue.push_back((next_state, next_presses));
            }
        }
    }

    usize::MAX
}

fn find_least_button_presses(machines: &Vec<(Indicators, Schematics, Joltage)>) -> Vec<usize> {
    let mut least_amount = Vec::with_capacity(machines.len());

    for machine in machines {
        least_amount.push(solve_single_machine(machine));
    }

    least_amount
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let lines = parse_input(input);

        let sum_of_least_amount_of_button_presses: usize =
            find_least_button_presses(&lines).iter().sum();

        sum_of_least_amount_of_button_presses.into()
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let red_tiles = parse_input(input);

        todo!()
    }
}

#[cfg(test)]
mod test {
    mod part_1 {
        use advent_of_code_2025::{test_example_part_1, test_part_1};

        #[test]
        fn outcome() {
            test_part_1!(479_usize);
        }

        #[test]
        fn example() {
            test_example_part_1!(7_usize);
        }
    }

    mod part_2 {
        use advent_of_code_2025::{test_example_part_2, test_part_2};

        #[test]
        fn outcome() {
            test_part_2!(1_476_550_548_usize);
        }

        #[test]
        fn example() {
            test_example_part_2!(24_isize);
        }
    }
}
