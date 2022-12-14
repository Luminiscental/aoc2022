use std::collections::HashSet;

use crate::day::Day;

fn drop_sand(grid: &mut HashSet<(i32, i32)>, ground: i32, use_ground: bool) -> Option<(i32, i32)> {
    let (mut sx, mut sy) = (500, 0);
    loop {
        let mut moved = false;
        match (sy + 1..ground).find(|&y| grid.contains(&(sx, y))) {
            Some(y) if y > sy + 1 => {
                sy = y - 1;
                moved = true;
            }
            None if use_ground => {
                grid.insert((sx, ground - 1));
                return Some((sx, ground - 1));
            }
            None if !use_ground => return None,
            _ => {}
        }
        if !grid.contains(&(sx - 1, sy + 1)) {
            sx -= 1;
            sy += 1;
            moved = true;
        } else if !grid.contains(&(sx + 1, sy + 1)) {
            sx += 1;
            sy += 1;
            moved = true;
        }
        if !moved {
            grid.insert((sx, sy));
            return Some((sx, sy));
        }
    }
}

pub struct Day14;

impl<'a> Day<'a> for Day14 {
    const DAY: usize = 14;
    type Input = HashSet<(i32, i32)>;
    type ProcessedInput = (i32, HashSet<(i32, i32)>);

    fn parse(input: &'a str) -> Self::Input {
        let mut grid = HashSet::new();
        for line in input.trim().lines() {
            let points = line
                .split(" -> ")
                .map(|coord| {
                    let (x, y) = coord.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect::<Vec<_>>();
            grid.extend(points.windows(2).flat_map(|wind| {
                let (x0, y0) = wind[0];
                let (x1, y1) = wind[1];
                let (x0, x1) = (i32::min(x0, x1), i32::max(x0, x1));
                let (y0, y1) = (i32::min(y0, y1), i32::max(y0, y1));
                (x0..=x1).flat_map(move |x| (y0..=y1).map(move |y| (x, y)))
            }));
        }
        grid
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut grid = input.clone();
        let ground = *grid.iter().map(|(_, y)| y).max().unwrap() + 2;
        let mut drops = 0;
        loop {
            match drop_sand(&mut grid, ground, false) {
                Some(_) => drops += 1,
                None => return ((ground, input), drops.to_string()),
            }
        }
    }

    fn solve_part2((ground, mut grid): Self::ProcessedInput) -> String {
        let mut drops = 0;
        loop {
            match drop_sand(&mut grid, ground, true) {
                Some((500, 0)) => return (drops + 1).to_string(),
                Some(_) => drops += 1,
                None => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod test_day14 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "};

    #[test]
    fn test_day14_examples() {
        let input = Day14::parse(EXAMPLE);
        let (input, part1) = Day14::solve_part1(input);
        let part2 = Day14::solve_part2(input);
        assert_eq!(part1, "24");
        assert_eq!(part2, "93");
    }
}

bench_day!(14);
