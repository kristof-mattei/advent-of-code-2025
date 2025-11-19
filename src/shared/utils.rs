#[macro_export]
#[expect(clippy::crate_in_macro_def, reason = "We do want the parent one")]
macro_rules! test_part_1 {
    ($value:expr) => {{
        use advent_of_code_2025::shared::Quest;
        use advent_of_code_2025::shared::solution::read_file_part;

        use crate::Solution;

        assert_eq!(
            $value,
            (Solution {}).part_1(&read_file_part("inputs", &QUEST, 1))
        );
    }};
}

#[macro_export]
#[expect(clippy::crate_in_macro_def, reason = "We do want the parent one")]
macro_rules! test_part_2 {
    ($value:expr) => {{
        use advent_of_code_2025::shared::Quest;
        use advent_of_code_2025::shared::solution::read_file_part;

        use crate::Solution;

        assert_eq!(
            $value,
            (Solution {}).part_2(&read_file_part("inputs", &QUEST, 2))
        );
    }};
}

#[macro_export]
#[expect(clippy::crate_in_macro_def, reason = "We do want the parent one")]
macro_rules! test_part_3 {
    ($value:expr) => {{
        use advent_of_code_2025::shared::Quest;
        use advent_of_code_2025::shared::solution::read_file_part;

        use crate::Solution;

        assert_eq!(
            $value,
            (Solution {}).part_3(&read_file_part("inputs", &QUEST, 3))
        );
    }};
}

#[macro_export]
#[expect(clippy::crate_in_macro_def, reason = "We do want the parent one")]
macro_rules! test_example_part_1 {
    ($value:expr, $part:expr) => {{
        use advent_of_code_2025::shared::Quest;
        use advent_of_code_2025::shared::solution::read_file_part;

        use crate::Solution;

        assert_eq!(
            $value,
            (Solution {}).part_1(&read_file_part("examples", &QUEST, $part))
        );
    }};
    ($value:expr) => {{
        use advent_of_code_2025::shared::Quest;
        use advent_of_code_2025::shared::solution::read_file;

        use crate::Solution;

        assert_eq!($value, (Solution {}).part_1(&read_file("examples", &QUEST)));
    }};
}

#[macro_export]
#[expect(clippy::crate_in_macro_def, reason = "We do want the parent one")]
macro_rules! test_example_part_2 {
    ($value:expr, $part:expr) => {{
        use advent_of_code_2025::shared::Quest;
        use advent_of_code_2025::shared::solution::read_file_part;

        use crate::Solution;

        assert_eq!(
            $value,
            (Solution {}).part_2(&read_file_part("examples", &QUEST, $part))
        );
    }};
    ($value:expr) => {{
        use advent_of_code_2025::shared::Quest;
        use advent_of_code_2025::shared::solution::read_file;

        use crate::Solution;

        assert_eq!($value, (Solution {}).part_2(&read_file("examples", &QUEST)));
    }};
}

#[macro_export]
#[expect(clippy::crate_in_macro_def, reason = "We do want the parent one")]
macro_rules! test_example_part_3 {
    ($value:expr, $part:expr) => {{
        use advent_of_code_2025::shared::Quest;
        use advent_of_code_2025::shared::solution::read_file_part;

        use crate::Solution;

        assert_eq!(
            $value,
            (Solution {}).part_3(&read_file_part("examples", &QUEST, $part))
        );
    }};
    ($value:expr) => {{
        use advent_of_code_2025::shared::Quest;
        use advent_of_code_2025::shared::solution::read_file;

        use crate::Solution;

        assert_eq!($value, (Solution {}).part_3(&read_file("examples", &QUEST)));
    }};
}
