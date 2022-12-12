use crate::{day::Day, util};

pub struct Input {
    heights: Vec<i32>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

fn neighbours<F: Fn(i32, i32) -> bool>(
    pos: (usize, usize),
    input: &Input,
    filter: F,
) -> impl Iterator<Item = (usize, usize)> {
    let idx = pos.0 + input.width * pos.1;
    let old_height = input.heights[idx];
    [
        (pos.0 > 0).then(|| (pos.0 - 1, pos.1, input.heights[idx - 1])),
        (pos.0 < input.width - 1).then(|| (pos.0 + 1, pos.1, input.heights[idx + 1])),
        (pos.1 > 0).then(|| (pos.0, pos.1 - 1, input.heights[idx - input.width])),
        (pos.1 < input.height - 1).then(|| (pos.0, pos.1 + 1, input.heights[idx + input.width])),
    ]
    .into_iter()
    .flatten()
    .filter_map(move |(p0, p1, h)| filter(old_height, h).then_some((p0, p1)))
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
        let ans = util::bfs(
            input.start,
            |pos| neighbours(pos, &input, |old_h, new_h| new_h - old_h <= 1),
            |pos| pos == input.end,
        )
        .unwrap()
        .to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        util::bfs(
            input.end,
            |pos| neighbours(pos, &input, |old_h, new_h| old_h - new_h <= 1),
            |pos| input.heights[pos.0 + input.width * pos.1] == 0,
        )
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
