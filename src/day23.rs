use hashbrown::{HashMap, HashSet};

use crate::day::Day;

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const SIDES: [[usize; 3]; 4] = [[0, 3, 5], [2, 4, 7], [0, 1, 2], [5, 6, 7]];

fn round(elves: &mut HashSet<(i32, i32)>, sides: &mut [[usize; 3]; 4]) -> bool {
    let mut propositions = HashMap::new();
    for elf in elves.iter().copied() {
        let mut neighbs = [false; 8];
        for (n, d) in neighbs.iter_mut().zip(DIRS.iter()) {
            *n = elves.contains(&(elf.0 + d.0, elf.1 + d.1));
        }
        if neighbs == [false; 8] {
            continue;
        }
        for [i, j, k] in sides.iter().copied() {
            if !neighbs[i] && !neighbs[j] && !neighbs[k] {
                let target = (elf.0 + DIRS[j].0, elf.1 + DIRS[j].1);
                propositions
                    .entry(target)
                    .and_modify(|e| *e = None)
                    .or_insert(Some(elf));
                break;
            }
        }
    }
    propositions.retain(|_, p| p.is_some());
    let moved = !propositions.is_empty();
    for (_, p) in propositions.iter() {
        elves.remove(&p.unwrap());
    }
    for (t, _) in propositions.drain() {
        elves.insert(t);
    }
    sides.rotate_left(1);
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
        let mut sides = SIDES;
        for _ in 0..10 {
            round(&mut elves, &mut sides);
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
        let mut sides = SIDES;
        (1..)
            .find(|_| !round(&mut elves, &mut sides))
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
