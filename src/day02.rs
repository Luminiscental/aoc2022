use crate::day::Day;

pub struct Day02;

impl<'a> Day<'a> for Day02 {
    const DAY: usize = 2;
    type Input = Vec<(i32, i32)>;
    type ProcessedInput = Vec<(i32, i32)>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .map(|line| {
                (
                    (line.as_bytes()[0] - b'A') as i32,
                    (line.as_bytes()[2] - b'X') as i32,
                )
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = input
            .iter()
            .map(|(l, r)| 3 * (r - l + 1).rem_euclid(3) + r + 1)
            .sum::<i32>()
            .to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        input
            .iter()
            .map(|(l, r)| r * 3 + (l + r - 1).rem_euclid(3) + 1)
            .sum::<i32>()
            .to_string()
    }
}
