use std::collections::{HashMap, HashSet};
use std::iter::once;

use indoc::indoc;

use aoc2021::TestCase;

pub(crate) struct Solution;

fn parse(input: &str) -> (HashMap<&str, HashSet<&str>>, HashSet<&str>) {
    let input = input.lines()
        .map(|l| l.split_once("-").unwrap())
        .collect::<Vec<_>>();

    let mut connections = HashMap::<&str, HashSet<&str>>::new();
    for (a, b) in input.iter() {
        connections.entry(a)
            .or_insert(HashSet::new())
            .insert(b);
        connections.entry(b)
            .or_insert(HashSet::new())
            .insert(a);
    }
    let caves = input.into_iter()
        .flat_map(|(a, b)| once(a).chain(once(b)))
        .collect::<HashSet<_>>();

    (connections, caves)
}

impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        let (connections, mut caves) = parse(input);

        fn path_count<'a>(position: &'a str, caves: &mut HashSet<&'a str>, connections: &HashMap<&str, HashSet<&'a str>>) -> usize {
            if position == "end" {
                return 1;
            } else {
                if position.chars().nth(0).unwrap().is_lowercase() {
                    caves.remove(position);
                }
                let path_count = connections[position].iter().fold(0, |paths, new_position| {
                    if caves.contains(*new_position) {
                        paths + path_count(new_position, caves, connections)
                    } else {
                        paths
                    }
                });
                caves.insert(position);
                path_count
            }
        }

        path_count("start", &mut caves, &connections).to_string()
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"
                    start-A
                    start-b
                    A-c
                    A-b
                    b-d
                    A-end
                    b-end\
                "}.to_string(),
                output: "10".to_string(),
            },
            TestCase {
                input: indoc! {"
                    dc-end
                    HN-start
                    start-kj
                    dc-start
                    dc-HN
                    LN-dc
                    HN-end
                    kj-sa
                    kj-HN
                    kj-dc\
                "}.to_string(),
                output: "19".to_string(),
            },
            TestCase {
                input: indoc! {"
                    fs-end
                    he-DX
                    fs-he
                    start-DX
                    pj-DX
                    end-zg
                    zg-sl
                    zg-pj
                    pj-he
                    RW-he
                    fs-DX
                    pj-RW
                    zg-RW
                    start-pj
                    he-WI
                    zg-he
                    pj-fs
                    start-RW\
                "}.to_string(),
                output: "226".to_string(),
            }
        ])
    }

    fn part2(&self, input: &str) -> String {
        let (connections, mut caves) = parse(input);

        fn path_count<'a>(position: &'a str, caves: &mut HashSet<&'a str>, connections: &HashMap<&str, HashSet<&'a str>>, allow_small_twice: bool, duplicate: Option<&'a str>) -> usize {
            if position == "end" {
                duplicate.map_or(1, |_| 0)
            } else {
                if position.chars().nth(0).unwrap().is_lowercase() {
                    caves.remove(position);
                }
                let path_count = connections[position].iter().fold(0, |mut paths, new_position| {
                    if caves.contains(*new_position) || duplicate.map_or(false, |it| it == *new_position) {
                        paths += path_count(new_position, caves, connections, allow_small_twice, duplicate.filter(|it| *it != *new_position));
                        if allow_small_twice && new_position.chars().nth(0).unwrap().is_lowercase() {
                            paths += path_count(new_position, caves, connections, false, Some(new_position))
                        }
                    }
                    paths
                });
                caves.insert(position);
                path_count
            }
        }

        path_count("start", &mut caves, &connections, true, None).to_string()
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"
                    start-A
                    start-b
                    A-c
                    A-b
                    b-d
                    A-end
                    b-end\
                "}.to_string(),
                output: "36".to_string(),
            },
            TestCase {
                input: indoc! {"
                    dc-end
                    HN-start
                    start-kj
                    dc-start
                    dc-HN
                    LN-dc
                    HN-end
                    kj-sa
                    kj-HN
                    kj-dc\
                "}.to_string(),
                output: "103".to_string(),
            },
            TestCase {
                input: indoc! {"
                    fs-end
                    he-DX
                    fs-he
                    start-DX
                    pj-DX
                    end-zg
                    zg-sl
                    zg-pj
                    pj-he
                    RW-he
                    fs-DX
                    pj-RW
                    zg-RW
                    start-pj
                    he-WI
                    zg-he
                    pj-fs
                    start-RW\
                "}.to_string(),
                output: "3509".to_string(),
            }
        ])
    }
}
