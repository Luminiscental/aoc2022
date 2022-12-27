use std::iter;

use crate::day::Day;

fn from_snafu(string: &str) -> i64 {
    string
        .as_bytes()
        .iter()
        .rev()
        .map(|&b| match b {
            b'=' => -2,
            b'-' => -1,
            b'0' => 0,
            b'1' => 1,
            b'2' => 2,
            _ => panic!("invalid digit {}", b as char),
        })
        .zip(iter::successors(Some(1), |n| Some(n * 5)))
        .map(|(a, b)| a * b)
        .sum()
}

fn to_snafu(mut number: i64) -> String {
    iter::from_fn(|| {
        (number != 0).then(|| {
            let digit = number.rem_euclid(5);
            let digit = if digit > 2 { digit - 5 } else { digit };
            number = (number - digit) / 5;
            match digit {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => unreachable!(),
            }
        })
    })
    .collect::<Vec<_>>()
    .into_iter()
    .rev()
    .collect()
}

pub struct Day25;

impl<'a> Day<'a> for Day25 {
    const DAY: usize = 25;
    type Input = Vec<i64>;
    type ProcessedInput = ();

    fn parse(input: &'a str) -> Self::Input {
        input.trim_end().lines().map(from_snafu).collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        ((), to_snafu(input.into_iter().sum()))
    }

    fn solve_part2(_: Self::ProcessedInput) -> String {
        "Merry Christmas!".to_string()
    }
}

#[cfg(test)]
mod test_day25 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        1=-0-2
        12111
        2=0=
        21
        2=01
        111
        20012
        112
        1=-1=
        1-12
        12
        1=
        122
    "};

    #[test]
    fn test_day25_examples() {
        let snafus = vec!["1", "2", "1=", "1-", "10", "11", "12", "2=", "2-", "20"];
        assert_eq!(snafus, (1..=10).map(to_snafu).collect::<Vec<_>>());
        assert_eq!(
            snafus.into_iter().map(from_snafu).collect::<Vec<_>>(),
            (1..=10).collect::<Vec<_>>()
        );

        assert_eq!(to_snafu(15), "1=0");
        assert_eq!(to_snafu(20), "1-0");
        assert_eq!(to_snafu(2022), "1=11-2");
        assert_eq!(to_snafu(12345), "1-0---0");
        assert_eq!(to_snafu(314159265), "1121-1110-1=0");

        let input = Day25::parse(EXAMPLE);
        let (_, part1) = Day25::solve_part1(input);
        assert_eq!(part1, "2=-1=0");
    }
}

bench_day!(25);
