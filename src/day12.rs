use std::collections::{HashSet, VecDeque};

use crate::day::Day;

pub struct Input {
    heights: Vec<i32>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

fn steps_from(start: (usize, usize), input: &Input) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_front((start, 0));
    visited.insert(start);
    while let Some((pos, steps)) = queue.pop_back() {
        let mut neighbs = Vec::new();
        if pos.0 > 0 {
            neighbs.push((pos.0 - 1, pos.1));
        }
        if pos.0 < input.width - 1 {
            neighbs.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            neighbs.push((pos.0, pos.1 - 1));
        }
        if pos.1 < input.height - 1 {
            neighbs.push((pos.0, pos.1 + 1));
        }
        for new_pos in neighbs.into_iter() {
            if input.heights[new_pos.0 + input.width * new_pos.1]
                - input.heights[pos.0 + input.width * pos.1]
                <= 1
                && !visited.contains(&new_pos)
            {
                if new_pos == input.end {
                    return Some(steps + 1);
                }
                queue.push_front((new_pos, steps + 1));
                visited.insert(new_pos);
            }
        }
    }
    None
}

pub struct Day12;

impl<'a> Day<'a> for Day12 {
    const DAY: usize = 12;
    type Input = Input;
    type ProcessedInput = Input;

    fn parse(input: &'a str) -> Self::Input {
        let width = input.find(|c: char| c.is_whitespace()).unwrap();
        let (mut start, mut end) = ((0, 0), (0, 0));
        let mut heights = Vec::new();
        for (i, line) in input.trim().lines().enumerate() {
            for (j, c) in line.bytes().enumerate() {
                let h = match c {
                    b'S' => {
                        start = (j, i);
                        0
                    }
                    b'E' => {
                        end = (j, i);
                        25
                    }
                    c => c - b'a',
                };
                heights.push(h as i32);
            }
        }
        let height = heights.len() / width;
        Input {
            heights,
            width,
            height,
            start,
            end,
        }
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = steps_from(input.start, &input).unwrap().to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let mut starts = Vec::new();
        for i in 0..input.height {
            for j in 0..input.width {
                if input.heights[j + input.width * i] == 0 {
                    starts.push((j, i));
                }
            }
        }
        starts
            .into_iter()
            .filter_map(|s| steps_from(s, &input))
            .min()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test_day12 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};

    #[test]
    fn test_day12_examples() {
        let input = Day12::parse(EXAMPLE);
        let (input, part1) = Day12::solve_part1(input);
        let part2 = Day12::solve_part2(input);
        assert_eq!(part1, "31");
        assert_eq!(part2, "29");
    }
}

bench_day!(12);
