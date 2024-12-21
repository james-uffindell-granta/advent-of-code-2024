use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::Add;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Debug)]
pub struct Input {
    spaces: HashSet<Coord>,
    start: Coord,
    end: Coord,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cheat {
    start: Coord,
    end: Coord,
}

impl Input {
    // map of cheats to savings
    pub fn find_cheats(&self) -> HashMap<Cheat, usize> {
        // first off, find the path
        let mut offsets = HashMap::new();
        let mut path = Vec::new();
        let mut location = self.start;
        path.push(location);
        offsets.insert(location, 0);
        while location != self.end {
            let neighbours = [
                location + (0, 1),
                location + (1, 0),
                location + (0, -1),
                location + (-1, 0),
            ];
            if let Ok(next) = neighbours
                .into_iter()
                .filter(|n| self.spaces.contains(n) && !offsets.contains_key(n))
                .exactly_one()
            {
                path.push(next);
                offsets.insert(next, path.len() - 1);
                location = next;
            } else {
                unreachable!();
            }
        }

        // now we have the path - find the cheats
        let mut cheats = HashMap::new();

        for coord in path {
            let possible_cheats = [
                coord + (0, 2),
                coord + (2, 0),
                coord + (0, -2),
                coord + (-2, 0),
            ];
            for valid_cheat in possible_cheats
                .into_iter()
                .filter(|c| self.spaces.contains(c))
            {
                // going from coord to valid_cheat is legal - does it save us time?
                let start_offset = *offsets.get(&coord).unwrap();
                let end_offset = *offsets.get(&valid_cheat).unwrap();
                if end_offset > start_offset + 2 {
                    // we found a valid cheat
                    cheats.insert(
                        Cheat {
                            start: coord,
                            end: valid_cheat,
                        },
                        end_offset - (start_offset + 2),
                    );
                }
            }
        }

        cheats
    }

    // map of cheats to savings
    pub fn find_better_cheats(&self) -> HashMap<Cheat, usize> {
        // first off, find the path
        let mut offsets = HashMap::new();
        let mut path = Vec::new();
        let mut location = self.start;
        path.push(location);
        offsets.insert(location, 0);
        while location != self.end {
            let neighbours = [
                location + (0, 1),
                location + (1, 0),
                location + (0, -1),
                location + (-1, 0),
            ];
            if let Ok(next) = neighbours
                .into_iter()
                .filter(|n| self.spaces.contains(n) && !offsets.contains_key(n))
                .exactly_one()
            {
                path.push(next);
                offsets.insert(next, path.len() - 1);
                location = next;
            } else {
                unreachable!();
            }
        }

        // now we have the path - find the cheats
        let mut cheats = HashMap::new();

        for coord in path {
            let possible_cheats = self
                .spaces
                .iter()
                .filter(|c| c.x.abs_diff(coord.x) + c.y.abs_diff(coord.y) <= 20);
            for valid_cheat in possible_cheats {
                // going from coord to valid_cheat is legal - does it save us time?
                let start_offset = *offsets.get(&coord).unwrap();
                let end_offset = *offsets.get(valid_cheat).unwrap();
                let cheat_distance =
                    (valid_cheat.x.abs_diff(coord.x) + valid_cheat.y.abs_diff(coord.y)) as usize;
                if end_offset > start_offset + cheat_distance {
                    // we found a valid cheat
                    cheats.insert(
                        Cheat {
                            start: coord,
                            end: *valid_cheat,
                        },
                        end_offset - (start_offset + cheat_distance),
                    );
                }
            }
        }

        cheats
    }
}

pub fn parse_input(input: &str) -> Input {
    let mut spaces = HashSet::new();
    let mut start = (0, 0).into();
    let mut end = (0, 0).into();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = (x as i64, y as i64).into();
            match c {
                '.' => {
                    spaces.insert(coord);
                }
                'S' => {
                    spaces.insert(coord);
                    start = coord;
                }
                'E' => {
                    spaces.insert(coord);
                    end = coord;
                }
                _ => {}
            }
        }
    }

    Input { spaces, start, end }
}

pub fn part_1(input: &Input) -> usize {
    let cheats = input.find_cheats();
    cheats.into_iter().filter(|(_, v)| v >= &100).count()
}

pub fn part_2(input: &Input) -> usize {
    let cheats = input.find_better_cheats();
    cheats.into_iter().filter(|(_, v)| v >= &100).count()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    use std::time::Instant;
    let now = Instant::now();
    println!("Part 1: {}", part_1(&input));
    println!("{:?}", now.elapsed());
    let now = Instant::now();
    println!("Part 2: {}", part_2(&input));
    println!("{:?}", now.elapsed());
}

#[test]
pub fn test() {
    let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";
    let input = parse_input(input);
    let cheats = input.find_cheats();

    let mut cheats_by_savings = HashMap::new();
    for (cheat, saving) in cheats {
        cheats_by_savings
            .entry(saving)
            .or_insert(HashSet::new())
            .insert(cheat);
    }

    for (saving, cheats) in cheats_by_savings {
        println!("{} cheats save {} time", cheats.len(), saving);
    }

    let cheats = input.find_better_cheats();

    let mut cheats_by_savings = HashMap::new();
    for (cheat, saving) in cheats {
        cheats_by_savings
            .entry(saving)
            .or_insert(HashSet::new())
            .insert(cheat);
    }

    for (saving, cheats) in cheats_by_savings {
        if saving >= 50 {
            println!("{} cheats save {} time", cheats.len(), saving);
        }
    }
}
