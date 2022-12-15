use std::collections::HashSet;

use crate::day::Day;

fn intersect(int1: (i32, i32), int2: (i32, i32)) -> Option<(i32, i32)> {
    let (left, right) = (int1.0.max(int2.0), int1.1.min(int2.1));
    (left <= right).then_some((left, right))
}

fn merge(int1: (i32, i32), int2: (i32, i32)) -> Option<(i32, i32)> {
    (int1.1 >= int2.0 - 1 && int1.0 <= int2.1 + 1)
        .then(|| (i32::min(int1.0, int2.0), i32::max(int1.1, int2.1)))
}

fn merge_all(mut intervals: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    'merging: loop {
        for i in 1..intervals.len() {
            for j in 0..i {
                if let Some(merged) = merge(intervals[i], intervals[j]) {
                    intervals[j] = merged;
                    intervals.swap_remove(i);
                    continue 'merging;
                }
            }
        }
        return intervals;
    }
}

pub struct Day15Generic<const ROW: i32>;
pub type Day15 = Day15Generic<2000000>;

impl<'a, const ROW: i32> Day<'a> for Day15Generic<ROW> {
    const DAY: usize = 15;
    type Input = Vec<Sensor>;
    type ProcessedInput = Vec<Sensor>;

    fn parse(input: &'a str) -> Self::Input {
        fn xy(s: &str) -> (i32, i32) {
            let (x, y) = s.split_once(',').unwrap();
            (x[2..].parse().unwrap(), y[3..].parse().unwrap())
        }
        input
            .trim()
            .lines()
            .map(|line| {
                let (sensor, beacon) = line.split_once(':').unwrap();
                Sensor::new(xy(&sensor[10..]), xy(&beacon[22..]))
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let intervals = merge_all(input.iter().filter_map(|s| s.slice(ROW)).collect());
        let obstructed = intervals.iter().map(|i| 1 + i.1 - i.0).sum::<i32>();
        let beacons = input
            .iter()
            .filter_map(|s| (s.beacon.1 == ROW).then_some(s.beacon.0))
            .collect::<HashSet<_>>()
            .len();
        (input, (obstructed - beacons as i32).to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        for y in 0..=2 * ROW {
            let slice = |s: &Sensor| intersect(s.slice(y)?, (0, 2 * ROW));
            let intervals = merge_all(input.iter().filter_map(slice).collect());
            if intervals.len() != 1 {
                assert_eq!(intervals.len(), 2);
                let x = i32::min(intervals[0].1, intervals[1].1) + 1;
                return (4000000 * x as u64 + y as u64).to_string();
            }
        }
        panic!("no gap found")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sensor {
    at: (i32, i32),
    beacon: (i32, i32),
    radius: i32,
}

impl Sensor {
    fn new(at: (i32, i32), beacon: (i32, i32)) -> Self {
        Self {
            at,
            beacon,
            radius: (at.0.abs_diff(beacon.0) + at.1.abs_diff(beacon.1)) as i32,
        }
    }

    fn slice(self, y: i32) -> Option<(i32, i32)> {
        let d = self.at.1.abs_diff(y) as i32;
        (d <= self.radius).then_some((self.at.0 - self.radius + d, self.at.0 + self.radius - d))
    }
}

#[cfg(test)]
mod test_day15 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "};

    #[test]
    fn test_day15_examples() {
        let input = Day15Generic::<10>::parse(EXAMPLE);
        let (input, part1) = Day15Generic::<10>::solve_part1(input);
        let part2 = Day15Generic::<10>::solve_part2(input);
        assert_eq!(part1, "26");
        assert_eq!(part2, "56000011");
    }
}

bench_day!(15);
