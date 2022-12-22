use std::iter;

use crate::day::Day;

pub struct Board {
    path: Vec<((i32, i32), u32)>,
    width: usize,
    height: usize,
    board: Vec<Option<bool>>,
}

pub struct Day22;

impl<'a> Day<'a> for Day22 {
    const DAY: usize = 22;
    type Input = Board;
    type ProcessedInput = Board;

    fn parse(input: &'a str) -> Self::Input {
        let lines = input.trim_end().lines().collect::<Vec<_>>();
        let height = lines.len() - 2;
        let width = lines[..height].iter().map(|s| s.len()).max().unwrap();
        let board = lines[..height]
            .iter()
            .flat_map(|line| {
                line.chars()
                    .map(|c| match c {
                        ' ' => None,
                        '.' => Some(false),
                        '#' => Some(true),
                        _ => panic!("unrecognized tile {c:?}"),
                    })
                    .chain(iter::repeat(None).take(width - line.len()))
            })
            .collect();
        let mut facing = (1, 0);
        let mut path = Vec::new();
        let mut instr = *lines.last().unwrap();
        while !instr.is_empty() {
            match &instr[..1] {
                "L" => {
                    facing = (facing.1, -facing.0);
                    instr = &instr[1..];
                }
                "R" => {
                    facing = (-facing.1, facing.0);
                    instr = &instr[1..];
                }
                _ => {
                    let i = instr
                        .find(|c: char| !c.is_ascii_digit())
                        .unwrap_or(instr.len());
                    path.push((facing, instr[..i].parse().unwrap()));
                    instr = &instr[i..];
                }
            }
        }
        Board {
            path,
            width,
            height,
            board,
        }
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut pos = (0, 0);
        while input.board[pos.0 as usize].is_none() {
            pos.0 += 1;
        }
        for (dir, count) in input.path.iter().copied() {
            for _ in 0..count {
                let mut to = (pos.0 + dir.0, pos.1 + dir.1);
                if !(0..input.width as i32).contains(&to.0)
                    || !(0..input.height as i32).contains(&to.1)
                    || input.board[(to.0 + input.width as i32 * to.1) as usize].is_none()
                {
                    match dir {
                        (1, 0) => to.0 = 0,
                        (-1, 0) => to.0 = input.width as i32 - 1,
                        (0, 1) => to.1 = 0,
                        (0, -1) => to.1 = input.height as i32 - 1,
                        _ => unreachable!(),
                    }
                    while input.board[(to.0 + input.width as i32 * to.1) as usize].is_none() {
                        to = (to.0 + dir.0, to.1 + dir.1);
                    }
                }
                if input.board[(to.0 + input.width as i32 * to.1) as usize] == Some(false) {
                    pos = to;
                } else {
                    break;
                }
            }
        }
        let facing_score = match input.path.last().unwrap().0 {
            (1, 0) => 0,
            (0, 1) => 1,
            (-1, 0) => 2,
            (0, -1) => 3,
            _ => unreachable!(),
        };
        let ans = (1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + facing_score).to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test_day22 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
                ...#
                .#..
                #...
                ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
                ...#....
                .....#..
                .#......
                ......#.

        10R5L5R10L4R5L5
    "};

    #[test]
    fn test_day22_examples() {
        let input = Day22::parse(EXAMPLE);
        let (input, part1) = Day22::solve_part1(input);
        let part2 = Day22::solve_part2(input);
        assert_eq!(part1, "6032");
        assert_eq!(part2, "5031");
    }
}

bench_day!(22);
