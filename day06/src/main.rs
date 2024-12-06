use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Coord {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Orientation {
    Up,
    Left,
    Right,
    Down,
}

impl Orientation {
    pub fn turn_right(self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Left => Orientation::Up,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Area {
    obstructions_by_x: HashMap<usize, BTreeSet<usize>>,
    obstructions_by_y: HashMap<usize, BTreeSet<usize>>,
    size: Coord,
}

#[derive(Copy, Clone, Debug)]
pub struct Guard {
    location: Coord,
    orientation: Orientation,
}

#[derive(Clone, Debug)]
pub struct Input {
    area: Area,
    guard: Guard,
}

pub fn parse_input(input: &str) -> Input {
    let mut size = Coord { x: 0, y: 0 };
    let mut guard = None;
    let mut obstructions_by_x = HashMap::new();
    let mut obstructions_by_y = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let location = Coord { x, y };
            size = location;
            match c {
                '#' => {
                    obstructions_by_x
                        .entry(x)
                        .or_insert(BTreeSet::new())
                        .insert(y);
                    obstructions_by_y
                        .entry(y)
                        .or_insert(BTreeSet::new())
                        .insert(x);
                }
                '^' => {
                    guard = Some(Guard {
                        location,
                        orientation: Orientation::Up,
                    });
                }
                '>' => {
                    guard = Some(Guard {
                        location,
                        orientation: Orientation::Right,
                    });
                }
                '<' => {
                    guard = Some(Guard {
                        location,
                        orientation: Orientation::Left,
                    });
                }
                'v' => {
                    guard = Some(Guard {
                        location,
                        orientation: Orientation::Down,
                    });
                }
                _ => {}
            }
        }
    }
    let guard = guard.unwrap();

    Input {
        area: Area {
            obstructions_by_x,
            obstructions_by_y,
            size,
        },
        guard,
    }
}

impl Input {
    // returns where the guard stops (if that is in the grid)
    pub fn next_obstruction_for_guard(&self) -> Option<Coord> {
        match self.guard.orientation {
            Orientation::Up => self
                .area
                .obstructions_by_x
                .get(&self.guard.location.x)
                .and_then(|os| os.range(..self.guard.location.y).next_back())
                .map(|y| Coord {
                    x: self.guard.location.x,
                    y: *y,
                }),
            Orientation::Left => self
                .area
                .obstructions_by_y
                .get(&self.guard.location.y)
                .and_then(|os| os.range(..self.guard.location.x).next_back())
                .map(|x| Coord {
                    x: *x,
                    y: self.guard.location.y,
                }),
            Orientation::Right => self
                .area
                .obstructions_by_y
                .get(&self.guard.location.y)
                .and_then(|os| os.range(self.guard.location.x..).next())
                .map(|x| Coord {
                    x: *x,
                    y: self.guard.location.y,
                }),
            Orientation::Down => self
                .area
                .obstructions_by_x
                .get(&self.guard.location.x)
                .and_then(|os| os.range(self.guard.location.y..).next())
                .map(|y| Coord {
                    x: self.guard.location.x,
                    y: *y,
                }),
        }
    }
}

pub fn cells_in_path(input: &mut Input) -> HashSet<Coord> {
    let mut cells_walked = HashSet::new();
    cells_walked.insert(input.guard.location);

    while let Some(Coord { x, y }) = input.next_obstruction_for_guard() {
        match input.guard.orientation {
            Orientation::Up => {
                cells_walked.extend((y + 1..input.guard.location.y).map(|y| Coord { x, y }));
                input.guard.location = Coord { x, y: y + 1 };
            }
            Orientation::Left => {
                cells_walked.extend((x + 1..input.guard.location.x).map(|x| Coord { x, y }));
                input.guard.location = Coord { x: x + 1, y };
            }
            Orientation::Right => {
                cells_walked.extend((input.guard.location.x..x).map(|x| Coord { x, y }));
                input.guard.location = Coord { x: x - 1, y };
            }
            Orientation::Down => {
                cells_walked.extend((input.guard.location.y..y).map(|y| Coord { x, y }));
                input.guard.location = Coord { x, y: y - 1 };
            }
        }

        input.guard.orientation = input.guard.orientation.turn_right();
    }

    // now add the cells that take the guard off the board
    match input.guard.orientation {
        Orientation::Up => cells_walked.extend((0..input.guard.location.y).map(|y| Coord {
            x: input.guard.location.x,
            y,
        })),
        Orientation::Left => cells_walked.extend((0..input.guard.location.x).map(|x| Coord {
            x,
            y: input.guard.location.y,
        })),
        Orientation::Right => {
            cells_walked.extend((input.guard.location.x..=input.area.size.x).map(|x| Coord {
                x,
                y: input.guard.location.y,
            }))
        }
        Orientation::Down => {
            cells_walked.extend((input.guard.location.y..=input.area.size.y).map(|y| Coord {
                x: input.guard.location.x,
                y,
            }))
        }
    }

    cells_walked
}

pub fn enters_loop(input: &mut Input) -> bool {
    let mut corners_walked = HashSet::new();

    while let Some(Coord { x, y }) = input.next_obstruction_for_guard() {
        input.guard.location = match input.guard.orientation {
            Orientation::Up => Coord { x, y: y + 1 },
            Orientation::Left => Coord { x: x + 1, y },
            Orientation::Right => Coord { x: x - 1, y },
            Orientation::Down => Coord { x, y: y - 1 },
        };

        if !corners_walked.insert((input.guard.location, input.guard.orientation)) {
            return true;
        }

        input.guard.orientation = input.guard.orientation.turn_right();
    }

    // if we're going off the board then no loop
    false
}

pub fn part_1(input: &Input) -> usize {
    cells_in_path(&mut input.clone()).len()
}

pub fn part_2(input: &Input) -> usize {
    let mut answer = 0;
    for cell in cells_in_path(&mut input.clone()) {
        if cell == input.guard.location {
            // can't put a new obstruction where the guard is
            continue;
        }

        let mut new_input = input.clone();
        // try putting an obstruction there
        new_input
            .area
            .obstructions_by_x
            .entry(cell.x)
            .or_default()
            .insert(cell.y);
        new_input
            .area
            .obstructions_by_y
            .entry(cell.y)
            .or_default()
            .insert(cell.x);
        if enters_loop(&mut new_input) {
            answer += 1;
        }
    }

    answer
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test() {
    let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
    let input = parse_input(input);
    assert_eq!(part_1(&input), 41);
    assert_eq!(part_2(&input), 6);
}
