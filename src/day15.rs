use std::collections::HashSet;

use crate::day::Day;

fn intersect(int1: (i32, i32), int2: (i32, i32)) -> Option<(i32, i32)> {
    let left = int1.0.max(int2.0);
    let right = int1.1.min(int2.1);
    (left <= right).then_some((left, right))
}

fn merge(int1: (i32, i32), int2: (i32, i32)) -> Option<(i32, i32)> {
    if int2.0 - 1 <= int1.1 && int1.1 <= int2.1 {
        Some((int1.0.min(int2.0), int2.1))
    } else if int2.0 <= int1.0 && int1.0 <= int2.1 + 1 {
        Some((int2.0, int1.1.max(int2.1)))
    } else if int1.0 - 1 <= int2.1 && int2.1 <= int1.1 {
        Some((int2.0.min(int1.0), int1.1))
    } else if int1.0 <= int2.0 && int2.0 <= int1.1 + 1 {
        Some((int1.0, int2.1.max(int1.1)))
    } else {
        None
    }
}

fn combine_intervals(intervals: &mut Vec<(i32, i32)>) {
    let mut progress = true;
    while progress {
        progress = false;
        'search: for i in 1..intervals.len() {
            for j in 0..i {
                if let Some(merged) = merge(intervals[i], intervals[j]) {
                    intervals[j] = merged;
                    intervals.swap_remove(i);
                    progress = true;
                    break 'search;
                }
            }
        }
    }
}

fn range(sensor: (i32, i32), beacon: (i32, i32), y: i32) -> Option<(i32, i32)> {
    let dist = sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1);
    (sensor.1.abs_diff(y) <= dist).then(|| {
        let radius = dist - sensor.1.abs_diff(y);
        (sensor.0 - radius as i32, sensor.0 + radius as i32)
    })
}

pub struct Day15Generic<const ROW: i32>;
pub type Day15 = Day15Generic<2000000>;

impl<'a, const ROW: i32> Day<'a> for Day15Generic<ROW> {
    const DAY: usize = 15;
    type Input = Vec<((i32, i32), (i32, i32))>;
    type ProcessedInput = Vec<((i32, i32), (i32, i32))>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| {
                let (sensor, beacon) = line.split_once(':').unwrap();
                let (sx, sy) = sensor[10..].split_once(',').unwrap();
                let (bx, by) = beacon[22..].split_once(',').unwrap();
                (
                    (sx[2..].parse().unwrap(), sy[3..].parse().unwrap()),
                    (bx[2..].parse().unwrap(), by[3..].parse().unwrap()),
                )
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut intervals = input
            .iter()
            .filter_map(|&(sensor, beacon)| range(sensor, beacon, ROW))
            .collect();
        combine_intervals(&mut intervals);
        let beacons = input
            .iter()
            .filter_map(|&(_, beacon)| (beacon.1 == ROW).then_some(beacon))
            .collect::<HashSet<_>>();
        let obstructed = intervals.into_iter().map(|int| 1 + int.1 - int.0).sum::<i32>();
        let ans = obstructed - beacons.len() as i32;
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        for y in 0..2 * ROW {
            let mut intervals = input
                .iter()
                .filter_map(|&(sensor, beacon)| range(sensor, beacon, y))
                .filter_map(|interval| intersect(interval, (0, 2 * ROW)))
                .collect();
            combine_intervals(&mut intervals);
            if intervals.len() != 1 {
                intervals.sort_by_key(|int| int.0);
                assert_eq!(intervals[0].1 + 1, intervals[1].0 - 1);
                let x = intervals[0].1 + 1;
                return (4000000 * x as u64 + y as u64).to_string();
            }
        }
        panic!("no gap found")
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
