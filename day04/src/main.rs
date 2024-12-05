use std::collections::HashMap;
use std::ops::Add;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Coord {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Coord {
    fn from((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }
}

impl Add<(i64, i64)> for Coord {
    type Output = Coord;

    fn add(self, (x, y): (i64, i64)) -> Self::Output {
        Self::Output {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Coord {
    // for 'starting with X'; get the next 3 in all 8 directions
    pub fn rays_from(self) -> Vec<[Coord; 3]> {
        vec![
            [self + (0, 1), self + (0, 2), self + (0, 3)],
            [self + (1, 1), self + (2, 2), self + (3, 3)],
            [self + (1, 0), self + (2, 0), self + (3, 0)],
            [self + (1, -1), self + (2, -2), self + (3, -3)],
            [self + (0, -1), self + (0, -2), self + (0, -3)],
            [self + (-1, -1), self + (-2, -2), self + (-3, -3)],
            [self + (-1, 0), self + (-2, 0), self + (-3, 0)],
            [self + (-1, 1), self + (-2, 2), self + (-3, 3)],
        ]
    }

    // for 'A in the middle'; get the four ways the surrounding corner cells
    // can be arranged with the first two on the same side
    pub fn corners_around(self) -> Vec<[Coord; 4]> {
        vec![
            [
                self + (-1, -1),
                self + (-1, 1),
                self + (1, -1),
                self + (1, 1),
            ],
            [
                self + (1, -1),
                self + (1, 1),
                self + (-1, -1),
                self + (-1, 1),
            ],
            [
                self + (-1, -1),
                self + (1, -1),
                self + (-1, 1),
                self + (1, 1),
            ],
            [
                self + (1, 1),
                self + (-1, 1),
                self + (1, -1),
                self + (-1, -1),
            ],
        ]
    }
}

#[derive(Debug)]
pub struct Input {
    grid: HashMap<Coord, char>,
}

pub fn parse_input(input: &str) -> Input {
    Input {
        grid: input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as i64, y as i64).into(), c))
            })
            .collect(),
    }
}

pub fn part_1(input: &Input) -> usize {
    let mut count = 0;
    for (coord, c) in &input.grid {
        if c != &'X' {
            continue;
        }

        for ray in coord.rays_from() {
            if let (Some('M'), Some('A'), Some('S')) = (
                input.grid.get(&ray[0]),
                input.grid.get(&ray[1]),
                input.grid.get(&ray[2]),
            ) {
                count += 1
            }
        }
    }

    count
}

pub fn part_2(input: &Input) -> usize {
    let mut count = 0;
    for (coord, c) in &input.grid {
        if c != &'A' {
            continue;
        }

        for corners in coord.corners_around() {
            if let (Some('M'), Some('M'), Some('S'), Some('S')) = (
                input.grid.get(&corners[0]),
                input.grid.get(&corners[1]),
                input.grid.get(&corners[2]),
                input.grid.get(&corners[3]),
            ) {
                count += 1
            }
        }
    }

    count
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test() {
    let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;
    let grid = parse_input(input);
    assert_eq!(part_1(&grid), 18);
    assert_eq!(part_2(&grid), 9);
}
