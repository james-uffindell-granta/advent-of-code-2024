use gcd::Gcd;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

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

impl Sub<(i64, i64)> for Coord {
    type Output = Coord;

    fn sub(self, (x, y): (i64, i64)) -> Self::Output {
        Self::Output {
            x: self.x - x,
            y: self.y - y,
        }
    }
}

impl Sub<Coord> for Coord {
    type Output = (i64, i64);

    fn sub(self, Coord { x, y }: Coord) -> Self::Output {
        (self.x - x, self.y - y)
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    antenna_locations: HashMap<char, Vec<Coord>>,
    area_bounds: Coord,
}

impl Input {
    pub fn contains(&self, other: Coord) -> bool {
        other.x >= 0
            && other.y >= 0
            && other.x <= self.area_bounds.x
            && other.y <= self.area_bounds.y
    }
}

pub fn parse_input(input: &str) -> Input {
    let mut area_bounds = (0, 0).into();
    let mut antenna_locations = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let current_location = (x as i64, y as i64).into();
            area_bounds = current_location;
            if c.is_ascii_alphanumeric() {
                antenna_locations
                    .entry(c)
                    .or_insert(Vec::new())
                    .push(current_location);
            }
        }
    }

    Input {
        antenna_locations,
        area_bounds,
    }
}

pub fn part_1(input: &Input) -> usize {
    input
        .antenna_locations
        .values()
        .flat_map(|antennas| antennas.iter().tuple_combinations())
        .flat_map(|(&first, &second)| {
            let distance = second - first;
            [first - distance, second + distance]
        })
        .filter(|node| input.contains(*node))
        .collect::<HashSet<_>>()
        .len()
}

pub fn part_2(input: &Input) -> usize {
    input
        .antenna_locations
        .values()
        .flat_map(|antennas| antennas.iter().tuple_combinations())
        .flat_map(|(&first, &second)| {
            let (dx, dy) = second - first;
            let gcd = dx.unsigned_abs().gcd(dy.unsigned_abs()) as i64;
            let (dx, dy) = (dx / gcd, dy / gcd);

            (0..)
                .map(move |n| first - (n * dx, n * dy))
                .take_while(|c| input.contains(*c))
                .chain(
                    (0..)
                        .map(move |n| second + (n * dx, n * dy))
                        .take_while(|c| input.contains(*c)),
                )
        })
        .filter(|node| input.contains(*node))
        .collect::<HashSet<_>>()
        .len()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test() {
    let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;

    let input = parse_input(input);
    assert_eq!(part_1(&input), 14);
    assert_eq!(part_2(&input), 34);
}

#[test]
pub fn test2() {
    let input = r#"T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........
"#;

    let input = parse_input(input);
    assert_eq!(part_2(&input), 9);
}
