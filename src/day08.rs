use std::collections::{HashSet};

use indoc::indoc;

use aoc2021::TestCase;

pub(crate) struct Solution;

impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        let x = input.split('\n').map(|l| {
            let (_, output) = l.split_once(" | ").unwrap();
            output.split(' ').filter(|n| [2, 3, 4, 7].iter().any(|&x| x == n.len())).count()
        });

        x.sum::<usize>().to_string()
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
                output: "2".to_string(),
            },
            TestCase {
                input: "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
                output: "3".to_string(),
            },
            TestCase {
                input: "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
                output: "3".to_string(),
            },
            TestCase {
                input: "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
                output: "1".to_string(),
            },
            TestCase {
                input: "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
                output: "3".to_string(),
            },
            TestCase {
                input: "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
                output: "4".to_string(),
            },
            TestCase {
                input: "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
                output: "3".to_string(),
            },
            TestCase {
                input: "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
                output: "1".to_string(),
            },
            TestCase {
                input: "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
                output: "4".to_string(),
            },
            TestCase {
                input: "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
                output: "2".to_string(),
            },
            TestCase {
                input: indoc! {"
                    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
                    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
                    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
                    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
                    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
                    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
                    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
                    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
                    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
                    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\
                "}.to_string(),
                output: "26".to_string(),
            }
        ])
    }

    fn part2(&self, input: &str) -> String {
        let x = input.split('\n').map(|l| {
            let (input, output) = l.split_once(" | ").unwrap();

            let input: Vec<_> = input.split(' ').collect();
            let one = *input.iter().find(|x| x.len() == 2).unwrap();
            let four = *input.iter().find(|x| x.len() == 4).unwrap();
            let seven = *input.iter().find(|x| x.len() == 3).unwrap();
            let eight = *input.iter().find(|x| x.len() == 7).unwrap();

            let nine = *input.iter().find(|x| x.len() == 6 && four.chars().all(|f| x.chars().any(|n| f == n))).unwrap();
            let zero = *input.iter().find(|x| x.len() == 6 && seven.chars().all(|s| x.chars().any(|z| z == s)) && **x != nine).unwrap();
            let six = *input.iter().find(|x| x.len() == 6 && **x != nine && **x != zero).unwrap();

            let three = *input.iter().find(|x| x.len() == 5 && one.chars().all(|o| x.chars().any(|t| o == t))).unwrap();
            let fife = *input.iter().find(|x| x.len() == 5 && four.chars().filter(|&f| !x.chars().any(|n| n == f)).count() == 1 && **x != three).unwrap();
            let two = *input.iter().find(|x| x.len() == 5 && **x != three && **x != fife).unwrap();

            //  aaaa    ....    aaaa    aaaa    ....
            // b    c  .    c  .    c  .    c  b    c
            // b    c  .    c  .    c  .    c  b    c
            //  ....    ....    dddd    dddd    dddd
            // e    f  .    f  e    .  .    f  .    f
            // e    f  .    f  e    .  .    f  .    f
            //  gggg    ....    gggg    gggg    ....
            //
            //  aaaa    aaaa    aaaa    aaaa    aaaa
            // b    .  b    .  .    c  b    c  b    c
            // b    .  b    .  .    c  b    c  b    c
            //  dddd    dddd    ....    dddd    dddd
            // .    f  e    f  .    f  e    f  .    f
            // .    f  e    f  .    f  e    f  .    f
            //  gggg    gggg    ....    gggg    gggg

            let numbers = [
                zero, one, two, three, four, fife, six, seven, eight, nine
            ].map(|n| HashSet::from_iter(n.chars()));

            output.split(' ')
                .map(|f| HashSet::<char>::from_iter(f.chars()))
                .map(|cs| numbers.iter().position(|ncs| cs == *ncs).unwrap())
                .fold(0, |acc, x| acc * 10 + x)
        });

        x.sum::<usize>().to_string()
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf".to_string(),
                output: "5353".to_string(),
            },
            TestCase {
                input: "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
                output: "8394".to_string(),
            },
            TestCase {
                input: "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
                output: "9781".to_string(),
            },
            TestCase {
                input: "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
                output: "1197".to_string(),
            },
            TestCase {
                input: "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
                output: "9361".to_string(),
            },
            TestCase {
                input: "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
                output: "4873".to_string(),
            },
            TestCase {
                input: "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
                output: "8418".to_string(),
            },
            TestCase {
                input: "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
                output: "4548".to_string(),
            },
            TestCase {
                input: "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
                output: "1625".to_string(),
            },
            TestCase {
                input: "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
                output: "8717".to_string(),
            },
            TestCase {
                input: "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
                output: "4315".to_string(),
            },
            TestCase {
                input: indoc! {"
                    be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
                    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
                    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
                    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
                    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
                    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
                    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
                    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
                    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
                    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\
                "}.to_string(),
                output: "61229".to_string(),
            },
        ])
    }
}
