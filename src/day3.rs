use std::num::ParseIntError;
use std::str::FromStr;

use aoc_runner_derive::aoc;

#[derive(Clone, Copy)]
struct U12(u64);

impl U12 {
    fn bit_at(&self, index: usize) -> u64 {
        assert!(index < 12);
        (self.0 >> (11 - index)) & 1
    }
}

impl FromStr for U12 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(U12(u64::from_str_radix(s, 2)?))
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let mut num_lines = 0;
    let mut counts = (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);

    for number in input.split_whitespace() {
        let number = U12::from_str(number)?;

        num_lines += 1;
        counts.0 += number.bit_at(0);
        counts.1 += number.bit_at(1);
        counts.2 += number.bit_at(2);
        counts.3 += number.bit_at(3);
        counts.4 += number.bit_at(4);
        counts.5 += number.bit_at(5);
        counts.6 += number.bit_at(6);
        counts.7 += number.bit_at(7);
        counts.8 += number.bit_at(8);
        counts.9 += number.bit_at(9);
        counts.10 += number.bit_at(10);
        counts.11 += number.bit_at(11);
    }

    let half = num_lines as u64 / 2;

    let gamma = (if counts.0 > half { 2048 } else { 0 }
        + if counts.1 > half { 1024 } else { 0 }
        + if counts.2 > half { 512 } else { 0 }
        + if counts.3 > half { 256 } else { 0 }
        + if counts.4 > half { 128 } else { 0 }
        + if counts.5 > half { 64 } else { 0 }
        + if counts.6 > half { 32 } else { 0 }
        + if counts.7 > half { 16 } else { 0 }
        + if counts.8 > half { 8 } else { 0 }
        + if counts.9 > half { 4 } else { 0 }
        + if counts.10 > half { 2 } else { 0 }
        + if counts.11 > half { 1 } else { 0 });

    let epsilon = (if counts.0 < half { 2048 } else { 0 }
        + if counts.1 < half { 1024 } else { 0 }
        + if counts.2 < half { 512 } else { 0 }
        + if counts.3 < half { 256 } else { 0 }
        + if counts.4 < half { 128 } else { 0 }
        + if counts.5 < half { 64 } else { 0 }
        + if counts.6 < half { 32 } else { 0 }
        + if counts.7 < half { 16 } else { 0 }
        + if counts.8 < half { 8 } else { 0 }
        + if counts.9 < half { 4 } else { 0 }
        + if counts.10 < half { 2 } else { 0 }
        + if counts.11 < half { 1 } else { 0 });

    Ok(gamma * epsilon)
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    let numbers = input
        .split_whitespace()
        .map(|src| U12::from_str(src))
        .collect::<Result<Vec<_>, _>>()?;

    let oxygen_generator_rating = {
        let mut pos = 0;
        let mut filtered_numbers = numbers.clone();

        loop {
            let ones_in_position = filtered_numbers
                .iter()
                .filter(|number| number.bit_at(pos) == 1)
                .count();

            let target = if (ones_in_position * 2) >= filtered_numbers.len() {
                1
            } else {
                0
            };

            filtered_numbers.retain(|number| number.bit_at(pos) == target);

            if filtered_numbers.len() == 1 {
                break filtered_numbers[0];
            }

            pos += 1;
        }
    };

    let co2_scrubber_rating = {
        let mut pos = 0;
        let mut filtered_numbers = numbers;

        loop {
            let ones_in_position = filtered_numbers
                .iter()
                .filter(|number| number.bit_at(pos) == 1)
                .count();

            let target = if (ones_in_position * 2) < filtered_numbers.len() {
                1
            } else {
                0
            };

            filtered_numbers.retain(|number| number.bit_at(pos) == target);

            if filtered_numbers.len() == 1 {
                break filtered_numbers[0];
            }

            pos += 1;
        }
    };

    Ok(oxygen_generator_rating.0 * co2_scrubber_rating.0)
}
