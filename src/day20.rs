use std::iter;

use crate::day::Day;

#[derive(Clone)]
pub struct Signal {
    numbers: Vec<i64>,
    right: Vec<usize>,
}

impl Signal {
    fn new(numbers: Vec<i64>) -> Self {
        let right = (1..numbers.len()).chain(iter::once(0)).collect();
        Self { numbers, right }
    }

    fn mix(&mut self) {
        for (i, n) in self.numbers.iter().enumerate() {
            let moves = n.rem_euclid(self.numbers.len() as i64 - 1);
            let mut left = self.right.iter().position(|&idx| idx == i).unwrap();
            for _ in 0..moves {
                let next = (self.right[i], self.right[self.right[i]]);
                self.right[left] = next.0;
                self.right[i] = next.1;
                self.right[next.0] = i;
                left = next.0;
            }
        }
    }

    fn score(&self) -> i64 {
        let zero = self.numbers.iter().position(|&n| n == 0).unwrap();
        [1000, 2000, 3000]
            .into_iter()
            .map(|c: usize| {
                let moves = c.rem_euclid(self.numbers.len());
                let mut i = zero;
                for _ in 0..moves {
                    i = self.right[i];
                }
                self.numbers[i]
            })
            .sum()
    }
}

pub struct Day20;

impl<'a> Day<'a> for Day20 {
    const DAY: usize = 20;
    type Input = Signal;
    type ProcessedInput = Signal;

    fn parse(input: &'a str) -> Self::Input {
        Signal::new(
            input
                .trim()
                .lines()
                .map(|line| line.parse().unwrap())
                .collect(),
        )
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut signal = input.clone();
        signal.mix();
        (input, signal.score().to_string())
    }

    fn solve_part2(mut signal: Self::ProcessedInput) -> String {
        signal.numbers.iter_mut().for_each(|n| *n *= 811589153);
        for _ in 0..10 {
            signal.mix();
        }
        signal.score().to_string()
    }
}

#[cfg(test)]
mod test_day20 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        1
        2
        -3
        3
        -2
        0
        4
    "};

    #[test]
    fn test_day20_examples() {
        let input = Day20::parse(EXAMPLE);
        let (input, part1) = Day20::solve_part1(input);
        let part2 = Day20::solve_part2(input);
        assert_eq!(part1, "3");
        assert_eq!(part2, "1623178306");
    }
}

bench_day!(20);
