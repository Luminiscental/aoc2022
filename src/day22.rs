use std::collections::{HashMap, HashSet, VecDeque};

use crate::day::Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Step {
    Move(u32),
    Turn(i32),
}

pub struct Input {
    start: usize,
    width: usize,
    height: usize,
    faces: HashMap<(usize, usize), Vec<bool>>,
    path: Vec<Step>,
}

type Vec3 = [i32; 3];

// (tile, side) -> (connected_tile, flip, connected_side)
type Connections = HashMap<((usize, usize), Dir), ((usize, usize), bool, Dir)>;

// normal -> (tile, x_axis, y_axis)
type Orientations = HashMap<Vec3, ((usize, usize), Vec3, Vec3)>;

impl Dir {
    fn shift(self, pos: (i32, i32)) -> (i32, i32) {
        let delta = match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        };
        (pos.0 + delta.0, pos.1 + delta.1)
    }

    fn rotate(self, normal: Vec3, x_axis: Vec3, y_axis: Vec3) -> (Vec3, Vec3, Vec3) {
        let transf = |v0, v1, v2| {
            [
                v0 * x_axis[0] + v1 * y_axis[0] + v2 * normal[0],
                v0 * x_axis[1] + v1 * y_axis[1] + v2 * normal[1],
                v0 * x_axis[2] + v1 * y_axis[2] + v2 * normal[2],
            ]
        };
        match self {
            Self::Up => (transf(0, 1, 0), transf(1, 0, 0), transf(0, 0, -1)),
            Self::Down => (transf(0, -1, 0), transf(1, 0, 0), transf(0, 0, 1)),
            Self::Left => (transf(1, 0, 0), transf(0, 0, -1), transf(0, 1, 0)),
            Self::Right => (transf(-1, 0, 0), transf(0, 0, 1), transf(0, 1, 0)),
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn score(self) -> i32 {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }

    fn turn(self, sign: i32) -> Self {
        match (self, sign) {
            (Self::Up, -1) => Self::Left,
            (Self::Up, 1) => Self::Right,
            (Self::Down, -1) => Self::Right,
            (Self::Down, 1) => Self::Left,
            (Self::Left, -1) => Self::Down,
            (Self::Left, 1) => Self::Up,
            (Self::Right, -1) => Self::Up,
            (Self::Right, 1) => Self::Down,
            _ => panic!("invalid sign {sign}"),
        }
    }

    fn wrap(self, at: Dir, flip: bool, pos: (usize, usize), dim: usize) -> (usize, usize) {
        let pos = if flip {
            (dim - 1 - pos.0, dim - 1 - pos.1)
        } else {
            pos
        };
        match (self, at) {
            (Self::Up, Self::Up) => (pos.0, 0),
            (Self::Up, Self::Down) => (pos.0, dim - 1),
            (Self::Up, Self::Left) => (0, pos.0),
            (Self::Up, Self::Right) => (dim - 1, pos.0),
            (Self::Down, Self::Up) => (pos.0, 0),
            (Self::Down, Self::Down) => (pos.0, dim - 1),
            (Self::Down, Self::Left) => (0, pos.0),
            (Self::Down, Self::Right) => (dim - 1, pos.0),
            (Self::Left, Self::Up) => (pos.1, 0),
            (Self::Left, Self::Down) => (pos.1, dim - 1),
            (Self::Left, Self::Left) => (0, pos.1),
            (Self::Left, Self::Right) => (dim - 1, pos.1),
            (Self::Right, Self::Up) => (pos.1, 0),
            (Self::Right, Self::Down) => (pos.1, dim - 1),
            (Self::Right, Self::Left) => (0, pos.1),
            (Self::Right, Self::Right) => (dim - 1, pos.1),
        }
    }
}

fn fold_net(faces: &HashMap<(usize, usize), Vec<bool>>) -> Orientations {
    let bottom = faces.keys().copied().next().unwrap();
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut orientations = HashMap::new();
    queue.push_front((bottom, ([0, 0, 1], [1, 0, 0], [0, 1, 0])));
    seen.insert(bottom);
    orientations.insert([0, 0, 1], (bottom, [1, 0, 0], [0, 1, 0]));
    while let Some((tile, (normal, x_axis, y_axis))) = queue.pop_back() {
        for d in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let nt = d.shift((tile.0 as i32, tile.1 as i32));
            if nt.0 < 0 || nt.1 < 0 {
                continue;
            }
            let nt = (nt.0 as usize, nt.1 as usize);
            if !seen.contains(&nt) && faces.contains_key(&nt) {
                let (new_normal, new_x_axis, new_y_axis) = d.rotate(normal, x_axis, y_axis);
                queue.push_front((nt, (new_normal, new_x_axis, new_y_axis)));
                seen.insert(nt);
                orientations.insert(new_normal, (nt, new_x_axis, new_y_axis));
            }
        }
    }
    orientations
}

fn cube_connections(orientations: &Orientations) -> Connections {
    let mut connections = HashMap::new();
    for normal in [
        [0, 0, 1],
        [0, 0, -1],
        [1, 0, 0],
        [-1, 0, 0],
        [0, 1, 0],
        [0, -1, 0],
    ] {
        let &(tile, x_axis, y_axis) = orientations.get(&normal).unwrap();
        for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let (n_normal, n_rel_x_axis, _n_rel_y_axis) = dir.rotate(normal, x_axis, y_axis);
            let &(n_tile, n_x_axis, n_y_axis) = orientations.get(&n_normal).unwrap();
            let dot = |v1: [i32; 3], v2: [i32; 3]| v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2];
            let n_dir = match (dot(n_rel_x_axis, n_x_axis), dot(n_rel_x_axis, n_y_axis)) {
                (1, 0) => dir.opposite(),
                (-1, 0) => dir,
                (0, 1) => dir.turn(-1),
                (0, -1) => dir.turn(1),
                _ => unreachable!(),
            };
            let flip = [
                dot(x_axis, n_x_axis),
                dot(x_axis, n_y_axis),
                dot(y_axis, n_x_axis),
                dot(y_axis, n_y_axis),
            ]
            .contains(&-1);
            connections.insert((tile, dir), (n_tile, flip, n_dir));
        }
    }
    connections
}

fn score(input: &Input, connections: &Connections, dim: usize) -> i32 {
    let mut tile = (input.start, 0);
    let mut pos = (0, 0);
    let mut facing = Dir::Right;
    for step in input.path.iter().copied() {
        match step {
            Step::Turn(s) => facing = facing.turn(s),
            Step::Move(n) => {
                for _ in 0..n {
                    let next = facing.shift((pos.0 as i32, pos.1 as i32));
                    let cross = if next.0 == -1 {
                        Some(Dir::Left)
                    } else if next.1 == -1 {
                        Some(Dir::Up)
                    } else if next.0 == dim as i32 {
                        Some(Dir::Right)
                    } else if next.1 == dim as i32 {
                        Some(Dir::Down)
                    } else {
                        None
                    };
                    let (new_tile, new_facing, new_pos) = if let Some(dir) = cross {
                        let (conn, sign, at) = *connections.get(&(tile, dir)).unwrap();
                        (conn, at.opposite(), dir.wrap(at, sign, pos, dim))
                    } else {
                        (tile, facing, (next.0 as usize, next.1 as usize))
                    };
                    if !input.faces.get(&new_tile).unwrap()[new_pos.0 + new_pos.1 * dim] {
                        tile = new_tile;
                        pos = new_pos;
                        facing = new_facing;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    1000 * (tile.1 * dim + pos.1 + 1) as i32
        + 4 * (tile.0 * dim + pos.0 + 1) as i32
        + facing.score()
}

pub struct Day22Generic<const N: usize>;
pub type Day22 = Day22Generic<50>;

impl<'a, const N: usize> Day<'a> for Day22Generic<N> {
    const DAY: usize = 22;
    type Input = Input;
    type ProcessedInput = Input;

    fn parse(input: &'a str) -> Self::Input {
        let lines = input.trim_end().lines().collect::<Vec<_>>();
        let height = lines.len() - 2;
        let width = lines[..height].iter().map(|s| s.len()).max().unwrap();
        let (width, height) = (width / N, height / N);
        let mut faces = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                if lines[y * N].chars().nth(x * N).map_or(true, |c| c == ' ') {
                    continue;
                }
                let face = lines
                    .iter()
                    .skip(y * N)
                    .take(N)
                    .flat_map(|line| line.chars().skip(x * N).take(N).map(|c| c == '#'))
                    .collect();
                faces.insert((x, y), face);
            }
        }
        let start = (0..width).find(|&x| faces.contains_key(&(x, 0))).unwrap();
        let mut path = Vec::new();
        let mut path_string = *lines.last().unwrap();
        while !path_string.is_empty() {
            match &path_string[..1] {
                "L" => {
                    path.push(Step::Turn(-1));
                    path_string = &path_string[1..];
                }
                "R" => {
                    path.push(Step::Turn(1));
                    path_string = &path_string[1..];
                }
                _ => {
                    let end = path_string
                        .find(|c: char| !c.is_ascii_digit())
                        .unwrap_or(path_string.len());
                    path.push(Step::Move(path_string[..end].parse().unwrap()));
                    path_string = &path_string[end..];
                }
            }
        }
        Input {
            start,
            width,
            height,
            faces,
            path,
        }
    }

    fn solve_part1(input: Self::Input) -> (Self::ProcessedInput, String) {
        let wrap = |pos: (i32, i32)| -> (usize, usize) {
            if pos.0 == -1 {
                (input.width - 1, pos.1 as usize)
            } else if pos.1 == -1 {
                (pos.0 as usize, input.height - 1)
            } else if pos.0 == input.width as i32 {
                (0, pos.1 as usize)
            } else if pos.1 == input.height as i32 {
                (pos.0 as usize, 0)
            } else {
                (pos.0 as usize, pos.1 as usize)
            }
        };
        let mut connections = HashMap::new();
        for pos in input.faces.keys().copied() {
            for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                let mut conn = wrap(dir.shift((pos.0 as i32, pos.1 as i32)));
                while !input.faces.contains_key(&conn) {
                    conn = wrap(dir.shift((conn.0 as i32, conn.1 as i32)));
                }
                connections.insert((pos, dir), (conn, false, dir.opposite()));
            }
        }
        let ans = score(&input, &connections, N).to_string();
        (input, ans)
    }

    fn solve_part2(input: Self::ProcessedInput) -> String {
        let orientations = fold_net(&input.faces);
        let connections = cube_connections(&orientations);
        score(&input, &connections, N).to_string()
    }
}

#[cfg(test)]
mod test_day22 {
    use super::*;
    use indoc::indoc;

    const EXAMPLE: &str = indoc! {"
                ...#
                .#..
                #...
                ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
                ...#....
                .....#..
                .#......
                ......#.

        10R5L5R10L4R5L5
    "};

    #[test]
    fn test_day22_examples() {
        let input = Day22Generic::<4>::parse(EXAMPLE);
        let (input, part1) = Day22Generic::<4>::solve_part1(input);
        let part2 = Day22Generic::<4>::solve_part2(input);
        assert_eq!(part1, "6032");
        assert_eq!(part2, "5031");
    }
}

bench_day!(22);
