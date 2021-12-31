use std::{
    fmt,
    num::ParseIntError,
    ops::{BitAnd, BitXor, Sub},
    str::FromStr,
};

use aoc_runner_derive::aoc;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct SegmentDisplay(u8);

impl SegmentDisplay {
    fn count(&self) -> u32 {
        self.0.count_ones()
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

impl BitAnd for SegmentDisplay {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        SegmentDisplay(self.0 & rhs.0)
    }
}

impl BitXor for SegmentDisplay {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Sub for SegmentDisplay {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}

impl fmt::Debug for SegmentDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SegmentDisplay({:07b})", self.0)
    }
}

impl FromStr for SegmentDisplay {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SegmentDisplay(s.chars().fold(0, |mem, s| match s {
            'a' => mem | 0b1000000,
            'b' => mem | 0b0100000,
            'c' => mem | 0b0010000,
            'd' => mem | 0b0001000,
            'e' => mem | 0b0000100,
            'f' => mem | 0b0000010,
            'g' => mem | 0b0000001,
            _ => unreachable!(),
        })))
    }
}

struct Line {
    signal: [SegmentDisplay; 10],
    output: [SegmentDisplay; 4],
}

impl Line {
    fn decoded_digits(&self) -> [SegmentDisplay; 10] {
        let mut signal = self.signal.to_vec();
        signal.sort_unstable_by_key(|display| display.count());

        let one = signal[0];
        assert_eq!(one.count(), 2);

        let seven = signal[1];
        assert_eq!(seven.count(), 3);

        let four = signal[2];
        assert_eq!(four.count(), 4);

        let eight = signal[9];
        assert_eq!(eight.count(), 7);

        let a = one ^ seven;
        assert_eq!(a.count(), 1);

        let g = signal[3] & signal[4] & signal[5] & signal[6] & signal[7] & signal[8] ^ a;
        assert_eq!(g.count(), 1);

        let e = eight - seven - four ^ g;
        assert_eq!(e.count(), 1);

        let nine = eight - e;
        assert_eq!(nine.count(), 6);

        let zero_and_six: Vec<SegmentDisplay> = [signal[6], signal[7], signal[8]]
            .iter()
            .filter(|s| **s != nine)
            .map(SegmentDisplay::clone)
            .collect();
        assert_eq!(zero_and_six.len(), 2);

        let (zero, six) = match (zero_and_six[0] & seven).count() {
            3 => (zero_and_six[0], zero_and_six[1]),
            2 => (zero_and_six[1], zero_and_six[0]),
            _ => unreachable!(),
        };

        let five = six - e;
        assert_eq!(five.count(), 5);

        let d = eight - zero;
        assert_eq!(d.count(), 1);

        let b = four - one - d;
        assert_eq!(b.count(), 1);

        let c = zero - six;
        assert_eq!(c.count(), 1);

        let f = one - c;
        assert_eq!(f.count(), 1);

        let two = eight - b - f;
        assert_eq!(two.count(), 5);

        let three = eight - b - e;
        assert_eq!(three.count(), 5);

        [zero, one, two, three, four, five, six, seven, eight, nine]
    }
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

#[aoc(day8, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    let lines = input
        .split('\n')
        .map(|line| line.parse::<Line>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut result = 0;

    for line in lines {
        let digits = line.decoded_digits();

        result += (digits.iter().position(|d| d == &line.output[0]).unwrap() * 1000) as u64;
        result += (digits.iter().position(|d| d == &line.output[1]).unwrap() * 100) as u64;
        result += (digits.iter().position(|d| d == &line.output[2]).unwrap() * 10) as u64;
        result += (digits.iter().position(|d| d == &line.output[3]).unwrap() * 1) as u64;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::part1("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), Ok(0));
        assert_eq!(super::part1("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"), Ok(26));
        assert_eq!(super::part2("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), Ok(5353));
        assert_eq!(super::part2("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"), Ok(61229));
    }
}
