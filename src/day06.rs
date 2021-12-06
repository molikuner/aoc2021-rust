use aoc2021::{TestCase};

pub(crate) struct Solution;

impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        let input: Vec<u8> = input.split(',').map(|fish| fish.parse().unwrap()).collect();

        format!("18: {}; 80: {}", Self::fish_count(&input, 18), Self::fish_count(&input, 80))
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: "3,4,3,1,2".to_string(),
                output: "18: 26; 80: 5934".to_string()
            }
        ])
    }

    fn part2(&self, input: &str) -> String {
        let input: Vec<u8> = input.split(',').map(|fish| fish.parse().unwrap()).collect();

        format!("256: {}", Self::fish_count(&input, 256))
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: "3,4,3,1,2".to_string(),
                output: "256: 26984457539".to_string()
            }
        ])
    }
}

const FISH_RESET_COUNTER: usize = 6;
const NEW_FISH_COUNTER: usize = 8;
impl Solution {
    fn fish_count(fish: &Vec<u8>, cycles: usize) -> String {
        let mut fish_counts = [0usize; NEW_FISH_COUNTER + 1];
        for fish in fish {
            fish_counts[*fish as usize] += 1;
        }

        for _ in 0..cycles {
            let new_fish_count = fish_counts[0];
            for i in 1..fish_counts.len() {
                fish_counts[i - 1] = fish_counts[i];
            }
            fish_counts[FISH_RESET_COUNTER] += new_fish_count;
            fish_counts[NEW_FISH_COUNTER] = new_fish_count;

        }
        fish_counts.iter().sum::<usize>().to_string()
    }
}
