use std::collections::VecDeque;

use hashbrown::HashMap;
use itertools::iproduct;

use crate::day::Day;

fn get_distances<'a>(
    valves: &HashMap<&'a str, (i32, Vec<&'a str>)>,
) -> HashMap<(&'a str, &'a str), i32> {
    let mut distances = HashMap::new();
    for source in valves.keys().copied() {
        let mut queue = VecDeque::new();
        queue.push_front((source, 0));
        distances.insert((source, source), 0);
        while let Some((target, dist)) = queue.pop_back() {
            for neighb in valves.get(&target).unwrap().1.iter().copied() {
                distances.entry((source, neighb)).or_insert_with(|| {
                    queue.push_front((neighb, dist + 1));
                    dist + 1
                });
            }
        }
    }
    distances
}

pub struct Volcano {
    start: u32,
    valves: Vec<(u32, i32)>,
    distances: HashMap<(u32, u32), i32>,
}

fn potential(time: i32, open: u32, valves: &[(u32, i32)]) -> i32 {
    let release_times = (1..time - 1).rev().step_by(2);
    let valves = valves
        .iter()
        .filter_map(|(v, f)| (open & v == 0).then_some(f));
    release_times.zip(valves).map(|(t, f)| t * f).sum()
}

fn max_releases(time: i32, volcano: &Volcano) -> HashMap<u32, i32> {
    let mut releases = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_front((0, (volcano.start, 0, time)));
    while let Some((released, (loc, open, time))) = queue.pop_back() {
        let seen = releases.entry(open).or_insert(0);
        *seen = i32::max(*seen, released);
        for (valve, flow) in volcano.valves.iter().copied() {
            let bit = 1 << valve;
            if flow == 0 || bit & open != 0 {
                continue;
            }
            let new_time = time - volcano.distances.get(&(loc, valve)).unwrap() - 1;
            let new_open = open | bit;
            let new_released = released + flow * new_time;
            if new_time > 0
                && releases.get(&new_open).map_or(true, |&r| {
                    r < new_released + potential(new_time, new_open, &volcano.valves)
                })
            {
                queue.push_front((new_released, (valve, new_open, new_time)));
            }
        }
    }
    releases
}

pub struct Day16;

impl<'a> Day<'a> for Day16 {
    const DAY: usize = 16;
    type Input = Volcano;
    type ProcessedInput = Volcano;

    fn parse(input: &'a str) -> Self::Input {
        let graph = input
            .trim()
            .lines()
            .map(|line| {
                let (valve, tunnels) = line.split_once(';').unwrap();
                let flow = valve[23..].parse().unwrap();
                let s = tunnels.find(|c: char| c.is_ascii_uppercase()).unwrap();
                let tunnels = tunnels[s..].split(", ").collect();
                (&valve[6..8], (flow, tunnels))
            })
            .collect::<HashMap<_, _>>();
        let distances = get_distances(&graph);
        let valves = graph
            .iter()
            .filter_map(|(k, v)| (*k == "AA" || v.0 != 0).then_some(*k))
            .collect::<Vec<_>>();
        let flag = |valve| valves.iter().position(|&v| v == valve).unwrap() as u32;
        let distances = distances
            .into_iter()
            .filter_map(|((k1, k2), d)| {
                (valves.contains(&k1) && valves.contains(&k2)).then(|| ((flag(k1), flag(k2)), d))
            })
            .collect();
        let mut valves = valves
            .iter()
            .map(|valve| (flag(valve), graph.get(valve).unwrap().0))
            .collect::<Vec<_>>();
        valves.sort_by_key(|&(_, f)| -f);
        let start = flag("AA");
        Volcano {
            start,
            valves,
            distances,
        }
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = max_releases(30, &input)
            .into_values()
            .max()
            .unwrap()
            .to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let releases = max_releases(26, &input);
        iproduct!(releases.iter(), releases.iter())
            .filter_map(|((o1, r1), (o2, r2))| ((o1 & o2) == 0).then_some(r1 + r2))
            .max()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test_day16 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    "};

    #[test]
    fn test_day16_examples() {
        let input = Day16::parse(EXAMPLE);
        let (input, part1) = Day16::solve_part1(input);
        let part2 = Day16::solve_part2(input);
        assert_eq!(part1, "1651");
        assert_eq!(part2, "1707");
    }
}

bench_day!(16);
