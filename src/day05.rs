use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::iter::empty;
use std::str::FromStr;

use indoc::indoc;

use aoc2021::{parse_lines, TestCase};

type CoordinateComponent = u16;

#[derive(PartialEq, Eq, Hash)]
struct Coordinate {
    x: CoordinateComponent,
    y: CoordinateComponent,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((x, y)) = s.split_once(",") {
            let x = x.parse::<CoordinateComponent>().map_err(|e| e.to_string())?;
            let y = y.parse::<CoordinateComponent>().map_err(|e| e.to_string())?;
            Ok(Coordinate { x, y })
        } else {
            Err("Could not split coordinate components".to_string())
        }
    }
}

struct Line {
    start: Coordinate,
    end: Coordinate,
}

impl Line {
    fn straight_coordinates(&self) -> Box<dyn Iterator<Item=Coordinate> + '_> {
        if self.start.x == self.end.x {
            let min_y = min(self.start.y, self.end.y);
            let max_y = max(self.start.y, self.end.y);
            Box::new((min_y..=max_y).map(|y| Coordinate { x: self.start.x, y }))
        } else if self.start.y == self.end.y {
            let min_x = min(self.start.x, self.end.x);
            let max_x = max(self.start.x, self.end.x);
            Box::new((min_x..=max_x).map(|x| Coordinate { x, y: self.start.y }))
        } else {
            Box::new(empty())
        }
    }

    fn coordinates(&self) -> Box<dyn Iterator<Item=Coordinate> + '_> {
        if self.start.x == self.end.x || self.start.y == self.end.y {
            self.straight_coordinates()
        } else {
            let min_x = min(self.start.x, self.end.x);
            let max_x = max(self.start.x, self.end.x);
            let min_y = min(self.start.y, self.end.y);
            let max_y = max(self.start.y, self.end.y);
            if max_x - min_x == max_y - min_y {
                let x: Vec<_> = match self.start.x.cmp(&self.end.x) {
                    Ordering::Less => (self.start.x..=self.end.x).collect(),
                    Ordering::Equal => panic!("x should not be equal at this point"),
                    Ordering::Greater => (self.end.x..=self.start.x).rev().collect()
                };
                let y: Vec<_> = match self.start.y.cmp(&self.end.y) {
                    Ordering::Less => (self.start.y..=self.end.y).collect(),
                    Ordering::Equal => panic!("y should not be equal at this point"),
                    Ordering::Greater => (self.end.y..=self.start.y).rev().collect()
                };
                Box::new(x.into_iter().zip(y).map(|(x, y)| Coordinate { x, y }))
            } else {
                panic!("Can not generate coordinates for lines that are not 0, 45 or 90 degree to the x-axis: {}", self)
            }
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = s.split_once(" -> ") {
            let start = Coordinate::from_str(start)?;
            let end = Coordinate::from_str(end)?;
            Ok(Line { start, end })
        } else {
            Err("Could not split coordinates for line".to_string())
        }
    }
}

pub(crate) struct Solution;

impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        Self::dangerous_paces_count(input, |line| line.straight_coordinates()).to_string()
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"
                    0,9 -> 5,9
                    8,0 -> 0,8
                    9,4 -> 3,4
                    2,2 -> 2,1
                    7,0 -> 7,4
                    6,4 -> 2,0
                    0,9 -> 2,9
                    3,4 -> 1,4
                    0,0 -> 8,8
                    5,5 -> 8,2
                "}.to_string(),
                output: "5".to_string(),
            }
        ])
    }

    fn part2(&self, input: &str) -> String {
        Self::dangerous_paces_count(input, |line| line.coordinates()).to_string()
    }


    fn test_part2(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"
                    0,9 -> 5,9
                    8,0 -> 0,8
                    9,4 -> 3,4
                    2,2 -> 2,1
                    7,0 -> 7,4
                    6,4 -> 2,0
                    0,9 -> 2,9
                    3,4 -> 1,4
                    0,0 -> 8,8
                    5,5 -> 8,2
                "}.to_string(),
                output: "12".to_string(),
            }
        ])
    }
}

impl Solution {
    fn dangerous_paces_count(input: &str, coordinates_of_line: fn(&Line) -> Box<dyn Iterator<Item=Coordinate> + '_>) -> usize {
        let lines: Vec<_> = parse_lines(input, |l| Line::from_str(l)).collect();

        let mut covered_coordinates = HashMap::<Coordinate, u8>::new();

        for coordinate in lines.iter().flat_map(coordinates_of_line) {
            covered_coordinates.entry(coordinate)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let dangerous_coordinate_count = covered_coordinates.iter().filter(|(_coordinate, count)| **count > 1).count();
        dangerous_coordinate_count
    }
}
