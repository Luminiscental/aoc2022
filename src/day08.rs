use itertools::iproduct;
use std::collections::HashSet;

use crate::day::Day;

fn view_ray<I>(sight: I, grid: &[u32], visible: &mut HashSet<usize>)
where
    I: Iterator<Item = usize>,
{
    let mut hidden = -1i32;
    for i in sight {
        let height = grid[i];
        if height as i32 > hidden {
            visible.insert(i);
            hidden = height as i32;
        }
    }
}

fn count_ray<I>(sight: I, height: u32, grid: &[u32]) -> usize
where
    I: Iterator<Item = usize>,
{
    let mut count = 0;
    for i in sight {
        count += 1;
        if grid[i] >= height {
            break;
        }
    }
    count
}

pub struct Day08;

impl<'a> Day<'a> for Day08 {
    const DAY: usize = 8;
    type Input = (usize, usize, Vec<u32>);
    type ProcessedInput = (usize, usize, Vec<u32>);

    fn parse(input: &'a str) -> Self::Input {
        let width = input.trim().lines().next().unwrap().len();
        let grid = input
            .trim()
            .lines()
            .flat_map(str::chars)
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        let height = grid.len() / width;
        (width, height, grid)
    }

    fn solve_part1((w, h, grid): Self::Input) -> (Self::ProcessedInput, String) {
        let mut visible = HashSet::new();
        for j in 0..w {
            view_ray((0..h).map(|i| j + w * i), &grid, &mut visible);
            view_ray((0..h).rev().map(|i| j + w * i), &grid, &mut visible);
        }
        for i in 0..h {
            view_ray((0..w).map(|j| j + w * i), &grid, &mut visible);
            view_ray((0..w).rev().map(|j| j + w * i), &grid, &mut visible);
        }
        ((w, h, grid), visible.len().to_string())
    }

    fn solve_part2((width, height, grid): Self::ProcessedInput) -> String {
        iproduct!(1..width - 1, 1..height - 1)
            .map(|(i, j)| {
                let h = grid[j + width * i];
                count_ray((0..j).rev().map(|k| k + width * i), h, &grid)
                    * count_ray((j + 1..width).map(|k| k + width * i), h, &grid)
                    * count_ray((0..i).rev().map(|k| j + width * k), h, &grid)
                    * count_ray((i + 1..height).map(|k| j + width * k), h, &grid)
            })
            .max()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test_day08 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn test_day08_examples() {
        let input = Day08::parse(EXAMPLE);
        let (input, part1) = Day08::solve_part1(input);
        let part2 = Day08::solve_part2(input);
        assert_eq!(part1, "21");
        assert_eq!(part2, "8");
    }
}

bench_day!(08);
