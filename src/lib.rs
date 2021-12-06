use std::fmt::Display;

pub trait Solution {
    fn part1(&self, _input: &str) -> String {
        "the part 1 is not implemented currently!".to_string()
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        return Box::from([]);
    }


    fn part2(&self, _input: &str) -> String {
        "the part 2 is not implemented currently!".to_string()
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        return Box::from([]);
    }
}

pub fn parse_lines<'a, F: 'a + Fn(&str) -> Result<R, T>, R, T: Display>(input: &'a str, f: F) -> Box<dyn Iterator<Item = R> + 'a> {
    Box::new(input.lines().map(move |it| f(it).unwrap_or_else(|e| panic!("Could not parse input: {}\n{}", e, input))))
}

pub struct TestCase {
    pub input: String,
    pub output: String,
}
