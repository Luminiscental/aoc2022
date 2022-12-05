use crate::day::Day;

pub struct Day05;

impl<'a> Day<'a> for Day05 {
    const DAY: usize = 5;
    type Input = (Vec<Vec<u8>>, Vec<(usize, usize, usize)>);
    type ProcessedInput = (Vec<Vec<u8>>, Vec<(usize, usize, usize)>);

    fn parse(input: &'a str) -> Self::Input {
        let (stacks, moves) = input.split_once("\n\n").unwrap();
        let rows = stacks.lines().rev().collect::<Vec<_>>();
        let stacks = (0..rows[0].len() / 4 + 1)
            .map(|s| {
                rows.iter()
                    .skip(1)
                    .map(|r| r.as_bytes()[4 * s + 1])
                    .take_while(u8::is_ascii_uppercase)
                    .collect()
            })
            .collect();
        let moves = moves
            .lines()
            .map(|line| {
                let mut nums = line.split_whitespace().skip(1).step_by(2);
                let mut num = || nums.next().unwrap().parse::<usize>().unwrap();
                (num(), num() - 1, num() - 1)
            })
            .collect();
        (stacks, moves)
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut stacks = input.0.clone();
        for (count, from, to) in input.1.iter().copied() {
            let cut_index = stacks[from].len() - count;
            let moved = stacks[from].split_off(cut_index);
            stacks[to].extend(moved.into_iter().rev());
        }
        let ans = stacks.iter().map(|s| *s.last().unwrap() as char).collect();
        (input, ans)
    }

    fn solve_part2((mut stacks, moves): Self::ProcessedInput) -> String {
        for (count, from, to) in moves.iter().copied() {
            let cut_index = stacks[from].len() - count;
            let moved = stacks[from].split_off(cut_index);
            stacks[to].extend(moved);
        }
        stacks.iter().map(|s| *s.last().unwrap() as char).collect()
    }
}

#[cfg(test)]
mod test_day05 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn test_day05_examples() {
        let input = Day05::parse(EXAMPLE);
        let (input, part1) = Day05::solve_part1(input);
        let part2 = Day05::solve_part2(input);
        assert_eq!(part1, "CMZ");
        assert_eq!(part2, "MCD");
    }
}

bench_day!(05);
