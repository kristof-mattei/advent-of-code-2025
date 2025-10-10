use advent_of_code_2025::shared::{PartSolution, Parts};

advent_of_code_2025::solution!(5, 5);

fn something(_input: &str) -> PartSolution {
    PartSolution::USize(5)
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        something(input)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        something(input)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {

        use advent_of_code_2025::{test_example_part_1, test_part_1};

        use crate::DAY;

        #[test]
        fn outcome() {
            test_part_1!(5);
        }

        #[test]
        fn example() {
            test_example_part_1!(5, 5);
        }
    }

    mod part_2 {
        use advent_of_code_2025::{test_example_part_1, test_part_1};

        use crate::DAY;

        #[test]
        fn outcome() {
            test_part_1!(5);
        }

        #[test]
        fn example() {
            test_example_part_1!(5);
        }
    }
}
