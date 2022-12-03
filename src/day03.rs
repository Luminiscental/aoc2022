use crate::day::Day;

fn priority(c: char) -> u32 {
    match c as u8 {
        b if b >= b'a' => 1 + (b - b'a') as u32,
        b => 27 + (b - b'A') as u32,
    }
}

fn common(strings: &[&str]) -> char {
    strings[0]
        .chars()
        .find(|&c| strings[1..].iter().all(|s| s.contains(c)))
        .unwrap()
}

pub struct Day03;

impl<'a> Day<'a> for Day03 {
    const DAY: usize = 3;
    type Input = Vec<&'a str>;
    type ProcessedInput = Vec<&'a str>;

    fn parse(input: &'a str) -> Self::Input {
        input.lines().map(str::trim).collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = input
            .iter()
            .map(|line| [&line[..line.len() / 2], &line[line.len() / 2..]])
            .map(|p| common(&p))
            .map(priority)
            .sum::<u32>()
            .to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        input
            .chunks(3)
            .map(common)
            .map(priority)
            .sum::<u32>()
            .to_string()
    }
}
