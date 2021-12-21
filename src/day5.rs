use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

struct Line((u32, u32), (u32, u32));

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.0 .0 == self.1 .0 || self.0 .1 == self.1 .1
    }

    fn iter_points(&self) -> impl Iterator<Item = (u32, u32)> {
        let delta = (
            (self.1 .0 as i64) - (self.0 .0 as i64),
            (self.1 .1 as i64) - (self.0 .1 as i64),
        );

        let start = self.0.clone();
        let step = (delta.0.signum(), delta.1.signum());
        let steps = delta.0.abs() + delta.1.abs();

        (0..=steps).map(move |index| {
            (
                start.0 + (step.0 * index) as u32,
                start.1 + (step.1 * index) as u32,
            )
        })
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" -> ");

        let lhs = iter.next().unwrap();
        let rhs = iter.next().unwrap();

        let numbers = lhs
            .split(',')
            .chain(rhs.split(','))
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self((numbers[0], numbers[1]), (numbers[2], numbers[3])))
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let lines = input
        .split('\n')
        .map(|depth| depth.parse::<Line>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut result = 0;
    let mut smoke = HashMap::<(u32, u32), u64>::new();

    for line in lines {
        if line.is_horizontal_or_vertical() {
            for point in line.iter_points() {
                *smoke.entry(point).or_insert(0) += 1;

                if smoke[&point] == 2 {
                    result += 1;
                }
            }
        }
    }

    Ok(result)
}
