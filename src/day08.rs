use crate::day::Day;

fn scan<I: Iterator<Item = usize>>(ray: I, grid: &[u32], vis_score: &mut [(bool, u32)]) {
    let mut seen = [None; 10];
    for (n, i) in ray.enumerate() {
        let h = grid[i] as usize;
        vis_score[i].0 |= seen[h].is_none();
        vis_score[i].1 *= (n - seen[h].unwrap_or(0)) as u32;
        seen.iter_mut().take(h + 1).for_each(|s| *s = Some(n));
    }
}

pub struct Day08;

impl<'a> Day<'a> for Day08 {
    const DAY: usize = 8;
    type Input = (usize, usize, Vec<u32>); // (width, height, grid)
    type ProcessedInput = Vec<(bool, u32)>; // [(visible, score)]

    fn parse(input: &'a str) -> Self::Input {
        let width = input.find(|c: char| c.is_whitespace()).unwrap();
        let grid = input.chars().filter_map(|c| c.to_digit(10)).collect();
        let height = Vec::len(&grid) / width;
        (width, height, grid)
    }

    fn solve_part1((width, height, grid): Self::Input) -> (Self::ProcessedInput, String) {
        let mut vis_score = vec![(false, 1); grid.len()];
        for j in 0..width {
            let ran = (0..height).map(|i| j + width * i);
            scan(ran.clone().rev(), &grid, &mut vis_score);
            scan(ran, &grid, &mut vis_score);
        }
        for i in 0..height {
            let ran = (0..width).map(|j| j + width * i);
            scan(ran.clone().rev(), &grid, &mut vis_score);
            scan(ran, &grid, &mut vis_score);
        }
        let visible = vis_score.iter().filter(|&&(v, _)| v).count();
        (vis_score, visible.to_string())
    }

    fn solve_part2(vis_score: Self::ProcessedInput) -> String {
        vis_score.iter().map(|&(_, s)| s).max().unwrap().to_string()
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
