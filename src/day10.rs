use std::collections::VecDeque;
use indoc::indoc;

use aoc2021::TestCase;

pub(crate) struct Solution;

impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        fn error_value(bracket: char) -> u32 {
            match bracket {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => panic!("Unknown bracket: {}", bracket)
            }
        }

        fn line_error_value(line: &str) -> Option<u32> {
            let mut required_chars = Vec::<char>::new();

            for symbol in line.chars() {
                let last = required_chars.last();
                match symbol {
                    '(' => required_chars.push(')'),
                    '[' => required_chars.push(']'),
                    '{' => required_chars.push('}'),
                    '<' => required_chars.push('>'),
                    _ => {
                        if last.is_some() && *last.unwrap() == symbol {
                            required_chars.pop();
                        } else {
                            return Some(error_value(symbol));
                        }
                    }
                }
            }

            None
        }

        input.lines()
            .filter_map(|line| line_error_value(line))
            .sum::<u32>()
            .to_string()
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
                output: "1197".to_string(),
            },
            TestCase {
                input: "[[<[([]))<([[{}[[()]]]".to_string(),
                output: "3".to_string(),
            },
            TestCase {
                input: "[{[{({}]{}}([{[{{{}}([]".to_string(),
                output: "57".to_string(),
            },
            TestCase {
                input: "[<(<(<(<{}))><([]([]()".to_string(),
                output: "3".to_string(),
            },
            TestCase {
                input: "<{([([[(<>()){}]>(<<{{".to_string(),
                output: "25137".to_string(),
            },
            TestCase {
                input: indoc! {"
                    [({(<(())[]>[[{[]{<()<>>
                    [(()[<>])]({[<{<<[]>>(
                    {([(<{}[<>[]}>{[]{[(<()>
                    (((({<>}<{<{<>}{[]{[]{}
                    [[<[([]))<([[{}[[()]]]
                    [{[{({}]{}}([{[{{{}}([]
                    {<[[]]>}<{[{[{[]{()[[[]
                    [<(<(<(<{}))><([]([]()
                    <{([([[(<>()){}]>(<<{{
                    <{([{{}}[<[[[<>{}]]]>[]]\
                "}.to_string(),
                output: "26397".to_string(),
            }
        ])
    }

    fn part2(&self, input: &str) -> String {
        fn error_value(bracket: char) -> u32 {
            match bracket {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("Unknown bracket: {}", bracket)
            }
        }

        fn line_error_value(line: &str) -> Option<u64> {
            let mut required_chars = VecDeque::<char>::new();

            for symbol in line.chars() {
                let last = required_chars.front();
                match symbol {
                    '(' => required_chars.push_front(')'),
                    '[' => required_chars.push_front(']'),
                    '{' => required_chars.push_front('}'),
                    '<' => required_chars.push_front('>'),
                    _ => {
                        if last.is_some() && *last.unwrap() == symbol {
                            required_chars.pop_front();
                        } else {
                            return None;
                        }
                    }
                }
            }

            return if required_chars.is_empty() {
                None
            } else {
                let error = required_chars.iter().fold(
                    0,
                    |acc, &it| acc * 5 + error_value(it) as u64,
                );
                Some(error)
            };
        }

        let mut errors = input.lines()
            .filter_map(|line| line_error_value(line))
            .collect::<Vec<_>>();

        errors.sort_unstable();

        errors[errors.len() / 2].to_string()
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: "[({(<(())[]>[[{[]{<()<>>".to_string(),
                output: "288957".to_string(),
            },
            TestCase {
                input: "[(()[<>])]({[<{<<[]>>(".to_string(),
                output: "5566".to_string(),
            },
            TestCase {
                input: "(((({<>}<{<{<>}{[]{[]{}".to_string(),
                output: "1480781".to_string(),
            },
            TestCase {
                input: "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
                output: "995444".to_string(),
            },
            TestCase {
                input: "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
                output: "294".to_string(),
            },
            TestCase {
                input: indoc! {"
                    [({(<(())[]>[[{[]{<()<>>
                    [(()[<>])]({[<{<<[]>>(
                    {([(<{}[<>[]}>{[]{[(<()>
                    (((({<>}<{<{<>}{[]{[]{}
                    [[<[([]))<([[{}[[()]]]
                    [{[{({}]{}}([{[{{{}}([]
                    {<[[]]>}<{[{[{[]{()[[[]
                    [<(<(<(<{}))><([]([]()
                    <{([([[(<>()){}]>(<<{{
                    <{([{{}}[<[[[<>{}]]]>[]]\
                "}.to_string(),
                output: "288957".to_string(),
            }
        ])
    }
}
