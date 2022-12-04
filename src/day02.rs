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

#[cfg(test)]
mod test_day02 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        A Y
        B X
        C Z
    "};

    #[test]
    fn test_day02_examples() {
        let input = Day02::parse(EXAMPLE);
        let (input, part1) = Day02::solve_part1(input);
        let part2 = Day02::solve_part2(input);
        assert_eq!(part1, "15");
        assert_eq!(part2, "12");
    }
}

bench_day!(02);
