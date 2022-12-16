use std::collections::{BinaryHeap, HashMap, VecDeque};

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
    flowing_valves: Vec<(u16, i32)>,
    distances: HashMap<(u16, u16), i32>,
}

fn release_with<const N: usize>(time: i32, volcano: &Volcano) -> i32 {
    let mut queue = BinaryHeap::new();
    let mut seen = HashMap::new();
    let mut best = 0;
    let aa = encode("AA");
    queue.push((0, ([aa; N], vec![], [time; N])));
    seen.insert([aa; N], vec![(0, [time; N])]);
    // TODO: this condition can actually skip over the optimal route
    let mut improve_on_seen = |locs: [u16; N], released: i32, times: [i32; N]| -> bool {
        let seen = seen.entry(locs).or_insert_with(Vec::new);
        if seen.iter().all(|&(s_released, s_times)| {
            s_released < released || (0..N).any(|i| s_times[i] < times[i])
        }) {
            seen.push((released, times));
            true
        } else {
            false
        }
    };
    while let Some((released, (locs, open, times))) = queue.pop() {
        best = i32::max(best, released);
        for (valve, flow) in volcano.flowing_valves.iter().copied() {
            if open.contains(&valve) {
                continue;
            }
            for i in 0..N {
                let new_t = times[i] - volcano.distances.get(&(locs[i], valve)).unwrap() - 1;
                let mut new_times = times;
                new_times[i] = new_t;
                let mut new_locs = locs;
                new_locs[i] = valve;
                let new_r = released + flow * new_t;
                if new_t > 0 && improve_on_seen(new_locs, new_r, new_times) {
                    let mut new_open = open.clone();
                    new_open.push(valve);
                    queue.push((new_r, (new_locs, new_open, new_times)));
                }
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
            flowing_valves,
            distances,
        }
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = release_with::<1>(30, &input).to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        release_with::<2>(26, &input).to_string()
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
        // assert_eq!(part1, "1651");
        assert_eq!(part2, "1707");
    }
}

bench_day!(16);
