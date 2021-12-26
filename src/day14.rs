use std::collections::{HashMap};
use std::iter::once;

use indoc::indoc;

use aoc2021::TestCase;

pub(crate) struct Solution;


impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        const STEPS: usize = 10;

        let (template, rules) = input.split_once("\n\n").unwrap();
        let rules = rules.lines().map(|l| l.split_once(" -> ").unwrap()).collect::<HashMap<_, _>>();

        fn insert(template: &str, rules: &HashMap<&str, &str>) -> String {
            let mut res = String::new();
            for (c, nc) in template.chars().zip(template.chars().skip(1).chain(once('#'))) {
                res.push(c);
                if let Some(addition) = rules.get(format!("{}{}", c, nc).as_str()) {
                    res.push_str(addition)
                }
            }
            res
        }

        let mut res = template.to_string();
        for _ in 0..STEPS {
            res = insert(res.as_ref(), &rules);
        }
        let mut occurrences = HashMap::<char, usize>::new();
        for c in res.chars() {
            *occurrences.entry(c).or_insert(0) += 1;
        }

        let (_, max_count) = occurrences.iter().max_by_key(|(_, count)| **count).unwrap();
        let (_, min_count) = occurrences.iter().min_by_key(|(_, count)| **count).unwrap();

        (max_count - min_count).to_string()
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"
                    NNCB

                    CH -> B
                    HH -> N
                    CB -> H
                    NH -> C
                    HB -> C
                    HC -> B
                    HN -> C
                    NN -> C
                    BH -> H
                    NC -> B
                    NB -> B
                    BN -> B
                    BB -> N
                    BC -> B
                    CC -> N
                    CN -> C\
                "}.to_string(),
                output: "1588".to_string(),
            }
        ])
    }

    fn part2(&self, input: &str) -> String {
        const STEPS: usize = 40;

        let (template, rules) = input.split_once("\n\n").unwrap();
        let rules = rules.lines().map(|l| l.split_once(" -> ").unwrap()).map(|(input, output)|
            ((input.chars().nth(0).unwrap(), input.chars().nth(1).unwrap()), output.chars().nth(0).unwrap())
        ).collect::<HashMap<_, _>>();

        fn insert(template: &mut HashMap<(char, char), usize>, rules: &HashMap<(char, char), char>) {
            let orig_template = template.clone();
            template.clear();
            for (key, count) in orig_template {
                if let Some(addition) = rules.get(&key) {
                    for key in [(key.0, *addition), (*addition, key.1)] {
                        *template.entry(key).or_insert(0) += count
                    }
                } else {
                    template.insert(key, count);
                }
            }
        }

        let mut res = HashMap::<(char, char), usize>::new();
        for key in template.chars().zip(template.chars().skip(1).chain(once('#'))) {
            *res.entry(key).or_insert(0) += 1;
        }
        for _ in 0..STEPS {
            insert(&mut res, &rules);
        }

        let mut occurrences = HashMap::<char, usize>::new();
        for ((c, _), count) in res {
            *occurrences.entry(c).or_insert(0) += count;
        }

        let (_, max_count) = occurrences.iter().max_by_key(|(_, count)| **count).unwrap();
        let (_, min_count) = occurrences.iter().min_by_key(|(_, count)| **count).unwrap();

        (max_count - min_count).to_string()
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"
                    NNCB

                    CH -> B
                    HH -> N
                    CB -> H
                    NH -> C
                    HB -> C
                    HC -> B
                    HN -> C
                    NN -> C
                    BH -> H
                    NC -> B
                    NB -> B
                    BN -> B
                    BB -> N
                    BC -> B
                    CC -> N
                    CN -> C\
                "}.to_string(),
                output: "2188189693529".to_string(),
            }
        ])
    }
}
