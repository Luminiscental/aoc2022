use itertools::iproduct;
use std::collections::HashSet;

use crate::day::Day;

fn view_ray<I>(sight: I, grid: &[Vec<i32>], visible: &mut HashSet<(i32, i32)>)
where
    I: Iterator<Item = (i32, i32)>,
{
    let mut hidden = -1;
    for (i, j) in sight {
        let height = grid[i as usize][j as usize];
        if height > hidden {
            visible.insert((i, j));
            hidden = height;
        }
    }
}

fn count_ray<I>(sight: I, height: i32, grid: &[Vec<i32>]) -> usize
where
    I: Iterator<Item = (i32, i32)>,
{
    let mut count = 0;
    for (i, j) in sight {
        count += 1;
        if grid[i as usize][j as usize] >= height {
            break;
        }
    }
    count
}

pub struct Day08;

impl<'a> Day<'a> for Day08 {
    const DAY: usize = 8;
    type Input = Vec<Vec<i32>>;
    type ProcessedInput = Vec<Vec<i32>>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect()
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut visible = HashSet::new();
        let (width, height) = (input[0].len() as i32, input.len() as i32);
        for j in 0..width {
            view_ray((0..height).map(|i| (i, j)), &input, &mut visible);
            view_ray((0..height).rev().map(|i| (i, j)), &input, &mut visible);
        }
        for i in 0..height {
            view_ray((0..width).map(|j| (i, j)), &input, &mut visible);
            view_ray((0..width).rev().map(|j| (i, j)), &input, &mut visible);
        }
        (input, visible.len().to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let (width, height) = (input[0].len() as i32, input.len() as i32);
        iproduct!(1..width - 1, 1..height - 1)
            .map(|(i, j)| {
                let h = input[i as usize][j as usize];
                count_ray((0..j).rev().map(|k| (i, k)), h, &input)
                    * count_ray((j + 1..width).map(|k| (i, k)), h, &input)
                    * count_ray((0..i).rev().map(|k| (k, j)), h, &input)
                    * count_ray((i + 1..height).map(|k| (k, j)), h, &input)
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
