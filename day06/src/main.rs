use std::collections::{HashMap, HashSet, BTreeSet};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Orientation {
    Up, Left, Right,Down
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
    obstructions_by_x: HashMap<i64, BTreeSet<i64>>,
    obstructions_by_y: HashMap<i64, BTreeSet<i64>>,
    size: (i64, i64),
}

#[derive(Clone, Debug)]
pub struct Input {
    area: Area,
    guard_start: (i64, i64),
    guard_orientation: Orientation,
}

pub fn parse_input(input: &str) -> Input {
    let mut size = (0, 0);
    let mut guard = None;
    let mut obstructions_by_x = HashMap::new();
    let mut obstructions_by_y = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x as i64;
            let y = y as i64;
            size = (x, y);
            match c {
                '#' => {
                    obstructions_by_x.entry(x).or_insert(BTreeSet::new()).insert(y);
                    obstructions_by_y.entry(y).or_insert(BTreeSet::new()).insert(x);
                },
                '^' => {
                    guard = Some(((x, y), Orientation::Up));
                },
                '>' => {
                    guard = Some(((x, y), Orientation::Right));
                },
                '<' => {
                    guard = Some(((x, y), Orientation::Left));
                },
                'v' => {
                    guard = Some(((x, y), Orientation::Down));
                },
                _ => { }
            }
        }
    }
    let (guard_start, guard_orientation) = guard.unwrap();

    Input {
        area: Area {
            obstructions_by_x,
            obstructions_by_y,
            size
        },
        guard_start,
        guard_orientation,
    }
}

impl Input {
    // returns where the guard stops (if that is in the grid)
    pub fn next_obstruction_for_guard(&self) -> Option<(i64, i64)> {
        match self.guard_orientation {
            Orientation::Up => {
                let next_obstruction = self.area.obstructions_by_x.get(&self.guard_start.0).and_then(|obstructions| obstructions.range(..self.guard_start.1).next_back());
                next_obstruction.map(|o| (self.guard_start.0, *o))
            }
            Orientation::Left => {
                let next_obstruction = self.area.obstructions_by_y.get(&self.guard_start.1).and_then(|obstructions| obstructions.range(..self.guard_start.0).next_back());
                next_obstruction.map(|o| (*o, self.guard_start.1))
            }
            Orientation::Right => {
                let next_obstruction = self.area.obstructions_by_y.get(&self.guard_start.1).and_then(|obstructions| obstructions.range(self.guard_start.0..).next());
                next_obstruction.map(|o| (*o, self.guard_start.1))
            }
            Orientation::Down => {
                let next_obstruction = self.area.obstructions_by_x.get(&self.guard_start.0).and_then(|obstructions| obstructions.range(self.guard_start.1..).next());
                next_obstruction.map(|o| (self.guard_start.0, *o))
            },
        }
    }
}

pub fn cells_in_path(input: &mut Input) -> HashSet<(i64, i64)> {
    let mut cells_walked = HashSet::new();
    cells_walked.insert(input.guard_start);

    while let Some((x, y)) = input.next_obstruction_for_guard() {
        match input.guard_orientation {
            Orientation::Up => {
                cells_walked.extend((y + 1..input.guard_start.1).map(|y| (x, y)));
                input.guard_start = (x, y + 1);
            },
            Orientation::Left => {
                cells_walked.extend((x + 1..input.guard_start.0).map(|x| (x, y)));
                input.guard_start = (x + 1, y);
            },
            Orientation::Right => {
                cells_walked.extend((input.guard_start.0..x).map(|x| (x, y)));
                input.guard_start = (x - 1, y);
            },
            Orientation::Down => {
                cells_walked.extend((input.guard_start.1..y).map(|y| (x, y)));
                input.guard_start = (x, y - 1);
            },
        }
        input.guard_orientation = input.guard_orientation.turn_right();
        // println!("{:?}", cells_walked);
        // dbg!(&input.guard_start);
        // dbg!(&input.guard_orientation);
    }

    // now add the cells that take the guard off the board
    match input.guard_orientation {
        Orientation::Up => cells_walked.extend((0..input.guard_start.1).map(|y| (input.guard_start.0, y))),
        Orientation::Left => cells_walked.extend((0..input.guard_start.0).map(|x| (x, input.guard_start.1))),
        Orientation::Right => cells_walked.extend((input.guard_start.0..=input.area.size.0).map(|x| (x, input.guard_start.1))),
        Orientation::Down => cells_walked.extend((input.guard_start.1..=input.area.size.1).map(|y| (input.guard_start.0, y))),
    }

    cells_walked
}

pub fn enters_loop(input: &mut Input) -> bool {
    let mut cells_walked = HashSet::new();
    cells_walked.insert((input.guard_start, input.guard_orientation));

    while let Some((x, y)) = input.next_obstruction_for_guard() {
        match input.guard_orientation {
            Orientation::Up => {
                for guard_cell in (y + 1..input.guard_start.1).map(|y| (x, y)) {
                    if !cells_walked.insert((guard_cell, input.guard_orientation)) {
                        return true;
                    }
                }

                input.guard_start = (x, y + 1);
            },
            Orientation::Left => {
                for guard_cell in (x + 1..input.guard_start.0).map(|x| (x, y)) {
                    if !cells_walked.insert((guard_cell, input.guard_orientation)) {
                        return true;
                    }
                }

                input.guard_start = (x + 1, y);
            },
            Orientation::Right => {
                for guard_cell in (input.guard_start.0..x).map(|x| (x, y)) {
                    if !cells_walked.insert((guard_cell, input.guard_orientation)) {
                        return true;
                    }
                }

                input.guard_start = (x - 1, y);
            },
            Orientation::Down => {
                for guard_cell in (input.guard_start.1..y).map(|y| (x, y)) {
                    if !cells_walked.insert((guard_cell, input.guard_orientation)) {
                        return true;
                    }
                }
                input.guard_start = (x, y - 1);
            },
        }
        input.guard_orientation = input.guard_orientation.turn_right();
        // println!("{:?}", cells_walked);
        // dbg!(&input.guard_start);
        // dbg!(&input.guard_orientation);
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
        if cell == input.guard_start {
            // can't put a new obstruction where the guard is
            continue;
        }

        let mut new_input = input.clone();
        // try putting an obstruction there
        new_input.area.obstructions_by_x.entry(cell.0).or_default().insert(cell.1);
        new_input.area.obstructions_by_y.entry(cell.1).or_default().insert(cell.0);
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
