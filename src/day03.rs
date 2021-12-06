use std::cmp::Ordering;
use std::cmp::Ordering::Greater;

use aoc2021::{parse_lines, TestCase};

pub(crate) struct Solution;

type Word = u32;

impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        let bit_count = input.find('\n').unwrap();
        let input: Vec<_> = parse_lines(input, |l| Word::from_str_radix(l, 2)).collect();
        let value_count = input.len();

        let mut gamma: Word = 0;
        let mut epsilon: Word = 0;
        for i in 0..bit_count {
            let mask: Word = 1 << (bit_count - 1 - i);
            let one_count = Self::matching_numbers(&input, mask, false).count();
            if value_count - one_count < one_count {
                gamma ^= mask
            } else {
                epsilon ^= mask
            }
        }

        format!("gamma={}; epsilon={}; power={}", gamma, epsilon, gamma * epsilon)
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010".to_string(),
                output: "gamma=22; epsilon=9; power=198".to_string(),
            }
        ])
    }

    fn part2(&self, input: &str) -> String {
        let bit_count = input.find('\n').unwrap();
        let input: Vec<_> = parse_lines(input, |l| Word::from_str_radix(l, 2)).collect();

        let oxygen: Word = Self::current_reading(bit_count, input.clone(), |ones| ones != Greater)[0];
        let co2: Word = Self::current_reading(bit_count, input.clone(), |ones| ones == Greater)[0];

        format!("oxygen={}; co2={}; life_support={}", oxygen, co2, oxygen * co2)
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010".to_string(),
                output: "oxygen=23; co2=10; life_support=230".to_string(),
            }
        ])
    }
}

impl Solution {
    fn current_reading<F: Fn(Ordering) -> bool>(bit_count: usize, mut input: Vec<Word>, use_ones: F) -> Vec<Word> {
        for i in 0..bit_count {
            let mask = 1 << (bit_count - 1 - i);
            input = Self::matching_values(input, mask, &use_ones);
            if input.len() == 1 {
                return input;
            }
        }
        panic!("Could not find the best reading, current best readings are: {:?}", input)
    }

    fn matching_values<F: Fn(Ordering) -> bool>(input: Vec<Word>, mask: Word, use_ones: &F) -> Vec<Word> {
        let input_count = input.len();
        let ones = Self::matching_numbers(&input, mask, false).collect::<Vec<_>>();
        let ord = (input_count - ones.len()).cmp(&ones.len());

        if use_ones(ord) {
            ones
        } else {
            Self::matching_numbers(&input, mask, true).collect()
        }
    }

    fn matching_numbers<'a>(input: &'a Vec<Word>, mask: Word, zero: bool) -> Box<dyn Iterator<Item=Word> + 'a> {
        Box::new(input.iter().filter(move |x| (*x & mask != 0) != zero).map(|x| *x))
    }
}
