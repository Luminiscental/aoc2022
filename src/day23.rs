use hashbrown::{HashMap, HashSet};

use crate::day::Day;

fn round(elves: &mut HashSet<(i32, i32)>, dirs: &mut [(i32, i32)]) -> bool {
    let mut propositions = HashMap::new();
    let mut moved = false;
    for elf in elves.iter().copied() {
        if [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .all(|(dx, dy)| !elves.contains(&(elf.0 + dx, elf.1 + dy)))
        {
            continue;
        }
        for dir in dirs.iter() {
            let checks = if dir.0 == 0 {
                [
                    (elf.0 - 1, elf.1 + dir.1),
                    (elf.0, elf.1 + dir.1),
                    (elf.0 + 1, elf.1 + dir.1),
                ]
            } else {
                [
                    (elf.0 + dir.0, elf.1 - 1),
                    (elf.0 + dir.0, elf.1),
                    (elf.0 + dir.0, elf.1 + 1),
                ]
            };
            if checks.iter().all(|p| !elves.contains(p)) {
                propositions
                    .entry(checks[1])
                    .or_insert_with(Vec::new)
                    .push(elf);
                break;
            }
        }
    }
    propositions.retain(|_, from| from.len() == 1);
    propositions.iter().for_each(|(_, from)| {
        moved = true;
        elves.remove(&from[0]);
    });
    propositions.drain().for_each(|(to, _)| {
        elves.insert(to);
    });
    dirs.rotate_left(1);
    moved
}

pub struct Day23;

impl<'a> Day<'a> for Day23 {
    const DAY: usize = 23;
    type Input = HashSet<(i32, i32)>;
    type ProcessedInput = HashSet<(i32, i32)>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim_end()
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices()
                    .filter_map(move |(x, c)| (c == '#').then_some((x as i32, y as i32)))
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut elves = input.clone();
        let mut dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        for _ in 0..10 {
            round(&mut elves, &mut dirs);
        }
        let elf = *elves.iter().next().unwrap();
        let (mut min, mut max) = (elf, elf);
        elves.iter().for_each(|e| {
            min.0 = i32::min(min.0, e.0);
            min.1 = i32::min(min.1, e.1);
            max.0 = i32::max(max.0, e.0);
            max.1 = i32::max(max.1, e.1);
        });
        let ans = (1 + max.0 - min.0) * (1 + max.1 - min.1) - elves.len() as i32;
        (input, ans.to_string())
    }

    fn solve_part2(mut elves: Self::ProcessedInput) -> String {
        let mut dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        (1..)
            .find(|_| !round(&mut elves, &mut dirs))
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test_day23 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        ....#..
        ..###.#
        #...#.#
        .#...##
        #.###..
        ##.#.##
        .#..#..
    "};

    #[test]
    fn test_day23_examples() {
        let input = Day23::parse(EXAMPLE);
        let (input, part1) = Day23::solve_part1(input);
        let part2 = Day23::solve_part2(input);
        assert_eq!(part1, "110");
        assert_eq!(part2, "20");
    }
}

bench_day!(23);
