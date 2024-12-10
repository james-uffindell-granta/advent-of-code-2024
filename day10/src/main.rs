use std::collections::{HashMap, HashSet};

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

impl Coord {
    pub fn neighbours(Coord { x, y }: Self) -> [Coord; 4] {
        [
            (x + 1, y).into(),
            (x - 1, y).into(),
            (x, y + 1).into(),
            (x, y - 1).into(),
        ]
    }
}

pub struct Input {
    heights: HashMap<Coord, u32>,
}

impl Input {
    pub fn to_trailheads(&self) -> HashMap<Coord, HashSet<Coord>> {
        let mut trailheads = HashMap::new();
        for coord in self.heights.iter().filter_map(|(c, h)| (*h == 9).then_some(*c)) {
            trailheads.insert(coord, HashSet::from([coord]));
        }

        for height in (0..=8).rev() {
            for coord in self.heights.iter().filter_map(|(c, h)| (*h == height).then_some(*c)) {
                let mut reachable = HashSet::new();
                for neighbour in Coord::neighbours(coord) {
                    if let Some(h) = self.heights.get(&neighbour) {
                        if *h == height + 1 {
                            if let Some(r) = trailheads.get(&neighbour) {
                                reachable.extend(r);
                            }
                        }
                    }
                }

                trailheads.insert(coord, reachable);
            }
        }

        trailheads
    }

    pub fn to_trails(&self) -> HashMap<Coord, HashSet<Vec<Coord>>> {
        let mut result = HashMap::new();
        for coord in self.heights.iter().filter_map(|(c, h)| (*h == 9).then_some(*c)) {
            result.insert(coord, HashSet::from([vec![coord]]));
        }

        for height in (0..=8).rev() {
            for coord in self.heights.iter().filter_map(|(c, h)| (*h == height).then_some(*c)) {
                let mut trails = HashSet::new();
                for neighbour in Coord::neighbours(coord) {
                    if let Some(h) = self.heights.get(&neighbour) {
                        if *h == height + 1 {
                            if let Some(t) = result.get(&neighbour) {
                                for trail in t {
                                    let mut longer_trail = trail.clone();
                                    longer_trail.push(coord);
                                    trails.insert(longer_trail);
                                }
                            }
                        }
                    }
                }

                result.insert(coord, trails);
            }
        }

        result
    }
}

pub fn parse_input(input: &str) -> Input {
    Input {
        heights: input.lines().enumerate().flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                ((x as i64, y as i64).into(), c.to_digit(10).unwrap())
            })
        }).collect()
    }
}

pub fn part_1(input: &Input) -> usize {
    let starts = input.heights.iter().filter_map(|(c, h)| (*h ==  0).then_some(*c)).collect::<HashSet<_>>();
    let trailheads = input.to_trailheads();
    trailheads.iter().filter_map(|(c, set)| starts.contains(c).then_some(set.len())).sum()
}

pub fn part_2(input: &Input) -> usize {
    let starts = input.heights.iter().filter_map(|(c, h)| (*h ==  0).then_some(*c)).collect::<HashSet<_>>();
    let trailheads = input.to_trails();
    trailheads.iter().filter_map(|(c, set)| starts.contains(c).then_some(set.len())).sum()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    let input = parse_input(input);


    assert_eq!(part_1(&input), 36);
    assert_eq!(part_2(&input), 81);
}