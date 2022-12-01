use crate::day::Day;

pub struct Day01;

impl<'a> Day<'a> for Day01 {
    const DAY: usize = 1;

    type Input = Vec<u32>;
    type ProcessedInput = Vec<u32>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .split("\n\n")
            .map(|elf| elf.lines().map(|c| c.parse::<u32>().unwrap()).sum())
            .collect()
    }

    fn solve_part1(mut totals: Self::Input) -> (Self::ProcessedInput, String) {
        totals.sort();
        totals.reverse();
        let ans = totals[0].to_string();
        (totals, ans)
    }

    fn solve_part2(totals: Self::ProcessedInput) -> String {
        totals.into_iter().take(3).sum::<u32>().to_string()
    }
}
