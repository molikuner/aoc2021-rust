use indoc::indoc;
use aoc2021::TestCase;

pub(crate) struct Solution;

impl aoc2021::Solution for Solution {
    fn part1(&self, _input: &str) -> String {
        "the part 1 is not implemented currently!".to_string()
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {""}.to_string(),
                output: "".to_string()
            }
        ])
    }
}
