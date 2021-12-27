use std::num::ParseIntError;

use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let mut fishes = [0; 9];

    for age in input.split(',') {
        fishes[age.parse::<usize>()?] += 1;
    }

    for _ in 0..80 {
        fishes = [
            fishes[1],
            fishes[2],
            fishes[3],
            fishes[4],
            fishes[5],
            fishes[6],
            fishes[7] + fishes[0],
            fishes[8],
            fishes[0],
        ]
    }

    Ok(fishes.iter().sum())
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    let mut fishes = [0; 9];

    for age in input.split(',') {
        fishes[age.parse::<usize>()?] += 1;
    }

    for _ in 0..256 {
        fishes = [
            fishes[1],
            fishes[2],
            fishes[3],
            fishes[4],
            fishes[5],
            fishes[6],
            fishes[7] + fishes[0],
            fishes[8],
            fishes[0],
        ]
    }

    Ok(fishes.iter().sum())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::part1("3,4,3,1,2"), Ok(5934));
        assert_eq!(super::part2("3,4,3,1,2"), Ok(26984457539));
    }
}
