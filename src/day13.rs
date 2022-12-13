use std::cmp::Ordering;

use crate::day::Day;

pub enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

fn read_packet(mut string: &str) -> (Packet, &str) {
    if string.starts_with('[') {
        string = &string[1..];
        let mut list = Vec::new();
        while !string.starts_with(']') {
            assert!(string.starts_with(',') || list.is_empty());
            if string.starts_with(',') {
                string = &string[1..];
            }
            let (packet, rest) = read_packet(string);
            list.push(packet);
            string = rest;
        }
        (Packet::List(list), &string[1..])
    } else {
        let end = string.find(|c: char| !c.is_ascii_digit()).unwrap();
        (Packet::Num(string[..end].parse().unwrap()), &string[end..])
    }
}

fn cmp(lhs: &Packet, rhs: &Packet) -> Ordering {
    match (lhs, rhs) {
        (Packet::Num(l), Packet::Num(r)) => l.cmp(r),
        (Packet::List(ls), Packet::List(rs)) => {
            for (l, r) in ls.iter().zip(rs.iter()) {
                let c = cmp(l, r);
                if c != Ordering::Equal {
                    return c;
                }
            }
            ls.len().cmp(&rs.len())
        }
        (Packet::Num(l), _) => cmp(&Packet::List(vec![Packet::Num(*l)]), rhs),
        (_, Packet::Num(r)) => cmp(lhs, &Packet::List(vec![Packet::Num(*r)])),
    }
}

pub struct Day13;

impl<'a> Day<'a> for Day13 {
    const DAY: usize = 13;
    type Input = Vec<[Packet; 2]>;
    type ProcessedInput = Vec<[Packet; 2]>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .split("\n\n")
            .map(|chunk| {
                let mut lines = chunk.lines();
                let fst = lines.next().unwrap();
                let snd = lines.next().unwrap();
                [read_packet(fst).0, read_packet(snd).0]
            })
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let ans = input
            .iter()
            .enumerate()
            .filter(|(_, pair)| cmp(&pair[0], &pair[1]) == Ordering::Less)
            .map(|(i, _)| i + 1)
            .sum::<usize>()
            .to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let dividers = [Packet::Num(2), Packet::Num(6)];
        let mut packets = input
            .into_iter()
            .flat_map(|ar| ar.into_iter())
            .map(|p| (p, false))
            .chain(dividers.into_iter().map(|d| (d, true)))
            .collect::<Vec<_>>();
        packets.sort_unstable_by(|l, r| cmp(&l.0, &r.0));
        packets
            .into_iter()
            .enumerate()
            .filter_map(|(i, t)| t.1.then_some(i + 1))
            .product::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod test_day13 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "};

    #[test]
    fn test_day13_examples() {
        let input = Day13::parse(EXAMPLE);
        let (input, part1) = Day13::solve_part1(input);
        let part2 = Day13::solve_part2(input);
        assert_eq!(part1, "13");
        assert_eq!(part2, "140");
    }
}

bench_day!(13);
