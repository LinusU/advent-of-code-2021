use std::{num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

enum Command {
    Down(u64),
    Forward(u64),
    Up(u64),
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();

        let command = iter.next().unwrap();
        let distance = iter.next().unwrap().parse()?;

        match command {
            "down" => Ok(Command::Down(distance)),
            "forward" => Ok(Command::Forward(distance)),
            "up" => Ok(Command::Up(distance)),
            _ => panic!("Unknown command: {}", command),
        }
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let commands = input
        .split('\n')
        .map(|line| line.parse::<Command>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut depth = 0;
    let mut horizontal = 0;

    for command in commands {
        match command {
            Command::Down(distance) => depth += distance,
            Command::Forward(distance) => horizontal += distance,
            Command::Up(distance) => depth -= distance,
        }
    }

    Ok(horizontal * depth)
}
