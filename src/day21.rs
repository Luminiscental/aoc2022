use std::collections::HashMap;

use itertools::Itertools;

use crate::day::Day;

pub struct Day21;

pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

impl Op {
    fn eval(&self, l: i64, r: i64) -> i64 {
        match self {
            Self::Add => l + r,
            Self::Sub => l - r,
            Self::Mul => l * r,
            Self::Div => l / r,
            Self::Eq => unreachable!(),
        }
    }

    fn enforce_left(&self, output: i64, right: i64) -> i64 {
        match self {
            Self::Add => output - right,
            Self::Sub => output + right,
            Self::Mul if output % right == 0 => output / right,
            Self::Div => output * right,
            Self::Eq if output == 1 => right,
            _ => panic!("bad cancellation"),
        }
    }

    fn enforce_right(&self, output: i64, left: i64) -> i64 {
        match self {
            Self::Add => output - left,
            Self::Sub => left - output,
            Self::Mul if output % left == 0 => output / left,
            Self::Div if left % output == 0 => left / output,
            Self::Eq if output == 1 => left,
            _ => panic!("bad cancellation"),
        }
    }
}

pub enum Monkey<'a> {
    Num(i64),
    Op(&'a str, &'a str, Op),
}

impl<'a> Monkey<'a> {
    fn eval_without(&self, monkey: &'a str, monkeys: &HashMap<&'a str, Monkey<'a>>) -> Option<i64> {
        match self {
            Self::Num(n) => Some(*n),
            Self::Op(l, r, o) => (*l != monkey && *r != monkey)
                .then(|| {
                    Some(o.eval(
                        monkeys.get(l).unwrap().eval_without(monkey, monkeys)?,
                        monkeys.get(r).unwrap().eval_without(monkey, monkeys)?,
                    ))
                })
                .flatten(),
        }
    }

    fn enforce_from(
        &self,
        monkey: &'a str,
        output: i64,
        monkeys: &HashMap<&'a str, Monkey<'a>>,
    ) -> i64 {
        let Self::Op(left, right, op) = self else { unreachable!() };
        if *left == monkey {
            return op.enforce_left(
                output,
                monkeys
                    .get(right)
                    .unwrap()
                    .eval_without(monkey, monkeys)
                    .unwrap(),
            );
        } else if *right == monkey {
            return op.enforce_right(
                output,
                monkeys
                    .get(left)
                    .unwrap()
                    .eval_without(monkey, monkeys)
                    .unwrap(),
            );
        }
        let (left, right) = (monkeys.get(left).unwrap(), monkeys.get(right).unwrap());
        if let Some(n) = left.eval_without(monkey, monkeys) {
            right.enforce_from(monkey, op.enforce_right(output, n), monkeys)
        } else if let Some(m) = right.eval_without(monkey, monkeys) {
            left.enforce_from(monkey, op.enforce_left(output, m), monkeys)
        } else {
            panic!("complicated equation")
        }
    }
}

impl<'a> Day<'a> for Day21 {
    const DAY: usize = 21;
    type Input = HashMap<&'a str, Monkey<'a>>;
    type ProcessedInput = HashMap<&'a str, Monkey<'a>>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| {
                let (name, job) = line.split_once(": ").unwrap();
                let monkey = if job.starts_with(|c: char| c.is_ascii_digit()) {
                    Monkey::Num(job.parse().unwrap())
                } else {
                    let (lhs, op, rhs) = job.split_ascii_whitespace().collect_tuple().unwrap();
                    Monkey::Op(
                        lhs,
                        rhs,
                        match op {
                            "+" => Op::Add,
                            "-" => Op::Sub,
                            "*" => Op::Mul,
                            "/" => Op::Div,
                            _ => panic!("unknown operation {op}"),
                        },
                    )
                };
                (name, monkey)
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = input
            .get("root")
            .unwrap()
            .eval_without("", &input)
            .unwrap()
            .to_string();
        (input, ans)
    }

    fn solve_part2(mut input: Self::ProcessedInput) -> String {
        let Monkey::Op(_, _, op) = input.get_mut("root").unwrap() else { unreachable!() };
        *op = Op::Eq;
        input
            .get("root")
            .unwrap()
            .enforce_from("humn", 1, &input)
            .to_string()
    }
}

#[cfg(test)]
mod test_day21 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
    "};

    #[test]
    fn test_day21_examples() {
        let input = Day21::parse(EXAMPLE);
        let (input, part1) = Day21::solve_part1(input);
        let part2 = Day21::solve_part2(input);
        assert_eq!(part1, "152");
        assert_eq!(part2, "301");
    }
}

bench_day!(21);
