use std::{num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

struct SegmentDisplay {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
}

impl SegmentDisplay {
    fn count(&self) -> usize {
        (if self.a { 1 } else { 0 }
            + if self.b { 1 } else { 0 }
            + if self.c { 1 } else { 0 }
            + if self.d { 1 } else { 0 }
            + if self.e { 1 } else { 0 }
            + if self.f { 1 } else { 0 }
            + if self.g { 1 } else { 0 })
    }

    fn is_unique(&self) -> bool {
        match self.count() {
            2 => true,
            3 => true,
            4 => true,
            7 => true,
            _ => false,
        }
    }
}

impl FromStr for SegmentDisplay {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SegmentDisplay {
            a: s.contains('a'),
            b: s.contains('b'),
            c: s.contains('c'),
            d: s.contains('d'),
            e: s.contains('e'),
            f: s.contains('f'),
            g: s.contains('g'),
        })
    }
}

struct Line {
    signal: [SegmentDisplay; 10],
    output: [SegmentDisplay; 4],
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s
            .split_whitespace()
            .filter(|s| *s != "|")
            .map(|s| s.parse::<SegmentDisplay>().unwrap());

        Ok(Line {
            signal: [
                data.next().unwrap(),
                data.next().unwrap(),
                data.next().unwrap(),
                data.next().unwrap(),
                data.next().unwrap(),
                data.next().unwrap(),
                data.next().unwrap(),
                data.next().unwrap(),
                data.next().unwrap(),
                data.next().unwrap(),
            ],
            output: [
                data.next().unwrap(),
                data.next().unwrap(),
                data.next().unwrap(),
                data.next().unwrap(),
            ],
        })
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let lines = input
        .split('\n')
        .map(|line| line.parse::<Line>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut count = 0;

    for line in lines {
        count += line.output.iter().filter(|d| d.is_unique()).count();
    }

    Ok(count as u64)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::part1("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), Ok(0));
        assert_eq!(super::part1("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"), Ok(26));
    }
}
