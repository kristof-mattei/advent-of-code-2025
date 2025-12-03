use std::mem::swap;

use advent_of_code_2025::shared::{PartSolution, Parts};

advent_of_code_2025::solution!(17229, 170_520_923_035_051_u64);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<Vec<u8>>>()
}

fn two_batteries(battery_pack: &[u8]) -> (Option<u32>, Option<u32>) {
    let mut b1 = Option::None;
    let mut b2 = Option::None;

    for battery in battery_pack.iter().rev() {
        let battery: u32 = (*battery).into();

        if b2.is_none() {
            b2 = Some(battery);
            continue;
        }

        if Some(battery) >= b1 {
            if b2 < b1 {
                swap(&mut b1, &mut b2);
            }

            b1 = Some(battery);
            continue;
        }
    }

    (b1, b2)
}

fn twelve_batteries(battery_pack: &[u8]) -> Vec<u8> {
    let mut result = battery_pack[(battery_pack.len() - 12)..].to_vec();

    for b in battery_pack.iter().rev().skip(12) {
        insert_b(*b, &mut result);
    }

    result
}

fn insert_b(mut new_b: u8, new_battery_pack: &mut [u8]) {
    for b in new_battery_pack.iter_mut() {
        if new_b >= *b {
            swap(b, &mut new_b);
        } else {
            break;
        }
    }
}

impl Parts for Solution {
    fn part_1(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        let mut sum = 0;

        for battery_pack in parsed {
            let (b1, b2) = two_batteries(&battery_pack);

            sum += b1.unwrap_or_default() * 10;
            sum += b2.unwrap_or_default();
        }

        PartSolution::U32(sum)
    }

    fn part_2(&self, input: &str) -> PartSolution {
        let parsed = parse_input(input);

        let mut sum: u64 = 0;

        for battery_pack in parsed {
            let twelve_top = twelve_batteries(&battery_pack);

            for (index, b) in twelve_top.iter().rev().enumerate() {
                let multiplier = 10_u64.pow(u32::try_from(index).unwrap());
                let b = u64::from(*b);

                sum += b * multiplier;
            }
        }

        PartSolution::U64(sum)
    }
}

#[cfg(test)]
mod test {

    mod part_1 {

        use advent_of_code_2025::{test_example_part_1, test_part_1};

        use crate::DAY;

        #[test]
        fn outcome() {
            test_part_1!(17229);
        }

        #[test]
        fn example() {
            test_example_part_1!(357);
        }
    }

    mod part_2 {
        use advent_of_code_2025::{test_example_part_2, test_part_2};

        use crate::DAY;

        #[test]
        fn outcome() {
            test_part_2!(170_520_923_035_051_u64);
        }

        #[test]
        fn example() {
            test_example_part_2!(3_121_910_778_619_u64);
        }
    }
}
