use std::num::ParseIntError;

use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let mut num_lines = 0;
    let mut counts = (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);

    for number in input.split_whitespace() {
        let number = u64::from_str_radix(number, 2)?;

        num_lines += 1;
        counts.0 += (number & 0x800) >> 11;
        counts.1 += (number & 0x400) >> 10;
        counts.2 += (number & 0x200) >> 9;
        counts.3 += (number & 0x100) >> 8;
        counts.4 += (number & 0x080) >> 7;
        counts.5 += (number & 0x040) >> 6;
        counts.6 += (number & 0x020) >> 5;
        counts.7 += (number & 0x010) >> 4;
        counts.8 += (number & 0x008) >> 3;
        counts.9 += (number & 0x004) >> 2;
        counts.10 += (number & 0x02) >> 1;
        counts.11 += number & 0x01;
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
