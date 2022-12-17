use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::day::Day;

fn encode(s: &str) -> u16 {
    assert_eq!(s.len(), 2);
    u16::from_le_bytes([s.as_bytes()[0], s.as_bytes()[1]])
}

fn get_distances(valves: &HashMap<u16, (i32, Vec<u16>)>) -> HashMap<(u16, u16), i32> {
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
    valves: Vec<(u16, i32)>,
    distances: HashMap<(u16, u16), i32>,
}

fn max_release(time: i32, valves: &[(u16, i32)], distances: &HashMap<(u16, u16), i32>) -> i32 {
    let mut queue = VecDeque::new();
    let mut seen = HashMap::new();
    let mut best = 0;
    queue.push_front((0, (encode("AA"), vec![], time)));
    seen.insert(encode("AA"), vec![(0, vec![], time)]);
    let mut improve_on_seen = |loc: u16, released: i32, open: Vec<u16>, time: i32| -> bool {
        let seen = seen.entry(loc).or_insert_with(Vec::new);
        if seen.iter().all(|(s_released, s_open, s_time)| {
            *s_released < released || s_open.iter().any(|v| !open.contains(v)) || *s_time < time
        }) {
            seen.push((released, open, time));
            true
        } else {
            false
        }
    };
    while let Some((released, (loc, open, time))) = queue.pop_back() {
        best = i32::max(best, released);
        for (valve, flow) in valves.iter().copied() {
            if open.contains(&valve) {
                continue;
            }
            let new_time = time - distances.get(&(loc, valve)).unwrap() - 1;
            let mut new_open = open.clone();
            new_open.push(valve);
            let new_released = released + flow * new_time;
            if new_time > 0 && improve_on_seen(valve, new_released, new_open.clone(), new_time) {
                queue.push_front((new_released, (valve, new_open, new_time)));
            }
        }
    }
    best
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
                let tunnels = tunnels[s..].split(", ").map(encode).collect();
                (encode(&valve[6..8]), (flow, tunnels))
            })
            .collect::<HashMap<_, _>>();
        let distances = get_distances(&graph);
        let flowing_valves = graph
            .into_iter()
            .filter_map(|(valve, (flow, _))| (flow != 0).then_some((valve, flow)))
            .collect();
        Volcano {
            valves: flowing_valves,
            distances,
        }
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = max_release(30, &input.valves, &input.distances).to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let parts = input
            .valves
            .iter()
            .copied()
            .powerset()
            .map(|fv| max_release(26, &fv, &input.distances))
            .collect_vec();
        parts
            .iter()
            .zip(parts.iter().rev())
            .map(|(a, b)| a + b)
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
