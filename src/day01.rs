use aoc2021::{parse_lines, TestCase};
use crate::util::iterable_windows::WindowExt;

pub(crate) struct Solution;

impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        parse_lines(input, |x| x.parse::<i32>())
            .window::<2>()
            .filter(|[a, b]| a < b)
            .count()
            .to_string()
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        return Box::from([
            TestCase {
                input: "199\n200\n208\n210\n200\n207\n240\n269\n260\n263".to_string(),
                output: "7".to_string()
            }
        ]);
    }

    fn part2(&self, input: &str) -> String {
        parse_lines(input, |x| x.parse::<i32>())
            .window::<3>()
            .map(|[a, b, c]| a + b + c)
            .window::<2>()
            .filter(|[a, b]| a < b)
            .count()
            .to_string()
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        return Box::from([
            TestCase {
                input: "199\n200\n208\n210\n200\n207\n240\n269\n260\n263".to_string(),
                output: "5".to_string()
            }
        ]);
    }
}
