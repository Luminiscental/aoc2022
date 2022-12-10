use crate::day::Day;

fn decode(pixel: impl Fn(usize, usize) -> bool) -> char {
    let pixel01 = |x, y| pixel(x, y) as i32;
    match (pixel01(0, 0), pixel01(3, 0), pixel01(0, 5), pixel01(3, 5)) {
        (0, 0, 0, 0) => 'C',
        (0, 0, 0, 1) => 'G',
        (0, 0, 1, 1) => 'A',
        (0, 1, 0, 0) => 'J',
        (1, 0, 1, 0) if pixel(1, 5) => 'B',
        (1, 0, 1, 0) => 'P',
        (1, 0, 1, 1) if pixel(1, 0) => 'R',
        (1, 0, 1, 1) => 'L',
        (1, 1, 0, 0) => 'U',
        (1, 1, 1, 0) => 'F',
        (1, 1, 1, 1) if pixel(3, 4) => 'H',
        (1, 1, 1, 1) if pixel(1, 3) => 'Z',
        (1, 1, 1, 1) if pixel(1, 0) => 'E',
        (1, 1, 1, 1) => 'K',
        _ => panic!("unknown character"),
    }
}

fn run<F: FnMut(i32, i32) -> bool>(mut program: &[Option<i32>], mut body: F) {
    let mut queue = None;
    let mut x = 1;
    for cycle in 1.. {
        if !body(cycle, x) {
            break;
        }
        if let Some(n) = queue.take() {
            x += n;
        } else {
            queue = program[0];
            program = &program[1..];
        }
        if program.is_empty() {
            break;
        }
    }
}

pub struct Day10;

impl<'a> Day<'a> for Day10 {
    const DAY: usize = 10;
    type Input = Vec<Option<i32>>;
    type ProcessedInput = Vec<Option<i32>>;

    fn parse(input: &'a str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| line.starts_with('a').then(|| line[5..].parse().unwrap()))
            .collect()
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let mut ans = 0;
        run(&input, |cycle, x| {
            if (cycle - 20) % 40 == 0 {
                ans += cycle * x;
            }
            cycle < 220
        });
        (input, ans.to_string())
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let mut image = [[false; 40]; 6];
        run(&input, |cycle, x| {
            let (row, col) = ((cycle - 1) / 40, (cycle - 1) % 40);
            if col.abs_diff(x) <= 1 {
                image[row as usize][col as usize] = true;
            }
            true
        });
        (0..8).map(|i| decode(|x, y| image[y][5 * i + x])).collect()
    }
}

#[cfg(test)]
mod test_day10 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

    #[test]
    fn test_day10_examples() {
        let input = Day10::parse(EXAMPLE);
        let (_, part1) = Day10::solve_part1(input);
        assert_eq!(part1, "13140");
    }
}

bench_day!(10);
