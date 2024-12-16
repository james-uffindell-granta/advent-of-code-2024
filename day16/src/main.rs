use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
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

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    location: Coord,
    direction: Direction,
}

impl Position {
    pub fn neighbours(self) -> [Position; 3] {
        match self.direction {
            Direction::Up => [
                Position {
                    location: self.location,
                    direction: Direction::Left,
                },
                Position {
                    location: self.location,
                    direction: Direction::Right,
                },
                Position {
                    location: self.location + (0, -1),
                    direction: Direction::Up,
                },
            ],
            Direction::Left => [
                Position {
                    location: self.location,
                    direction: Direction::Up,
                },
                Position {
                    location: self.location,
                    direction: Direction::Down,
                },
                Position {
                    location: self.location + (-1, 0),
                    direction: Direction::Left,
                },
            ],
            Direction::Right => [
                Position {
                    location: self.location,
                    direction: Direction::Up,
                },
                Position {
                    location: self.location,
                    direction: Direction::Down,
                },
                Position {
                    location: self.location + (1, 0),
                    direction: Direction::Right,
                },
            ],
            Direction::Down => [
                Position {
                    location: self.location,
                    direction: Direction::Left,
                },
                Position {
                    location: self.location,
                    direction: Direction::Right,
                },
                Position {
                    location: self.location + (0, 1),
                    direction: Direction::Down,
                },
            ],
        }
    }
}

pub struct Input {
    spaces: HashSet<Coord>,
    start: Coord,
    end: Coord,
}

impl Input {
    pub fn valid_ends(&self) -> [Position; 4] {
        [
            Position {
                location: self.end,
                direction: Direction::Up,
            },
            Position {
                location: self.end,
                direction: Direction::Left,
            },
            Position {
                location: self.end,
                direction: Direction::Right,
            },
            Position {
                location: self.end,
                direction: Direction::Down,
            },
        ]
    }
}

pub struct ProcessedInput {
    scores: HashMap<Position, u64>,
    best_predecessors: HashMap<Position, HashSet<Position>>,
}

impl Input {
    pub fn to_scores(&self) -> ProcessedInput {
        let mut scores = HashMap::new();
        let mut best_predecessors = HashMap::new();
        let mut unvisited = self
            .spaces
            .iter()
            .cartesian_product(&[
                Direction::Up,
                Direction::Left,
                Direction::Right,
                Direction::Down,
            ])
            .map(|(&location, &direction)| Position {
                location,
                direction,
            })
            .collect::<HashSet<_>>();

        let mut frontier = BinaryHeap::new();

        // map from coordinate to lowest known risk so far
        let mut lowest_scores = HashMap::new();
        let start = Position {
            location: self.start,
            direction: Direction::Right,
        };

        // distance to the start is 0
        lowest_scores.insert(start, 0);

        frontier.push(Reverse((0, start)));

        while let Some((score, cell)) = frontier.pop().map(|Reverse(n)| n) {
            if scores.contains_key(&cell) {
                // already did this one
                continue;
            }

            let unvisited_neighbours = cell
                .neighbours()
                .into_iter()
                .filter(|n| self.spaces.contains(&n.location) && unvisited.contains(n))
                .collect::<Vec<_>>();

            for neighbour in unvisited_neighbours {
                let score_for_move_this_way = score
                    + if neighbour.location == cell.location {
                        1000
                    } else {
                        1
                    };

                match lowest_scores.get(&neighbour) {
                    Some(existing_score) if existing_score > &score_for_move_this_way => {
                        let minimum_score = existing_score.min(&score_for_move_this_way);

                        frontier.push(Reverse((*minimum_score, neighbour)));
                        lowest_scores.insert(neighbour, *minimum_score);

                        // the best way we've found to get here so far is not as good as this
                        best_predecessors.insert(neighbour, HashSet::from([cell]));
                    }
                    Some(existing_score) if existing_score == &score_for_move_this_way => {
                        // we've found another equally good way - remember this too
                        best_predecessors
                            .entry(neighbour)
                            .or_insert(HashSet::new())
                            .insert(cell);
                    }
                    Some(_) => {
                        // current path and score are better than this - nothing to do
                    }
                    None => {
                        best_predecessors
                            .entry(neighbour)
                            .or_insert(HashSet::new())
                            .insert(cell);
                        lowest_scores.insert(neighbour, score_for_move_this_way);
                        frontier.push(Reverse((score_for_move_this_way, neighbour)));
                    }
                }
            }

            unvisited.remove(&cell);
            scores.insert(cell, score);
        }

        ProcessedInput {
            scores,
            best_predecessors,
        }
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

pub fn part_1(input: &Input) -> u64 {
    let ProcessedInput { scores, .. } = input.to_scores();
    input
        .valid_ends()
        .into_iter()
        .map(|e| *scores.get(&e).unwrap())
        .min()
        .unwrap()
}

pub fn part_2(input: &Input) -> usize {
    let ProcessedInput {
        scores,
        best_predecessors,
    } = input.to_scores();
    let min_score = part_1(input);

    let mut steps_on_best_paths = HashSet::new();

    let mut cells_to_check = HashSet::new();
    for end in input.valid_ends() {
        if scores.get(&end).unwrap() == &min_score {
            cells_to_check.insert(end);
        }
    }

    while let Some(&cell) = cells_to_check.iter().next() {
        steps_on_best_paths.insert(cell);

        match best_predecessors.get(&cell) {
            Some(best_predecessors) => {
                for p in best_predecessors {
                    if !steps_on_best_paths.contains(p) {
                        cells_to_check.insert(*p);
                    }
                }
            }
            None => {
                if cell.location != input.start {
                    unreachable!();
                }
            }
        }

        cells_to_check.remove(&cell);
    }

    steps_on_best_paths
        .iter()
        .map(|c| c.location)
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
    let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
    let input = parse_input(input);

    assert_eq!(part_1(&input), 7036);
    assert_eq!(part_2(&input), 45);
}

#[test]
pub fn test_2() {
    let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";
    let input = parse_input(input);

    assert_eq!(part_1(&input), 11048);
    assert_eq!(part_2(&input), 64);
}
