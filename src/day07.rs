use crate::day::Day;

use std::collections::HashMap;

#[derive(Default)]
pub struct Directory<'a> {
    files: HashMap<&'a str, usize>,
    subdirectories: HashMap<&'a str, Directory<'a>>,
}

impl<'a> Directory<'a> {
    fn for_each_size<F: FnMut(usize)>(&self, f: &mut F) -> usize {
        let mut size = 0;
        for dir in self.subdirectories.values() {
            size += dir.for_each_size(f);
        }
        size += self.files.values().sum::<usize>();
        f(size);
        size
    }
}

pub struct Day07;

impl<'a> Day<'a> for Day07 {
    const DAY: usize = 7;
    type Input = Directory<'a>;
    type ProcessedInput = Vec<usize>;

    fn parse(input: &'a str) -> Self::Input {
        let (mut root, mut stack) = (Directory::default(), Vec::default());
        let mut cwd = &mut root;
        for command in input[9..].split("\n$ ") {
            if command.starts_with('c') {
                if &command[3..] == ".." {
                    stack.pop();
                    cwd = &mut root;
                    for d in stack.iter() {
                        cwd = cwd.subdirectories.get_mut(d).unwrap();
                    }
                } else {
                    stack.push(&command[3..]);
                    cwd = cwd.subdirectories.get_mut(&command[3..]).unwrap();
                }
            } else {
                for entry in command[3..].lines() {
                    let (meta, name) = entry.split_once(' ').unwrap();
                    if meta == "dir" {
                        cwd.subdirectories.insert(name, Directory::default());
                    } else {
                        cwd.files.insert(name, meta.parse().unwrap());
                    }
                }
            }
        }
        root
    }

    fn solve_part1(file_system: Self::Input) -> (Self::ProcessedInput, String) {
        let mut sizes = Vec::new();
        file_system.for_each_size(&mut |s| sizes.push(s));
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
