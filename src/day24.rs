use std::{iter, mem};

use hashbrown::{HashMap, HashSet};

use crate::day::Day;

pub struct Valley {
    width: i32,
    height: i32,
    blizzards: HashMap<(i32, i32), Vec<(i32, i32)>>,
}

impl Valley {
    fn navigate(&mut self, start: (i32, i32), end: (i32, i32)) -> i32 {
        let mut presence = iter::once(start).collect::<HashSet<_>>();
        let mut time = 0;
        while !presence.is_empty() {
            time += 1;
            for (pos, dirs) in mem::take(&mut self.blizzards) {
                for dir in dirs {
                    let new_pos = (
                        (pos.0 + dir.0).rem_euclid(self.width),
                        (pos.1 + dir.1).rem_euclid(self.height),
                    );
                    self.blizzards
                        .entry(new_pos)
                        .or_insert_with(Vec::new)
                        .push(dir);
                }
            }
            for pos in mem::take(&mut presence) {
                for dir in [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)] {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    let in_range = new_pos == start
                        || ((0..self.width).contains(&new_pos.0)
                            && (0..self.height).contains(&new_pos.1));
                    if new_pos == end {
                        return time;
                    } else if in_range && !self.blizzards.contains_key(&new_pos) {
                        presence.insert(new_pos);
                    }
                }
            }
        }
        panic!("no path found")
    }
}

pub struct Day24;

impl<'a> Day<'a> for Day24 {
    const DAY: usize = 24;
    type Input = Valley;
    type ProcessedInput = (i32, Valley);

    fn parse(input: &'a str) -> Self::Input {
        let lines = input.trim_end().lines().collect::<Vec<_>>();
        let (width, height) = (lines[0].len() - 2, lines.len() - 2);
        let mut blizzards = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                let dir = match lines[y + 1].as_bytes()[x + 1] {
                    b'>' => (1, 0),
                    b'<' => (-1, 0),
                    b'v' => (0, 1),
                    b'^' => (0, -1),
                    _ => continue,
                };
                blizzards
                    .entry((x as i32, y as i32))
                    .or_insert_with(Vec::new)
                    .push(dir);
            }
        }
        Valley {
            width: width as i32,
            height: height as i32,
            blizzards,
        }
    }

    fn solve_part1(mut valley: Self::Input) -> (Self::ProcessedInput, String) {
        let time = valley.navigate((0, -1), (valley.width - 1, valley.height));
        ((time, valley), time.to_string())
    }

    fn solve_part2((mut time, mut valley): Self::ProcessedInput) -> String {
        time += valley.navigate((valley.width - 1, valley.height), (0, -1));
        time += valley.navigate((0, -1), (valley.width - 1, valley.height));
        time.to_string()
    }
}

#[cfg(test)]
mod test_day24 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    "};

    #[test]
    fn test_day24_examples() {
        let input = Day24::parse(EXAMPLE);
        let (input, part1) = Day24::solve_part1(input);
        let part2 = Day24::solve_part2(input);
        assert_eq!(part1, "18");
        assert_eq!(part2, "54");
    }
}

bench_day!(24);
