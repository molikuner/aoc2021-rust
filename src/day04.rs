use std::fmt::{Display, Formatter};
use std::ops::Not;
use std::str::FromStr;

use indoc::indoc;

use aoc2021::TestCase;

pub(crate) struct Solution;

type BingoNumber = u8;

const BINGO_FIELD_SIZE: usize = 5;

#[derive(Copy, Clone)]
struct BingoFieldNumber {
    number: BingoNumber,
    crossed: bool,
}

impl Display for BingoFieldNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.crossed {
            // write it in blue
            write!(f, "\x1B[1;96;127m{}\x1B[0m", self.number)
        } else {
            write!(f, "{}", self.number)
        }
    }
}

struct BingoField {
    board: [BingoFieldNumber; BINGO_FIELD_SIZE * BINGO_FIELD_SIZE],
    won: bool,
}

impl BingoField {
    fn get(&self, x: usize, y: usize) -> &BingoFieldNumber {
        &self.board[BINGO_FIELD_SIZE * y as usize + x as usize]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut BingoFieldNumber {
        &mut self.board[BINGO_FIELD_SIZE * y as usize + x as usize]
    }

    fn cross_value(&mut self, value: BingoNumber) -> bool {
        for i in 0..self.board.len() {
            if self.board[i].number == value {
                self.board[i].crossed = true;
                return self.has_won_at_x(i % BINGO_FIELD_SIZE) || self.has_won_at_y(i / BINGO_FIELD_SIZE);
            }
        }
        return false;
    }

    fn has_won_at_y(&mut self, y: usize) -> bool {
        for x in 0..BINGO_FIELD_SIZE {
            if self.get(x, y).crossed.not() {
                return false;
            }
        };
        self.won = true;
        return true;
    }

    fn has_won_at_x(&mut self, x: usize) -> bool {
        for y in 0..BINGO_FIELD_SIZE {
            if self.get(x, y).crossed.not() {
                return false;
            }
        };
        self.won = true;
        return true;
    }

    fn score(&self, last_number: BingoNumber) -> u32 {
        let sum: u32 = self.board.iter().filter_map(|v| if v.crossed { None } else { Some(v.number as u32) }).sum();
        last_number as u32 * sum as u32
    }
}

impl Display for BingoField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let result: String = self.board.iter().enumerate().map(|(i, number)| {
            if i != 0 && i % BINGO_FIELD_SIZE == 0 {
                format!("\n{}", number)
            } else {
                format!(" {}", number)
            }
        }).collect();

        write!(f, "{}", result)
    }
}

impl FromStr for BingoField {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Vec<BingoNumber>> = s.lines().map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect();

        if lines.len() != BINGO_FIELD_SIZE {
            return Err(format!("There need to be exactly {} columns", BINGO_FIELD_SIZE));
        }

        let mut f = BingoField {
            board: [BingoFieldNumber { number: 0, crossed: false }; BINGO_FIELD_SIZE * BINGO_FIELD_SIZE],
            won: false,
        };

        for (y, line) in lines.iter().enumerate() {
            if line.len() != BINGO_FIELD_SIZE {
                return Err(format!("Every row needs to contain exactly {} elements. Row {} doesn't ({:?})", BINGO_FIELD_SIZE, y, line));
            }

            for (x, value) in line.iter().enumerate() {
                f.get_mut(x, y).number = *value;
            }
        }

        Ok(f)
    }
}

impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        let (numbers, fields) = input.split_once("\n\n").unwrap();
        let numbers: Vec<_> = numbers.split(',').map(|num| num.parse::<BingoNumber>().unwrap()).collect();
        let mut fields: Vec<_> = fields.split("\n\n").map(|field| BingoField::from_str(field).unwrap()).collect();

        for value in numbers {
            for (i, field) in fields.iter_mut().enumerate() {
                if field.won.not() && field.cross_value(value) {
                    return format!("last_number={}; winner={}; score={}", value, i, field.score(value));
                }
            }
        }
        format!("Could not find any winner")
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"
                    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

                    22 13 17 11  0
                    8  2 23  4 24
                    21  9 14 16  7
                     6 10  3 18  5
                     1 12 20 15 19

                     3 15  0  2 22
                     9 18 13 17  5
                    19  8  7 25 23
                    20 11 10 24  4
                    14 21 16 12  6

                    14 21 17 24  4
                    10 16 15  9 19
                    18  8 23 26 20
                    22 11 13  6  5
                     2  0 12  3  7
                "}.to_string(),
                output: "last_number=24; winner=2; score=4512".to_string(),
            }
        ])
    }

    fn part2(&self, input: &str) -> String {
        let (numbers, fields) = input.split_once("\n\n").unwrap();
        let numbers: Vec<_> = numbers.split(',').map(|num| num.parse::<BingoNumber>().unwrap()).collect();
        let mut fields: Vec<_> = fields.split("\n\n").map(|field| BingoField::from_str(field).unwrap()).collect();

        for value in numbers {
            let mut current_board_count = fields.iter().filter(|f| f.won.not()).count();
            for (i, field) in fields.iter_mut().enumerate() {
                if field.won.not() && field.cross_value(value) {
                    if current_board_count == 1 {
                        return format!("last_number={}; last_winner={}; score={}", value, i, field.score(value));
                    } else {
                        current_board_count -= 1;
                    }
                }
            }
        }
        format!("Could not find any winner")
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"
                    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

                    22 13 17 11  0
                    8  2 23  4 24
                    21  9 14 16  7
                     6 10  3 18  5
                     1 12 20 15 19

                     3 15  0  2 22
                     9 18 13 17  5
                    19  8  7 25 23
                    20 11 10 24  4
                    14 21 16 12  6

                    14 21 17 24  4
                    10 16 15  9 19
                    18  8 23 26 20
                    22 11 13  6  5
                     2  0 12  3  7
                "}.to_string(),
                output: "last_number=13; last_winner=1; score=1924".to_string(),
            }
        ])
    }
}
