use std::path::Path;
use std::{env, fs};

use super::quest::Quest;

fn read_file_base(filepath: impl AsRef<Path>) -> String {
    let f = fs::read_to_string(&filepath);
    f.unwrap_or_else(|e| {
        panic!(
            "Error reading file \"{}\": {e:?}",
            filepath.as_ref().to_str().unwrap()
        )
    })
}

/// Helper function that reads a text file to a string.
/// # Panics
///
/// if the file does not exist or cannot be read
#[must_use]
pub fn read_file(folder: &str, quest: &Quest) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("data").join(folder).join(format!("{}.txt", quest));

    read_file_base(filepath)
}

/// Helper function that reads a text file to string, appending a part suffix. E.g. like `01-2.txt`.
/// # Panics
///
/// if the file does not exist or cannot be read
#[must_use]
pub fn read_file_part(folder: &str, quest: &Quest, part: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{}-{}.txt", quest, part));

    read_file_base(filepath)
}

#[macro_export]
macro_rules! solution {
    () => {
        $crate::solution!(QuestSolution::None, QuestSolution::None);
    };
    ($solution_1:expr) => {
        $crate::solution!($solution_1, QuestSolution::None);
    };
    ($solution_1:expr, $solution_2:expr) => {
        /// The current quest.
        static QUEST: std::sync::LazyLock<$crate::shared::quest::Quest> =
            std::sync::LazyLock::new(|| {
                use std::path::Path;

                let path = Path::new(file!());
                let file_stem = path
                    .file_stem()
                    .expect("No stem found")
                    .to_str()
                    .expect("Invalid str");

                std::str::FromStr::from_str(file_stem).expect("Could not convert input to Quest")
            });

        fn main() {
            // use advent_of_code_2025::template::runner::*;
            let input = $crate::shared::solution::read_file("inputs", &QUEST);

            let part_1_expected_solution: QuestSolution = QuestSolution::from($solution_1);

            let s = Solution {};

            assert_eq!(part_1_expected_solution, s.part_1(&input));

            let part_2_expected_solution: QuestSolution = QuestSolution::from($solution_2);

            assert_eq!(part_2_expected_solution, s.part_2(&input));
            // run_part(part_one, &input, &QUEST, 1);
            // run_part(part_two, &input, &QUEST, 2);
        }

        pub struct Solution {}
    };
}
