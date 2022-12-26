use crate::day::Day;

#[derive(Clone)]
pub struct File {
    numbers: Vec<i64>,
    indices: Vec<usize>,
}

impl File {
    fn new(numbers: Vec<i64>) -> Self {
        let indices = (0..numbers.len()).collect();
        Self { numbers, indices }
    }

    fn mix(&mut self) {
        let wrap = |n: i64| n.rem_euclid(self.numbers.len() as i64 - 1);
        for (i, n) in self.numbers.iter().copied().enumerate() {
            let j = self.indices.iter().position(|&idx| idx == i).unwrap();
            self.indices.remove(j);
            self.indices.insert(wrap(j as i64 + n) as usize, i);
        }
    }

    fn score(&self) -> i64 {
        let zero = self.numbers.iter().position(|&n| n == 0).unwrap();
        let zero = self.indices.iter().position(|&idx| idx == zero).unwrap();
        [1000, 2000, 3000]
            .into_iter()
            .map(|c: usize| {
                let moves = (zero + c).rem_euclid(self.numbers.len());
                self.numbers[self.indices[moves]]
            })
            .sum()
    }
}

pub struct Day20;

impl<'a> Day<'a> for Day20 {
    const DAY: usize = 20;
    type Input = File;
    type ProcessedInput = File;

    fn parse(input: &'a str) -> Self::Input {
        File::new(
            input
                .trim()
                .lines()
                .map(|line| line.parse().unwrap())
                .collect(),
        )
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut file = input.clone();
        file.mix();
        (input, file.score().to_string())
    }

    fn solve_part2(mut file: Self::ProcessedInput) -> String {
        file.numbers.iter_mut().for_each(|n| *n *= 811589153);
        for _ in 0..10 {
            file.mix();
        }
        file.score().to_string()
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
