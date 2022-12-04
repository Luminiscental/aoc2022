use crate::day::Day;

pub struct Day04;

impl<'a> Day<'a> for Day04 {
    const DAY: usize = 4;
    type Input = Vec<((i32, i32), (i32, i32))>;
    type ProcessedInput = Vec<((i32, i32), (i32, i32))>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .lines()
            .map(|line| {
                let (e1, e2) = line.trim().split_once(',').unwrap();
                let (e1l, e1r) = e1.split_once('-').unwrap();
                let (e2l, e2r) = e2.split_once('-').unwrap();
                (
                    (e1l.parse().unwrap(), e1r.parse().unwrap()),
                    (e2l.parse().unwrap(), e2r.parse().unwrap()),
                )
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = input
            .iter()
            .filter(|((e1l, e1r), (e2l, e2r))| (e1l - e2l) * (e1r - e2r) <= 0)
            .count()
            .to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        input
            .iter()
            .filter(|((e1l, e1r), (e2l, e2r))| e1r >= e2l && e1l <= e2r)
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod test_day04 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc!{"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn test_day04_examples() {
        let input = Day04::parse(EXAMPLE);
        let (input, part1) = Day04::solve_part1(input);
        let part2 = Day04::solve_part2(input);
        assert_eq!(part1, "2");
        assert_eq!(part2, "4");
    }
}

bench_day!(04);
