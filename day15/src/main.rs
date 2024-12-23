use std::collections::HashSet;
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
    pub fn next(self, direction: Direction) -> Coord {
        match direction {
            Direction::Up => self + (0, -1),
            Direction::Left => self + (-1, 0),
            Direction::Right => self + (1, 0),
            Direction::Down => self + (0, 1),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up, Left, Right, Down,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    robot: Coord,
    boxes: HashSet<Coord>,
    walls: HashSet<Coord>,
    bounds: Coord,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    initial_state: State,
    moves: Vec<Direction>
}

impl State {
    pub fn next_free_space(&self, from: Coord, direction: Direction) -> Option<Coord> {
        match direction {
            Direction::Up => {
                for dy in 1..=from.y {
                    let space_to_try = from +(0, -dy);
                    // keep checking things until we find the space
                    if self.boxes.contains(&space_to_try) {
                        continue;
                    }

                    if self.walls.contains(&space_to_try) {
                        return None;
                    }

                    return Some(space_to_try);
                }
            },
            Direction::Left => {
                for dx in 1..=from.x {
                    let space_to_try = from + (-dx, 0);
                    // keep checking things until we find the space
                    if self.boxes.contains(&space_to_try) {
                        continue;
                    }

                    if self.walls.contains(&space_to_try) {
                        return None;
                    }

                    return Some(space_to_try);
                }
            },
            Direction::Right => {
                for dx in 1..=(self.bounds.x - from.x) {
                    let space_to_try = from + (dx, 0);
                    // keep checking things until we find the space
                    if self.boxes.contains(&space_to_try) {
                        continue;
                    }

                    if self.walls.contains(&space_to_try) {
                        return None;
                    }

                    return Some(space_to_try);
                }
            },
            Direction::Down => {
                for dy in 1..=(self.bounds.y - from.y) {
                    let space_to_try = from + (0, dy);
                    // keep checking things until we find the space
                    if self.boxes.contains(&space_to_try) {
                        continue;
                    }

                    if self.walls.contains(&space_to_try) {
                        return None;
                    }

                    return Some(space_to_try);
                }
            },
        }

        None
    }

    pub fn push_box(&self, box_to_move: Coord, direction: Direction) -> Option<State> {
        let mut boxes_moved = HashSet::new();
        let mut boxes_to_move = HashSet::new();
        boxes_to_move.insert(box_to_move);
        let delta = match direction {
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
        };

        while let Some(&b) = boxes_to_move.iter().next() {
            if self.walls.contains(&(b + delta)) || self.walls.contains(&(b + (1, 0) + delta)) {
                // there's a wall in the way of this box - we can't move
                return None;
            }

            // otherwise, assume we can move this box
            boxes_moved.insert(b);
            boxes_to_move.remove(&b);

            // and make a note to now also move all the boxes we moved into
            let new_box_location = b + delta;
            let locations_to_check = HashSet::from([new_box_location + (-1, 0), new_box_location, new_box_location + (1, 0)]);
            for b in locations_to_check {
                if !boxes_moved.contains(&b) && self.boxes.contains(&b) {
                    boxes_to_move.insert(b);
                }
            }
        }

        // if we got here then we managed to move every box that was in our way
        let mut new_state = self.clone();
        for b in &boxes_moved {
            new_state.boxes.remove(b);
        }

        for b in boxes_moved {
            // don't combine with previous loop 
            new_state.boxes.insert(b + delta);
        }

        Some(new_state)
    }

    pub fn step_expanded(&self, direction: Direction) -> Self {
        let new_robot = self.robot.next(direction);
        // we moved into a wall - not possible
        if self.walls.contains(&new_robot) {
            return self.clone();
        }

        // otherwise, see if we moved into a box
        if self.boxes.contains(&new_robot) {
            match self.push_box(new_robot, direction) {
                Some(mut state) => { 
                    state.robot = new_robot;
                    return state;
                },
                None => { return self.clone(); },
            }
        }

        if self.boxes.contains(&(new_robot + (-1, 0))) {
            // moved into the right side of a box
            match self.push_box(new_robot + (-1, 0), direction) {
                Some(mut state) => { 
                    state.robot = new_robot;
                    return state;
                },
                None => { return self.clone(); },
            }
        }

        let mut new_state = self.clone();
        new_state.robot = new_robot;
        new_state
    }

}

impl Input {
    pub fn run(&self) -> State {
        let mut state = self.initial_state.clone();
        for m in &self.moves {
            if let Some(free_space) = state.next_free_space(state.robot, *m) {
                // nothing but boxes between here and there - we can move
                // state = state.clone();
                let neighbour = state.robot.next(*m);
                // move the robot
                state.robot = neighbour;
                // adjust the boxes
                if neighbour != free_space {
                    state.boxes.remove(&neighbour);
                    state.boxes.insert(free_space);
                }
            }
        }

        state
    }

    pub fn run_expanded(&self) -> State {
        let mut state = self.initial_state.clone();
        for m in &self.moves {
            state = state.step_expanded(*m);
        }

        state
    }

}

pub fn parse_input(input: &str) -> Input {
    let (world, actions) = input.split_once("\n\n").unwrap();
    let mut bounds = (0, 0).into();
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot = (0, 0).into();
    for (y, line) in world.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = (x as i64, y as i64).into();
            bounds = coord;
            match c {
                '@' => robot = coord,
                '#' => { walls.insert(coord); },
                'O' => { boxes.insert(coord); },
                _ => { },
            }
        }
    }

    let mut moves = Vec::new();
    for d in actions.chars() {
        match d {
            '>' => { moves.push(Direction::Right); },
            '<' => { moves.push(Direction::Left); },
            '^' => { moves.push(Direction::Up); },
            'v' => { moves.push(Direction::Down); },
            _ => { },
        }
    }

    let initial_state = State {
        robot, boxes, walls, bounds,
    };

    Input { initial_state, moves }
}

pub fn part_1(input: &Input) -> i64 {
    let state = input.run();
    state.boxes.iter().map(|r| 100 * r.y + r.x).sum()
}

pub fn part_2(input: &Input) -> i64 {
    // just keep track of the left side of boxes
    let expanded_state = State {
        robot: (input.initial_state.robot.x * 2, input.initial_state.robot.y).into(),
        boxes: input.initial_state.boxes.iter().map(|r| (r.x * 2, r.y).into()).collect(),
        walls: input.initial_state.walls.iter().flat_map(|w| [(w.x * 2, w.y).into(), (w.x * 2 + 1, w.y).into()]).collect(),
        bounds: (input.initial_state.bounds.x * 2, input.initial_state.bounds.y).into(),
    };

    let expanded_input = Input {
        initial_state: expanded_state,
        moves: input.moves.clone(),
    };

    let state = expanded_input.run_expanded();
    state.boxes.iter().map(|r| 100 * r.y + r.x).sum()
}


fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test() {
    let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    let input = parse_input(input);

    assert_eq!(part_1(&input), 10092);
    assert_eq!(part_2(&input), 9021);
    // part_2(&input, (11, 7));
}


#[test]
pub fn test_small() {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    let input = parse_input(input);

    assert_eq!(part_1(&input), 2028);
    part_2(&input);
}

#[test]
pub fn test_third() {
    let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
    
        let input = parse_input(input);
        part_2(&input);
}