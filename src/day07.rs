use indoc::indoc;

use aoc2021::TestCase;

pub(crate) struct Solution;

impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        let mut input: Vec<u32> = input.split(',').map(|n| n.parse().unwrap()).collect();

        input.sort_unstable();

        let fuel_count = |target| input.iter().fold(0, |cur, &it|
            cur + it.max(target) - it.min(target),
        );

        let target = input[input.len() / 2];

        let fuel = fuel_count(target);

        fuel.to_string()
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"16,1,2,0,4,2,7,1,2,14"}.to_string(),
                output: "37".to_string(),
            }
        ])
    }

    fn part2(&self, input: &str) -> String {
        let input: Vec<u32> = input.split(',').map(|n| n.parse().unwrap()).collect();

        let fuel_count = |target| input.iter().fold(0, |cur, &it| {
            let distance = it.max(target) - it.min(target);

            cur + (distance + 1) * distance / 2
        });

        let target = input.iter().sum::<u32>() / input.len() as u32;

        let fuel = fuel_count(target).min(fuel_count(target + 1));

        fuel.to_string()
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"16,1,2,0,4,2,7,1,2,14"}.to_string(),
                output: "168".to_string(),
            }
        ])
    }
}
