use aoc2021::TestCase;

pub(crate) struct Solution;

impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        let mut horizontal = 0;
        let mut depth = 0;

        for operation in input.lines() {
            let (op, n) = Self::extract_operation(operation);
            match op {
                "forward" => horizontal += n,
                "down" => depth += n,
                "up" => depth -= n,
                _ => panic!("unknown operation {}", op)
            }
        }

        format!("position: horizontal={}; depth={}, solution={}", horizontal, depth, horizontal * depth)
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2".to_string(),
                output: "position: horizontal=15; depth=10, solution=150".to_string(),
            }
        ])
    }

    fn part2(&self, input: &str) -> String {
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;

        for operation in input.lines() {
            let (op, n) = Self::extract_operation(operation);
            match op {
                "forward" => {
                    horizontal += n;
                    depth += aim * n;
                }
                "down" => aim += n,
                "up" => aim -= n,
                _ => panic!("unknown operation {}", op)
            }
        }

        format!("position: horizontal={}; depth={}, solution={}", horizontal, depth, horizontal * depth)
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2".to_string(),
                output: "position: horizontal=15; depth=60, solution=900".to_string(),
            }
        ])
    }
}

impl Solution {
    fn extract_operation(operation: &str) -> (&str, i32) {
        let mut partition = operation.split(' ');
        let operation_name = partition.nth(0).unwrap_or_else(|| panic!("Could not get operation name from the input {}", operation));
        let operation_count = partition.nth(0)
            .unwrap_or_else(|| panic!("Could not get the operation count for {}", operation_name))
            .parse::<i32>()
            .unwrap_or_else(|e| panic!("Could not parse the operation count for {}; e: {}", operation_name, e));

        return (
            operation_name,
            operation_count
        );
    }
}
