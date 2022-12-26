use std::collections::VecDeque;

use hashbrown::HashSet;
use itertools::Itertools;

use crate::day::Day;

const DIRS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

pub struct Day18;

impl<'a> Day<'a> for Day18 {
    const DAY: usize = 18;
    type Input = HashSet<(i32, i32, i32)>;
    type ProcessedInput = HashSet<(i32, i32, i32)>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| {
                let (x, yz) = line.split_once(',').unwrap();
                let (y, z) = yz.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = input
            .iter()
            .map(|p| {
                DIRS.iter()
                    .filter(|d| !input.contains(&(p.0 + d.0, p.1 + d.1, p.2 + d.2)))
                    .count()
            })
            .sum::<usize>()
            .to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let (x_min, x_max) = input.iter().map(|p| p.0).minmax().into_option().unwrap();
        let (y_min, y_max) = input.iter().map(|p| p.1).minmax().into_option().unwrap();
        let (z_min, z_max) = input.iter().map(|p| p.2).minmax().into_option().unwrap();
        let low = (x_min - 1, y_min - 1, z_min - 1);
        let high = (x_max + 1, y_max + 1, z_max + 1);
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        let mut faces = 0;
        queue.push_front(low);
        seen.insert(low);
        while let Some(p) = queue.pop_back() {
            for d in DIRS.iter() {
                let n = (p.0 + d.0, p.1 + d.1, p.2 + d.2);
                if n.0 < low.0 || n.1 < low.1 || n.2 < low.2 {
                    continue;
                }
                if n.0 > high.0 || n.1 > high.1 || n.2 > high.2 {
                    continue;
                }
                if input.contains(&n) {
                    faces += 1;
                } else if !seen.contains(&n) {
                    queue.push_front(n);
                    seen.insert(n);
                }
            }
        }
        faces.to_string()
    }
}

#[cfg(test)]
mod test_day18 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    "};

    #[test]
    fn test_day18_examples() {
        let input = Day18::parse(EXAMPLE);
        let (input, part1) = Day18::solve_part1(input);
        let part2 = Day18::solve_part2(input);
        assert_eq!(part1, "64");
        assert_eq!(part2, "58");
    }
}

bench_day!(18);
