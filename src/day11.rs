use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use indoc::indoc;

use aoc2021::TestCase;
use crate::field::Field;

pub(crate) struct Solution;

struct WhaleField(Field<u8>);

impl WhaleField {
    fn increase(&mut self) {
        for x in self.iter_mut() {
            *x = *x + 1;
        }
    }

    fn flash_step(&mut self) -> usize {
        let mut flashes = 0usize;
        for (x, y) in self.iter_coordinates() {
            if self[(x, y)] > 9 {
                self[(x, y)] = 0;
                for (x, y) in self.iter_adjacent_coordinates(x, y, true) {
                    if self[(x, y)] != 0 {
                        self[(x, y)] += 1;
                    }
                }
                flashes += 1;
            }
        }
        flashes
    }

    fn flash(&mut self) -> usize {
        let mut all_flashes = 0;
        let mut new_flashes = self.flash_step();
        while new_flashes > 0 {
            all_flashes += new_flashes;
            new_flashes = self.flash_step();
        }
        all_flashes
    }
}

impl FromStr for WhaleField {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(WhaleField(Field::<u8>::from_str(s)?))
    }
}

impl Deref for WhaleField {
    type Target = Field<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WhaleField {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for WhaleField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.iter_field().enumerate().fold(Ok(()), |res, (i, column)| {
            res
                .and_then(|_| write!(f, "{}", if i == 0 { "" } else { "\n" }))
                .and_then(|_| column.fold(Ok(()), |res, value|
                    res.and_then(|_| write!(f, "{}", value)))
                )
        })
    }
}

const STEPS: u8 = 100;

impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        let mut input = WhaleField::from_str(input).unwrap();

        let mut flashes = 0usize;
        for _/*i*/ in 0..STEPS {
            // println!("\n{}:\n{}", i, input);
            input.increase();
            flashes += input.flash();
        }

        // println!("\n{}:\n{}", STEPS, input);
        flashes.to_string()
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"
                    5483143223
                    2745854711
                    5264556173
                    6141336146
                    6357385478
                    4167524645
                    2176841721
                    6882881134
                    4846848554
                    5283751526\
                "}.to_string(),
                output: "1656".to_string(),
            },
            TestCase {
                input: indoc! {"
                    11111
                    19991
                    19191
                    19991
                    11111\
                "}.to_string(),
                output: "259".to_string(),
            }
        ])
    }

    fn part2(&self, input: &str) -> String {
        let mut input = WhaleField::from_str(input).unwrap();

        fn is_non_zero(field: &WhaleField) -> bool {
            field.iter().any(|x| *x != 0)
        }

        let mut steps = 0usize;
        while is_non_zero(&input) {
            input.increase();
            input.flash();
            steps += 1;
        }

        steps.to_string()
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"
                    5483143223
                    2745854711
                    5264556173
                    6141336146
                    6357385478
                    4167524645
                    2176841721
                    6882881134
                    4846848554
                    5283751526\
                "}.to_string(),
                output: "195".to_string(),
            },
            TestCase {
                input: indoc! {"
                    11111
                    19991
                    19191
                    19991
                    11111\
                "}.to_string(),
                output: "6".to_string(),
            }
        ])
    }
}
