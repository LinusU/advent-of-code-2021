use std::num::ParseIntError;

use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let counts =
        input
            .split_whitespace()
            .fold((0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0), |mem, number| {
                let mut chars = number.chars();

                return (
                    mem.0 + if chars.next().unwrap() == '1' { 1 } else { 0 },
                    mem.1 + if chars.next().unwrap() == '1' { 1 } else { 0 },
                    mem.2 + if chars.next().unwrap() == '1' { 1 } else { 0 },
                    mem.3 + if chars.next().unwrap() == '1' { 1 } else { 0 },
                    mem.4 + if chars.next().unwrap() == '1' { 1 } else { 0 },
                    mem.5 + if chars.next().unwrap() == '1' { 1 } else { 0 },
                    mem.6 + if chars.next().unwrap() == '1' { 1 } else { 0 },
                    mem.7 + if chars.next().unwrap() == '1' { 1 } else { 0 },
                    mem.8 + if chars.next().unwrap() == '1' { 1 } else { 0 },
                    mem.9 + if chars.next().unwrap() == '1' { 1 } else { 0 },
                    mem.10 + if chars.next().unwrap() == '1' { 1 } else { 0 },
                    mem.11 + if chars.next().unwrap() == '1' { 1 } else { 0 },
                    mem.12 + 1,
                );
            });

    let half = counts.12 / 2;

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
