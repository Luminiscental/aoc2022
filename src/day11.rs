use crate::day::Day;

#[derive(Clone)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn apply(&self, value: u64) -> u64 {
        match self {
            Self::Add(n) => value + n,
            Self::Mul(n) => value * n,
            Self::Square => value * value,
        }
    }
}

fn read_operation(string: &str) -> Operation {
    assert!(string.starts_with("new = old "));
    match (&string[10..12], &string[12..]) {
        ("+ ", "old") => Operation::Mul(2),
        ("* ", "old") => Operation::Square,
        ("+ ", n) => Operation::Add(n.parse().unwrap()),
        ("* ", n) => Operation::Mul(n.parse().unwrap()),
        _ => panic!("couldn't read operation"),
    }
}

#[derive(Clone)]
pub struct Monkey {
    queue: Vec<u64>,
    inspections: usize,
    operation: Operation,
    test_base: u64,
    divisible_target: usize,
    nondivisible_target: usize,
}

fn monkey_business(monkeys: &mut [Monkey], rounds: usize, manageable: bool) -> usize {
    let global_base = monkeys.iter().map(|m| m.test_base).product::<u64>();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            for item in std::mem::take(&mut monkeys[i].queue).into_iter() {
                let item = monkeys[i].operation.apply(item);
                let item = if manageable { item / 3 } else { item % global_base };
                let target = if item % monkeys[i].test_base == 0 {
                    monkeys[i].divisible_target
                } else {
                    monkeys[i].nondivisible_target
                };
                monkeys[i].inspections += 1;
                monkeys[target].queue.push(item);
            }
        }
    }
    let mut counts = monkeys.iter().map(|m| m.inspections).collect::<Vec<_>>();
    counts.sort();
    counts.into_iter().rev().take(2).product()
}

pub struct Day11;

impl<'a> Day<'a> for Day11 {
    const DAY: usize = 11;
    type Input = Vec<Monkey>;
    type ProcessedInput = Vec<Monkey>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .split("\n\n")
            .map(|chunk| {
                let mut lines = chunk.lines().skip(1);
                Monkey {
                    queue: lines.next().unwrap()[18..]
                        .split(", ")
                        .map(|n| n.parse().unwrap())
                        .collect(),
                    inspections: 0,
                    operation: read_operation(&lines.next().unwrap()[13..]),
                    test_base: lines.next().unwrap()[21..].parse().unwrap(),
                    divisible_target: lines.next().unwrap()[29..].parse().unwrap(),
                    nondivisible_target: lines.next().unwrap()[30..].parse().unwrap(),
                }
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut monkeys = input.clone();
        let ans = monkey_business(&mut monkeys, 20, true).to_string();
        (input, ans)
    }

    fn solve_part2(mut monkeys: Self::ProcessedInput) -> String {
        monkey_business(&mut monkeys, 10000, false).to_string()
    }
}

#[cfg(test)]
mod test_day11 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    "};

    #[test]
    fn test_day11_examples() {
        let input = Day11::parse(EXAMPLE);
        let (input, part1) = Day11::solve_part1(input);
        let part2 = Day11::solve_part2(input);
        assert_eq!(part1, "10605");
        assert_eq!(part2, "2713310158");
    }
}

bench_day!(11);
