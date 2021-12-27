use std::{cmp::min, num::ParseIntError, ops::Sub};

use aoc_runner_derive::aoc;

fn median(sorted_list: &[u64]) -> f64 {
    let length = sorted_list.len();

    if length % 2 == 1 {
        sorted_list[length / 2] as f64
    } else {
        (sorted_list[length / 2] as f64 + sorted_list[length / 2 - 1] as f64) / 2.0
    }
}

fn mean(list: &[u64]) -> f64 {
    let sum: u64 = list.iter().sum();

    (sum as f64) / (list.len() as f64)
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

#[aoc(day7, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    let crabs = input
        .split(',')
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let target = mean(&crabs);
    let target_ceil = target.ceil() as u64;
    let target_floor = target.floor() as u64;

    let mut fuel_ceil = 0;
    let mut fuel_floor = 0;

    for crab in crabs {
        let distance_ceil = abs_difference(crab, target_ceil);
        let distance_floor = abs_difference(crab, target_floor);

        fuel_ceil += (distance_ceil * (distance_ceil + 1)) / 2;
        fuel_floor += (distance_floor * (distance_floor + 1)) / 2;
    }

    Ok(min(fuel_ceil, fuel_floor))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::part1("16,1,2,0,4,2,7,1,2,14"), Ok(37));
        assert_eq!(super::part2("16,1,2,0,4,2,7,1,2,14"), Ok(168));
    }
}
