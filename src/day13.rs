use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use indoc::indoc;

use aoc2021::TestCase;

pub(crate) struct Solution;

struct TransparentField(HashSet<(usize, usize)>);

impl TransparentField {
    fn fold_horizontal(&mut self, axis: usize) {
        let original_field = self.0.clone();
        self.0.clear();
        for (x, y) in original_field {
            self.0.insert((x, if y > axis { axis - (y - axis) } else { y }));
        }
    }

    fn fold_vertical(&mut self, axis: usize) {
        let original_field = self.0.clone();
        self.0.clear();
        for (x, y) in original_field {
            self.0.insert((if x > axis { axis - (x - axis) } else { x }, y));
        }
    }
}

impl FromStr for TransparentField {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TransparentField(s.lines().map(|p| {
            let (x, y) = p.split_once(',').unwrap();
            let x = usize::from_str(x).unwrap();
            let y = usize::from_str(y).unwrap();
            (x, y)
        }).collect()))
    }
}

impl Display for TransparentField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (_, max_y) = self.0.iter().max_by_key(|(_, y)| y).unwrap_or(&(0, 0));
        let (max_x, _) = self.0.iter().max_by_key(|(x, _)| x).unwrap_or(&(0, 0));

        (0..=*max_y).fold(Ok(()), |res, y|
            res
                .and_then(|_| write!(f, "\n"))
                .and_then(|_| (0..=*max_x).fold(Ok(()), |res, x|
                    res.and_then(|_| write!(f, "{}", if self.0.contains(&(x, y)) { "#" } else { "." }))
                )),
        )
    }
}

struct Fold {
    horizontal: bool,
    axis: usize,
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((s, axis)) = s.split_once('=') {
            let axis = usize::from_str(axis).map_err(|e| e.to_string())?;
            let horizontal = match s.chars().last() {
                Some('x') => false,
                Some('y') => true,
                _ => return Err(format!("Could not parse direction from '{}'", s))
            };
            Ok(
                Fold {
                    horizontal,
                    axis,
                }
            )
        } else {
            Err(format!("Could not get values from '{}'", s))
        }
    }
}

impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        let (field, folds) = input.split_once("\n\n").unwrap();

        let folds = folds.lines().map(|l| Fold::from_str(l).unwrap()).take(1).collect::<Vec<_>>();
        let mut field = TransparentField::from_str(field).unwrap();

        for fold in folds {
            if fold.horizontal {
                field.fold_horizontal(fold.axis)
            } else {
                field.fold_vertical(fold.axis)
            }
        }

        field.0.len().to_string()
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"
                    6,10
                    0,14
                    9,10
                    0,3
                    10,4
                    4,11
                    6,0
                    6,12
                    4,1
                    0,13
                    10,12
                    3,4
                    3,0
                    8,4
                    1,10
                    2,14
                    8,10
                    9,0

                    fold along y=7
                    fold along x=5\
                "}.to_string(),
                output: "17".to_string(),
            }
        ])
    }

    fn part2(&self, input: &str) -> String {
        let (field, folds) = input.split_once("\n\n").unwrap();

        let folds = folds.lines().map(|l| Fold::from_str(l).unwrap()).collect::<Vec<_>>();
        let mut field = TransparentField::from_str(field).unwrap();

        for fold in folds {
            if fold.horizontal {
                field.fold_horizontal(fold.axis)
            } else {
                field.fold_vertical(fold.axis)
            }
        }

        format!("{}", field)
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"
                    6,10
                    0,14
                    9,10
                    0,3
                    10,4
                    4,11
                    6,0
                    6,12
                    4,1
                    0,13
                    10,12
                    3,4
                    3,0
                    8,4
                    1,10
                    2,14
                    8,10
                    9,0

                    fold along y=7
                    fold along x=5\
                "}.to_string(),
                output: indoc! {"
                    #####
                    #...#
                    #...#
                    #...#
                    #####
                "}.to_string(),
            }
        ])
    }
}
