use std::fs::read_to_string;

use aoc2021::{Solution, TestCase};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod field;
mod util;

fn get_solution(day: u8) -> Box<dyn Solution> {
    match day {
        01 => Box::new(day01::Solution),
        02 => Box::new(day02::Solution),
        03 => Box::new(day03::Solution),
        04 => Box::new(day04::Solution),
        05 => Box::new(day05::Solution),
        06 => Box::new(day06::Solution),
        07 => Box::new(day07::Solution),
        08 => Box::new(day08::Solution),
        09 => Box::new(day09::Solution),
        10 => Box::new(day10::Solution),
        11 => Box::new(day11::Solution),
        12 => Box::new(day12::Solution),
        13 => Box::new(day13::Solution),
        14 => Box::new(day14::Solution),
        _ => todo!("Solutions for day {} is not yet implemented", day)
    }
}

fn main() {
    for current_day in [14] {
        let solution = get_solution(current_day);

        let input = read_to_string(format!("data/day{:0>2}", current_day)).unwrap_or("".to_string());

        println!("-------------   Day {:0>2}   -------------", current_day);
        run_part(1, |input| solution.part1(input), &*solution.test_part1(), &input);
        run_part(2, |input| solution.part2(input), &*solution.test_part2(), &input);
    }
}

fn run_part<F: Fn(&str) -> String>(part_id: u8, part: F, tests: &[TestCase], puzzle_input: &str) {
    println!("------------- Solution {} -------------", part_id);
    let mut i = 0;
    for test in tests {
        i += 1;
        println!("-------------  Test {:0>2} -------------", i);
        println!("{}\n-------------", test.input);
        let out = part(&*test.input);
        if out != test.output {
            println!("Test FAILED: {} != {}", test.output, out);
        } else {
            println!("success: {}", out);
        }
    }
    println!("------------- Real Input -------------");
    let sol1 = part(&puzzle_input);
    println!("{}\n\n", sol1);
}
