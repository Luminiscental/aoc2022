use std::collections::VecDeque;

use hashbrown::HashMap;
use regex::Regex;

use crate::day::Day;

pub struct Blueprint {
    id: i32,
    ore: i32,
    clay: i32,
    obsidian: (i32, i32),
    geode: (i32, i32),
}

impl Blueprint {
    fn max_geodes(&self, time: i32) -> i32 {
        let ore_cost = (self.ore)
            .max(self.clay)
            .max(self.obsidian.0)
            .max(self.geode.0);
        let mut queue = VecDeque::new();
        let mut seen = HashMap::new();
        let mut best = 0;
        queue.push_front(State {
            bots: (1, 0, 0, 0),
            resources: (0, 0, 0, 0),
            time,
        });
        seen.insert((1, 0, 0, 0), vec![((0, 0, 0, 0), time)]);
        while let Some(state) = queue.pop_back() {
            best = i32::max(best, state.resources.3);
            if state.time == 0 {
                continue;
            }
            state.for_moves(self, |state| {
                let max_ore = state.resources.0
                    + state.bots.0 * state.time
                    + state.time * (state.time - 1) / 2;
                let max_clay = state.resources.1
                    + state.bots.1 * state.time
                    + state.time * (state.time - 1) / 2;
                let max_obsidian_bots = (max_ore / self.obsidian.0)
                    .min(max_clay / self.obsidian.1)
                    .min(state.time - 1);
                let max_obsidian = state.resources.2
                    + state.bots.2 * state.time
                    + max_obsidian_bots * (max_obsidian_bots + 1) / 2;
                let max_geode_bots = (max_ore / self.geode.0)
                    .min(max_obsidian / self.geode.1)
                    .min(state.time - 1);
                let max_geodes = state.resources.3
                    + state.bots.3 * state.time
                    + max_geode_bots * (max_geode_bots + 1) / 2;
                let seen = seen.entry(state.bots).or_insert_with(Vec::new);
                if state.bots.0 <= ore_cost
                    && state.bots.1 <= self.obsidian.1
                    && state.bots.2 <= self.geode.1
                    && best < max_geodes
                    && seen.iter().all(|&(s_resources, s_time)| {
                        s_resources.3 < max_geodes
                            && (s_resources.0 < state.resources.0
                                || s_resources.1 < state.resources.1
                                || s_resources.2 < state.resources.2
                                || s_resources.3 < state.resources.3
                                || s_time < state.time)
                    })
                {
                    queue.push_front(state);
                    seen.push((state.resources, state.time));
                }
            });
        }
        best
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    bots: (i32, i32, i32, i32),
    resources: (i32, i32, i32, i32),
    time: i32,
}

impl State {
    fn for_moves<F>(self, bp: &Blueprint, mut f: F)
    where
        F: FnMut(State),
    {
        let time = self.time - 1;
        let resources = (
            self.resources.0 + self.bots.0,
            self.resources.1 + self.bots.1,
            self.resources.2 + self.bots.2,
            self.resources.3 + self.bots.3,
        );
        if self.resources.0 >= bp.geode.0 && self.resources.2 >= bp.geode.1 {
            let (mut bots, mut resources) = (self.bots, resources);
            bots.3 += 1;
            resources.0 -= bp.geode.0;
            resources.2 -= bp.geode.1;
            f(State {
                bots,
                resources,
                time,
            });
        }
        if self.resources.0 >= bp.obsidian.0 && self.resources.1 >= bp.obsidian.1 {
            let (mut bots, mut resources) = (self.bots, resources);
            bots.2 += 1;
            resources.0 -= bp.obsidian.0;
            resources.1 -= bp.obsidian.1;
            f(State {
                bots,
                resources,
                time,
            });
        }
        if self.resources.0 >= bp.clay {
            let (mut bots, mut resources) = (self.bots, resources);
            bots.1 += 1;
            resources.0 -= bp.clay;
            f(State {
                bots,
                resources,
                time,
            });
        }
        if self.resources.0 >= bp.ore {
            let (mut bots, mut resources) = (self.bots, resources);
            bots.0 += 1;
            resources.0 -= bp.ore;
            f(State {
                bots,
                resources,
                time,
            });
        }
        f(State {
            resources,
            bots: self.bots,
            time,
        });
    }
}

pub struct Day19;

impl<'a> Day<'a> for Day19 {
    const DAY: usize = 19;
    type Input = Vec<Blueprint>;
    type ProcessedInput = Vec<Blueprint>;

    fn parse(input: &'a str) -> Self::Input {
        let bp_regex = Regex::new(concat!(
                r"^Blueprint (?P<id>\d+): ",
                r"Each ore robot costs (?P<ore>\d+) ore. ",
                r"Each clay robot costs (?P<clay>\d+) ore. ",
                r"Each obsidian robot costs (?P<obsidian_ore>\d+) ore and (?P<obsidian_clay>\d+) clay. ",
                r"Each geode robot costs (?P<geode_ore>\d+) ore and (?P<geode_obsidian>\d+) obsidian.$"
            )).unwrap();
        input
            .trim()
            .lines()
            .map(|line| {
                let captures = bp_regex.captures(line).unwrap();
                let capture = |name| captures.name(name).unwrap().as_str().parse().unwrap();
                Blueprint {
                    id: capture("id"),
                    ore: capture("ore"),
                    clay: capture("clay"),
                    obsidian: (capture("obsidian_ore"), capture("obsidian_clay")),
                    geode: (capture("geode_ore"), capture("geode_obsidian")),
                }
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = input
            .iter()
            .map(|bp| bp.id * bp.max_geodes(24))
            .sum::<i32>()
            .to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        input
            .iter()
            .take(3)
            .map(|bp| bp.max_geodes(32))
            .product::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod test_day19 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
        Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
    "};

    #[test]
    fn test_day19_examples() {
        let input = Day19::parse(EXAMPLE);
        let (input, part1) = Day19::solve_part1(input);
        assert_eq!(part1, "33");
        assert_eq!(input[1].max_geodes(32), 62);
    }
}

bench_day!(19);
