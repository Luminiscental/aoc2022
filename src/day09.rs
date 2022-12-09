use std::collections::HashSet;

use crate::day::Day;

fn tail_visits<const N: usize>(moves: &[((i32, i32), u32)]) -> usize {
    let mut rope = [(0, 0); N];
    let mut tail_locs = HashSet::new();
    for (step, count) in moves.iter().copied() {
        for _ in 0..count {
            rope[0].0 += step.0;
            rope[0].1 += step.1;
            for i in 1..N {
                let disp = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                let signs = (disp.0.signum(), disp.1.signum());
                if signs.0 * disp.0 == 2 || signs.1 * disp.1 == 2 {
                    rope[i].0 += signs.0;
                    rope[i].1 += signs.1;
                }
            }
            tail_locs.insert(rope[N - 1]);
        }
    }
    tail_locs.len()
}

pub struct Day09;

impl<'a> Day<'a> for Day09 {
    const DAY: usize = 9;
    type Input = Vec<((i32, i32), u32)>;
    type ProcessedInput = Vec<((i32, i32), u32)>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| {
                let (dir, count) = line.split_once(' ').unwrap();
                let count = count.parse().unwrap();
                match dir {
                    "U" => ((0, 1), count),
                    "D" => ((0, -1), count),
                    "L" => ((-1, 0), count),
                    "R" => ((1, 0), count),
                    _ => panic!("unknown direction"),
                }
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = tail_visits::<2>(&input).to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        tail_visits::<10>(&input).to_string()
    }
}

#[cfg(test)]
mod test_day09 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE1: &str = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};

    const EXAMPLE2: &str = indoc! {"
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20
    "};

    #[test]
    fn test_day09_examples() {
        let input = Day09::parse(EXAMPLE1);
        let (input, part1) = Day09::solve_part1(input);
        let part2 = Day09::solve_part2(input);
        assert_eq!(part1, "13");
        assert_eq!(part2, "1");

        assert_eq!(Day09::solve_part2(Day09::parse(EXAMPLE2)), "36");
    }
}

bench_day!(09);
