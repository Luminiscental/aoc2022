use crate::{
    day::Day,
    util::{Tree, TreeZipper},
};

use std::collections::HashMap;

type Directory<'a> = Tree<&'a str, HashMap<&'a str, usize>>;

pub struct Day07;

impl<'a> Day<'a> for Day07 {
    const DAY: usize = 7;
    type Input = Directory<'a>;
    type ProcessedInput = Vec<usize>;

    fn parse(input: &'a str) -> Self::Input {
        let mut cwd = TreeZipper::new(Directory::default());
        for line in input.lines() {
            if line.starts_with("$ c") {
                match &line[5..] {
                    ".." => cwd.pop(),
                    dir => cwd.push(dir),
                }
            } else if line.starts_with(|c: char| c.is_ascii_digit()) {
                let (size, name) = line.split_once(' ').unwrap();
                cwd.cursor.value.insert(name, size.parse().unwrap());
            }
        }
        cwd.root()
    }

    fn solve_part1(file_system: Self::Input) -> (Self::ProcessedInput, String) {
        let mut sizes = Vec::new();
        file_system.fold(0, &mut |n, m| n + m, &mut |s, fs| {
            let size = s + fs.values().sum::<usize>();
            sizes.push(size);
            size
        });
        let ans = sizes
            .iter()
            .filter(|&&s| s <= 100000)
            .sum::<usize>()
            .to_string();
        (sizes, ans)
    }

    fn solve_part2(sizes: Self::ProcessedInput) -> String {
        let required = 30000000 - (70000000 - sizes.last().unwrap());
        sizes
            .into_iter()
            .filter(|&s| s >= required)
            .min()
            .unwrap()
            .to_string()
    }
}

#[cfg(test)]
mod test_day07 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    fn test_day07_examples() {
        let input = Day07::parse(EXAMPLE);
        let (input, part1) = Day07::solve_part1(input);
        let part2 = Day07::solve_part2(input);
        assert_eq!(part1, "95437");
        assert_eq!(part2, "24933642");
    }
}

bench_day!(07);
