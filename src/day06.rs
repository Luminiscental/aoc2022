use crate::day::Day;

pub struct Day06;

impl<'a> Day<'a> for Day06 {
    const DAY: usize = 6;
    type Input = &'a str;
    type ProcessedInput = &'a str;

    fn parse(input: &'a str) -> Self::Input {
        input.trim()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = (4..input.len())
            .find(|&i| (i - 4..i).all(|j| !input[i - 4..j].contains(&input[j..j + 1])))
            .unwrap()
            .to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        (14..input.len())
            .find(|&i| (i - 14..i).all(|j| !input[i - 14..j].contains(&input[j..j + 1])))
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test_day06 {
    use super::*;

    const EXAMPLE1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_day06_examples() {
        assert_eq!(Day06::solve_part1(EXAMPLE1).1, "7");
        assert_eq!(Day06::solve_part2(EXAMPLE1), "19");

        assert_eq!(Day06::solve_part1(EXAMPLE2).1, "5");
        assert_eq!(Day06::solve_part2(EXAMPLE2), "23");

        assert_eq!(Day06::solve_part1(EXAMPLE3).1, "6");
        assert_eq!(Day06::solve_part2(EXAMPLE3), "23");

        assert_eq!(Day06::solve_part1(EXAMPLE4).1, "10");
        assert_eq!(Day06::solve_part2(EXAMPLE4), "29");

        assert_eq!(Day06::solve_part1(EXAMPLE5).1, "11");
        assert_eq!(Day06::solve_part2(EXAMPLE5), "26");
    }
}

bench_day!(06);
