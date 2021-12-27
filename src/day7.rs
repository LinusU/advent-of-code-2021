use std::{num::ParseIntError, ops::Sub};

use aoc_runner_derive::aoc;

fn median(sorted_list: &[u64]) -> f64 {
    let length = sorted_list.len();

    if length % 2 == 1 {
        sorted_list[length / 2] as f64
    } else {
        (sorted_list[length / 2] as f64 + sorted_list[length / 2 - 1] as f64) / 2.0
    }
}

fn abs_difference<T: Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let mut crabs = input
        .split(',')
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    crabs.sort_unstable();

    let target = median(&crabs).round() as u64;

    let mut fuel = 0;

    for crab in crabs {
        fuel += abs_difference(crab, target);
    }

    Ok(fuel)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::part1("16,1,2,0,4,2,7,1,2,14"), Ok(37));
    }
}
