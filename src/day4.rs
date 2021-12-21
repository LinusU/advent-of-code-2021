use std::num::ParseIntError;

use aoc_runner_derive::aoc;

#[derive(Clone)]
struct BingoCell {
    number: u8,
    marked: bool,
}

impl BingoCell {
    fn new(number: u8) -> BingoCell {
        BingoCell {
            number,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }

    fn score(&self) -> u64 {
        if self.marked {
            0
        } else {
            self.number as u64
        }
    }
}

#[derive(Clone)]
struct BingoBoard([[BingoCell; 5]; 5]);

impl BingoBoard {
    fn from_lines<'a, I>(lines: &mut I) -> Option<BingoBoard>
    where
        I: Iterator<Item = &'a str>,
    {
        let lines = [
            lines.next()?,
            lines.next()?,
            lines.next()?,
            lines.next()?,
            lines.next()?,
        ];

        let foo = lines.map(|line| {
            let mut numbers = line.split_whitespace().map(|s| s.parse::<u8>().unwrap());

            [
                BingoCell::new(numbers.next().unwrap()),
                BingoCell::new(numbers.next().unwrap()),
                BingoCell::new(numbers.next().unwrap()),
                BingoCell::new(numbers.next().unwrap()),
                BingoCell::new(numbers.next().unwrap()),
            ]
        });

        Option::Some(BingoBoard(foo))
    }

    fn mark(&mut self, number: u8) {
        for row in &mut self.0 {
            for cell in row {
                if cell.number == number {
                    cell.mark();
                }
            }
        }
    }

    fn score(&self) -> u64 {
        self.0.iter().fold(0, |mem, row| {
            mem + row.iter().fold(0, |mem, cell| mem + cell.score())
        })
    }

    fn is_winning(&self) -> bool {
        for i in 0..5 {
            if self.0[i][0].marked
                && self.0[i][1].marked
                && self.0[i][2].marked
                && self.0[i][3].marked
                && self.0[i][4].marked
            {
                return true;
            }

            if self.0[0][i].marked
                && self.0[1][i].marked
                && self.0[2][i].marked
                && self.0[3][i].marked
                && self.0[4][i].marked
            {
                return true;
            }
        }

        false
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let mut lines = input.split('\n');

    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut boards = Vec::<BingoBoard>::new();

    loop {
        let _empty_line = lines.next();

        match BingoBoard::from_lines(&mut lines) {
            Some(board) => boards.push(board),
            None => break,
        }
    }

    for number in numbers {
        for board in &mut boards {
            board.mark(number);

            if board.is_winning() {
                return Ok(board.score() * (number as u64));
            }
        }
    }

    panic!("No winning board")
}
