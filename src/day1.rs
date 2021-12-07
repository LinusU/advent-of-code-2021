use std::num::ParseIntError;

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    Ok(input
        .split_whitespace()
        .map(|depth| depth.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?
        .windows(2)
        .map(|slice| if slice[1] > slice[0] { 1 } else { 0 })
        .sum())
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    Ok(input
        .split_whitespace()
        .map(|depth| depth.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?
        .windows(4)
        .map(|slice| if (slice[1] + slice[2] + slice[3]) > (slice[0] + slice[1] + slice[2]) { 1 } else { 0 })
        .sum())
}
