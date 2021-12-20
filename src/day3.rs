use std::num::ParseIntError;

use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let numbers = input
        .split_whitespace()
        .map(|s| u64::from_str_radix(s, 2))
        .collect::<Result<Vec<_>, _>>()?;

    let counts =
        numbers
            .iter()
            .fold((0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0), |mem, number| {
                return (
                    mem.0 + ((number & 0x800) >> 11),
                    mem.1 + ((number & 0x400) >> 10),
                    mem.2 + ((number & 0x200) >> 9),
                    mem.3 + ((number & 0x100) >> 8),
                    mem.4 + ((number & 0x080) >> 7),
                    mem.5 + ((number & 0x040) >> 6),
                    mem.6 + ((number & 0x020) >> 5),
                    mem.7 + ((number & 0x010) >> 4),
                    mem.8 + ((number & 0x008) >> 3),
                    mem.9 + ((number & 0x004) >> 2),
                    mem.10 + ((number & 0x02) >> 1),
                    mem.11 + (number & 0x01),
                );
            });

    let half = numbers.len() as u64 / 2;

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
