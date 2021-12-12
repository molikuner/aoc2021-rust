use indoc::indoc;
use aoc2021::TestCase;

pub(crate) struct Solution;

impl aoc2021::Solution for Solution {
    fn part1(&self, input: &str) -> String {
        let input: Vec<Vec<u8>> = input.lines().map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect()).collect();

        Self::low_points(&input)
            .map(|(x, y)| input[y][x] as u16 + 1)
            .sum::<u16>()
            .to_string()
    }

    fn test_part1(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"\
                    2199943210
                    3987894921
                    9856789892
                    8767896789
                    9899965678\
                "}.to_string(),
                output: "15".to_string()
            }
        ])
    }

    fn part2(&self, input: &str) -> String {
        let input: Vec<Vec<u8>> = input.lines().map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect()).collect();

        fn basin_size(input: &Vec<Vec<u8>>, pos: &(usize, usize)) -> usize {
            let mut to_do = vec![*pos];
            let mut checked = Vec::<(usize, usize)>::new();
            let mut found = Vec::<(usize, usize)>::new();

            while let Some((x, y)) = to_do.pop() {
                if checked.iter().any(|&(ox, oy)| ox == x && oy == y) {
                    continue
                } else {
                    checked.push((x, y))
                }
                if input[y][x] < 9 {
                    found.push((x, y));
                }

                let mut may_check = Vec::<(usize, usize)>::new();

                if let Some(x) = x.checked_sub(1) {
                    may_check.push((x, y))
                }
                if let Some(y) = y.checked_sub(1) {
                    may_check.push((x, y))
                }
                if let Some(x) = x.checked_add(1) {
                    may_check.push((x, y))
                }
                if let Some(y) = y.checked_add(1) {
                    may_check.push((x, y))
                }

                for (x, y) in may_check {
                    if let Some(value) = input.get(y).and_then(|row| row.get(x)) {
                        if *value < 9 {
                            to_do.push((x, y))
                        }
                    }
                }
            }

            found.len()
        }

        let mut basins: Vec<_> =
            Self::low_points(&input)
                .map(|pos| basin_size(&input, &pos))
                .collect();

        basins.sort_unstable();

        basins.as_slice()[basins.len().saturating_sub(3)..]
            .iter()
            .fold(1, |acc, &i| acc * i)
            .to_string()
    }

    fn test_part2(&self) -> Box<[TestCase]> {
        Box::from([
            TestCase {
                input: indoc! {"\
                    2199943210
                    3987894921
                    9856789892
                    8767896789
                    9899965678\
                "}.to_string(),
                output: "1134".to_string()
            }
        ])
    }
}

impl Solution {
    fn low_points(input: &Vec<Vec<u8>>) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..input.len()).flat_map(|y| (0..input[y].len()).map(move |x| (x, y))).filter(|&(x, y)| {
            let this = input[y][x];
            let top = y.checked_sub(1).and_then(|y| input.get(y)).map(|row| row[x]).map(|value| value > this).unwrap_or(true);
            let right = x.checked_add(1).and_then(|x| input[y].get(x)).map(|&value| value > this).unwrap_or(true);
            let bottom = y.checked_add(1).and_then(|y| input.get(y)).map(|row| row[x]).map(|value| value > this).unwrap_or(true);
            let left = x.checked_sub(1).and_then(|x| input[y].get(x)).map(|&value| value > this).unwrap_or(true);

            top && right && bottom && left
        })
    }
}
