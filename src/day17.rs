use hashbrown::HashMap;
use lazy_static::lazy_static;

use crate::day::Day;

struct Rock {
    points: Vec<(i32, i32)>,
    width: i32,
    height: i32,
}

struct Placement {
    idx: usize,
    jet: usize,
    x: i32,
    level: i32,
}

pub struct Cycle {
    start: usize,
    period: usize,
    height: i32,
}

lazy_static! {
    static ref ROCKS: [Rock; 5] = [
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
        Rock::new(vec![(1, 0), (0, 1), (1, 1), (1, 2), (2, 1)]),
        Rock::new(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
        Rock::new(vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
        Rock::new(vec![(0, 0), (1, 0), (0, 1), (1, 1)]),
    ];
}

impl Rock {
    fn new(points: Vec<(i32, i32)>) -> Self {
        let width = points.iter().map(|p| p.0).max().unwrap() + 1;
        let height = points.iter().map(|p| p.1).max().unwrap() + 1;
        Self {
            points,
            width,
            height,
        }
    }
}

fn height_after(rocks: u64, cycle: &Cycle, jets: &[i32]) -> u64 {
    let after = rocks - 1 - cycle.start as u64;
    let remainder = cycle.start + 1 + (after % cycle.period as u64) as usize;
    simulate(remainder, jets) as u64 + cycle.height as u64 * (after / cycle.period as u64)
}

fn simulate(rocks: usize, jets: &[i32]) -> i32 {
    let (mut tower, mut level, mut jet) = Default::default();
    for rock in ROCKS.iter().cycle().take(rocks) {
        place_rock(0, rock, &mut jet, jets, &mut level, &mut tower);
    }
    level
}

fn find_cycle(jets: &[i32]) -> Cycle {
    let (mut tower, mut level, mut jet) = Default::default();
    let mut starts = Vec::new();
    for (idx, rock) in ROCKS.iter().cycle().enumerate() {
        let (x, support) = place_rock(idx, rock, &mut jet, jets, &mut level, &mut tower);
        if support.is_empty() {
            starts.clear();
        } else {
            starts.retain(|p: &Placement| support.iter().all(|&i| i >= p.idx));
        }
        if (level - rock.height + 1..level)
            .all(|y| (0..7).all(|x| tower.get(&(x, y)).map_or(true, |&i| i == idx)))
        {
            if let Some(p) = starts
                .iter()
                .find(|&p| x == p.x && jet == p.jet && (idx - p.idx) % ROCKS.len() == 0)
            {
                return Cycle {
                    start: p.idx,
                    period: idx - p.idx,
                    height: level - p.level,
                };
            }
        }
        starts.push(Placement { idx, jet, x, level });
    }
    unreachable!()
}

fn place_rock(
    idx: usize,
    rock: &Rock,
    jet: &mut usize,
    jets: &[i32],
    level: &mut i32,
    tower: &mut HashMap<(i32, i32), usize>,
) -> (i32, Vec<usize>) {
    let (mut x, mut y) = (2, *level + 3);
    let mut support = Vec::new();
    let mut unblocked = |x, y| {
        let sl = support.len();
        support.extend(
            rock.points
                .iter()
                .filter_map(|p| tower.get(&(x + p.0, y + p.1))),
        );
        sl == support.len()
    };
    loop {
        let dx = jets[*jet];
        *jet = (*jet + 1) % jets.len();
        if x + dx >= 0 && x + dx + rock.width <= 7 && unblocked(x + dx, y) {
            x += dx;
        }
        if y > 0 && unblocked(x, y - 1) {
            y -= 1;
        } else {
            break;
        }
    }
    *level = i32::max(*level, y + rock.height);
    tower.extend(rock.points.iter().map(|p| ((x + p.0, y + p.1), idx)));
    (x, support)
}

pub struct Day17;

impl<'a> Day<'a> for Day17 {
    const DAY: usize = 17;
    type Input = Vec<i32>;
    type ProcessedInput = (Cycle, Vec<i32>);

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .chars()
            .map(|c| if c == '<' { -1 } else { 1 })
            .collect()
    }

    fn solve_part1(jets: Self::Input) -> (Self::ProcessedInput, String) {
        let cycle = find_cycle(&jets);
        let ans = height_after(2022, &cycle, &jets).to_string();
        ((cycle, jets), ans)
    }

    fn solve_part2((cycle, jets): Self::ProcessedInput) -> String {
        height_after(1000000000000, &cycle, &jets).to_string()
    }
}

#[cfg(test)]
mod test_day17 {
    use super::*;

    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_day17_examples() {
        let input = Day17::parse(EXAMPLE);
        let (input, part1) = Day17::solve_part1(input);
        let part2 = Day17::solve_part2(input);
        assert_eq!(part1, "3068");
        assert_eq!(part2, "1514285714288");
    }
}

bench_day!(17);
