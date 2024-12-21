use std::collections::{btree_map::Keys, HashMap};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum KeypadButton {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Activate,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

pub fn horizontal_between(start: KeypadButton, target: KeypadButton) -> Option<(Direction, usize)> {
    match start {
        KeypadButton::One | KeypadButton::Four | KeypadButton::Seven => match target {
            KeypadButton::One | KeypadButton::Four | KeypadButton::Seven => None,
            KeypadButton::Zero | KeypadButton::Two | KeypadButton::Five | KeypadButton::Eight => Some((Direction::Right, 1)),
            KeypadButton::Three | KeypadButton::Six | KeypadButton::Nine | KeypadButton::Activate=> Some((Direction::Right, 2)),
        },
        KeypadButton::Two | KeypadButton::Five | KeypadButton::Eight | KeypadButton::Zero => match target {
            KeypadButton::One | KeypadButton::Four | KeypadButton::Seven => Some((Direction::Left, 1)),
            KeypadButton::Zero | KeypadButton::Two | KeypadButton::Five | KeypadButton::Eight => None,
            KeypadButton::Three | KeypadButton::Six | KeypadButton::Nine | KeypadButton::Activate=> Some((Direction::Right, 1)),
        },
        KeypadButton::Three | KeypadButton::Six | KeypadButton::Nine | KeypadButton::Activate => match target {
            KeypadButton::One | KeypadButton::Four | KeypadButton::Seven => Some((Direction::Left, 2)),
            KeypadButton::Zero | KeypadButton::Two | KeypadButton::Five | KeypadButton::Eight => Some((Direction::Left, 1)),
            KeypadButton::Three | KeypadButton::Six | KeypadButton::Nine | KeypadButton::Activate=> None,
        },
    }
}

pub fn vertical_between(start: KeypadButton, target: KeypadButton) -> Option<(Direction, usize)> {
    match start {
        KeypadButton::Zero | KeypadButton::Activate => match target {
            KeypadButton::Zero | KeypadButton::Activate => None,
            KeypadButton::One | KeypadButton::Two | KeypadButton::Three => Some((Direction::Up, 1)),
            KeypadButton::Four | KeypadButton::Five | KeypadButton::Six => Some((Direction::Up, 2)),
            KeypadButton::Seven | KeypadButton::Eight | KeypadButton::Nine => Some((Direction::Up, 3)),
        },
        KeypadButton::One | KeypadButton::Two | KeypadButton::Three => match target {
            KeypadButton::Zero | KeypadButton::Activate => Some((Direction::Down, 1)),
            KeypadButton::One | KeypadButton::Two | KeypadButton::Three => None,
            KeypadButton::Four | KeypadButton::Five | KeypadButton::Six => Some((Direction::Up, 1)),
            KeypadButton::Seven | KeypadButton::Eight | KeypadButton::Nine => Some((Direction::Up, 2)),
        },
        KeypadButton::Four | KeypadButton::Five | KeypadButton::Six => match target {
            KeypadButton::Zero | KeypadButton::Activate => Some((Direction::Down, 2)),
            KeypadButton::One | KeypadButton::Two | KeypadButton::Three => Some((Direction::Down, 1)),
            KeypadButton::Four | KeypadButton::Five | KeypadButton::Six => None,
            KeypadButton::Seven | KeypadButton::Eight | KeypadButton::Nine => Some((Direction::Up, 1)),
        },
        KeypadButton::Seven | KeypadButton::Eight | KeypadButton::Nine => match target {
            KeypadButton::Zero | KeypadButton::Activate => Some((Direction::Down, 3)),
            KeypadButton::One | KeypadButton::Two | KeypadButton::Three => Some((Direction::Down, 2)),
            KeypadButton::Four | KeypadButton::Five | KeypadButton::Six => Some((Direction::Down, 1)),
            KeypadButton::Seven | KeypadButton::Eight | KeypadButton::Nine => None,
        },
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum DirectionalButton {
    Up,
    Left,
    Right,
    Down,
    Activate,
}

pub fn shortest_sequence_to_press(start: DirectionalButton, target: DirectionalButton) -> Vec<DirectionalButton> {
    // go from start to target and press it
    // assumes we want the first robot is on button start and needs to move to target and press it
    // assumes the next robot is on A
    // and gives us the shortest sequence it needs to run
    match start {
        DirectionalButton::Up => match target {
            DirectionalButton::Up => vec![DirectionalButton::Activate],
            DirectionalButton::Left => vec![DirectionalButton::Down, DirectionalButton::Left, DirectionalButton::Activate],
            DirectionalButton::Right => vec![DirectionalButton::Down, DirectionalButton::Right, DirectionalButton::Activate],
            DirectionalButton::Down => vec![DirectionalButton::Down, DirectionalButton::Activate],
            DirectionalButton::Activate => vec![DirectionalButton::Right, DirectionalButton::Activate],
        },
        DirectionalButton::Left => match target {
            DirectionalButton::Up => vec![DirectionalButton::Right, DirectionalButton::Up, DirectionalButton::Activate],
            DirectionalButton::Left => vec![DirectionalButton::Activate],
            DirectionalButton::Right => vec![DirectionalButton::Right, DirectionalButton::Right, DirectionalButton::Activate],
            DirectionalButton::Down => vec![DirectionalButton::Right, DirectionalButton::Activate],
            DirectionalButton::Activate => vec![DirectionalButton::Right, DirectionalButton::Right, DirectionalButton::Up, DirectionalButton::Activate],
        },
        DirectionalButton::Right => match target {
            DirectionalButton::Up => vec![DirectionalButton::Left, DirectionalButton::Up, DirectionalButton::Activate],
            DirectionalButton::Left => vec![DirectionalButton::Left, DirectionalButton::Left, DirectionalButton::Activate],
            DirectionalButton::Right => vec![DirectionalButton::Activate],
            DirectionalButton::Down => vec![DirectionalButton::Left, DirectionalButton::Activate],
            DirectionalButton::Activate => vec![DirectionalButton::Up, DirectionalButton::Activate],
        }
        DirectionalButton::Down => match target {
            DirectionalButton::Up => vec![DirectionalButton::Up, DirectionalButton::Activate],
            DirectionalButton::Left => vec![DirectionalButton::Left, DirectionalButton::Activate],
            DirectionalButton::Right => vec![DirectionalButton::Right, DirectionalButton::Activate],
            DirectionalButton::Down => vec![DirectionalButton::Activate],
            DirectionalButton::Activate => vec![DirectionalButton::Up, DirectionalButton::Right, DirectionalButton::Activate],
        },
        DirectionalButton::Activate => match target {
            DirectionalButton::Up => vec![DirectionalButton::Left, DirectionalButton::Activate],
            DirectionalButton::Left => vec![DirectionalButton::Down, DirectionalButton::Left, DirectionalButton::Left, DirectionalButton::Activate],
            DirectionalButton::Right => vec![DirectionalButton::Down, DirectionalButton::Activate],
            DirectionalButton::Down => vec![DirectionalButton::Down, DirectionalButton::Left, DirectionalButton::Activate],
            DirectionalButton::Activate => vec![DirectionalButton::Activate],
        }
    }
}

pub fn shortest_sequence_between(start: KeypadButton, target: KeypadButton) -> Vec<DirectionalButton> {
    // we start at start, we go to target
    let horizontal_moves = horizontal_between(start, target);
    let vertical_moves = vertical_between(start, target);

    let mut first_robot_presses = Vec::new();

    // because of the shape of the subsequent keypads, we always want to do the lefts before we do the others
    let mut postponed_left = false;
    let mut postponed_down = false;
    if let Some((Direction::Left, times)) = horizontal_moves {
        // we always want to do this first if we can.
        if start == KeypadButton::Zero || (start == KeypadButton::Activate && times == 2) {
            postponed_left = true;
        } else {
            first_robot_presses.extend(std::iter::repeat(DirectionalButton::Left).take(times));
        }
    }

    if let Some((Direction::Down, times)) = vertical_moves {
        if start == KeypadButton::One || (start == KeypadButton::Four && times == 2) || (start == KeypadButton::Seven && times == 3) {
            postponed_down = true;
        } else {
            first_robot_presses.extend(std::iter::repeat(DirectionalButton::Down).take(times));
        }
    }

    // if start != KeypadButton::Zero && start != KeypadButton::Activate {
    //     if let Some((Direction::Left, times)) = horizontal_moves {
    //         first_robot_presses.extend(std::iter::repeat(DirectionalButton::Left).take(times));
    //     }    
    // }

    // if start != KeypadButton::One && start != KeypadButton::Four && start != KeypadButton::Seven {
    //     if let Some((Direction::Down, times)) = vertical_moves {
    //         first_robot_presses.extend(std::iter::repeat(DirectionalButton::Down).take(times));
    //     }  
    // }

    if let Some((Direction::Up, times)) = vertical_moves {
        first_robot_presses.extend(std::iter::repeat(DirectionalButton::Up).take(times));
    } 

    if let Some((Direction::Right, times)) = horizontal_moves {
        first_robot_presses.extend(std::iter::repeat(DirectionalButton::Right).take(times));
    }

    if postponed_left {
        if let Some((Direction::Left, times)) = horizontal_moves {
            first_robot_presses.extend(std::iter::repeat(DirectionalButton::Left).take(times));
        }    
    }

    if postponed_down {
        if let Some((Direction::Down, times)) = vertical_moves {
            first_robot_presses.extend(std::iter::repeat(DirectionalButton::Down).take(times));
        }  
    }

    first_robot_presses.push(DirectionalButton::Activate);
    println!("shortest sequence to press {:?} from {:?} for first robot is {:?} with length {}", target, start, first_robot_presses, first_robot_presses.len());
    first_robot_presses.insert(0, DirectionalButton::Activate);

    let mut second_robot_presses = Vec::new();

    for pair in first_robot_presses.windows(2) {
        second_robot_presses.extend(shortest_sequence_to_press(pair[0], pair[1]));
    }

    // println!("shortest sequence to press {:?} from {:?} for second robot is {:?} with length {}", target, start, second_robot_presses, second_robot_presses.len());
    second_robot_presses.insert(0, DirectionalButton::Activate);

    let mut third_robot_presses = Vec::new();

    for pair in second_robot_presses.windows(2) {
        third_robot_presses.extend(shortest_sequence_to_press(pair[0], pair[1]));
    }

    // println!("shortest sequence to press {:?} from {:?} is {:?} with length {}", target, start, third_robot_presses, third_robot_presses.len());
    third_robot_presses
}


#[derive(Clone, Debug)]
pub struct Sequence {
    numbers: Vec<KeypadButton>,
    numeric_part: usize,
}

impl Sequence {
    pub fn shortest_presses(&self) -> Vec<DirectionalButton> {
        let mut numbers = self.numbers.clone();
        numbers.insert(0, KeypadButton::Activate);
        let mut presses = Vec::new();
        for pair in numbers.windows(2) {
            presses.extend(shortest_sequence_between(pair[0], pair[1]));
        }

        presses
    }

    pub fn complexity(&self) -> usize {
        let length = self.shortest_presses().len();
        println!("{:?} has length {}", self, length);
        length * self.numeric_part
    }
}

pub struct Input {
    codes: Vec<Sequence>
}

pub fn parse_input(input: &str) -> Input {
    Input { codes: 
        input.lines().map(|line| {
            let numeric = line[..line.len() - 1].parse().unwrap();
            Sequence { numbers: line.chars().map(|c| match c {
                'A' => KeypadButton::Activate,
                '1' => KeypadButton::One,
                '2' => KeypadButton::Two,
                '3' => KeypadButton::Three,
                '4' => KeypadButton::Four,
                '5' => KeypadButton::Five,
                '6' => KeypadButton::Six,
                '7' => KeypadButton::Seven,
                '8' => KeypadButton::Eight,
                '9' => KeypadButton::Nine,
                '0' => KeypadButton::Zero,
                _ => unreachable!(),
            }).collect(),
            numeric_part: numeric }
        }).collect()
    }
}

pub fn part_1(input: &Input) -> usize {
    input.codes.iter().map(|s| s.complexity()).sum()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    use std::time::Instant;
    let now = Instant::now();
    println!("Part 1: {}", part_1(&input)); // 163280 - too high
    // had lengths 78, 76, 68, 74, 74, 
    println!("{:?}", now.elapsed());
    let now = Instant::now();
    // println!("Part 2: {}", part_2(&input));
    println!("{:?}", now.elapsed());
}

#[test]
pub fn test_simple() {
    // shortest_sequence_between(KeypadButton::Activate, KeypadButton::Zero);

    // <v<A >>^A <vA <A >>^A A vA A <^A >A     <v<A >>^A A vA ^A <vA >^A A <A >A <v<A >A >^A A A vA <^A >A
    // < A   v < A   A   > > ^ A   < A   A   > A   v A   A   ^ A    < v A    A   A   > ^ A
    // ^   <   <  A    ^    ^   A >  >  A   v  v  v   A
    // 1   7   9   A

    let input = "179A
";
    let input = parse_input(input);

    for c in input.codes {
        let presses = c.shortest_presses();
        println!("Sequence {:?} has shortest instruction {:?} with length {:?}", c, presses, presses.len());
    }
}

#[test]
pub fn test_simple_2() {
    // shortest_sequence_between(KeypadButton::Activate, KeypadButton::Zero);

    // <v<A >>^A <vA <A >>^A A vA A <^A >A     <v<A >>^A A vA ^A <vA >^A A <A >A <v<A >A >^A A A vA <^A >A
    // < A   v < A   A   > > ^ A   < A   A   > A   v A   A   ^ A    < v A    A   A   > ^ A
    // ^   <   <  A    ^    ^   A >  >  A   v  v  v   A
    // 1   7   9   A

    let input = "456A
";
    let input = parse_input(input);

    for c in input.codes {
        let presses = c.shortest_presses();
        println!("Sequence {:?} has shortest instruction {:?} with length {:?}", c, presses, presses.len());
    }
}

#[test]
pub fn test_simple_3() {
    //  <v<A>>^A   vA^A.   <vA<AA>>^A   A vA  <^A  >A A   vA  ^A.  <vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    // <A>A  .  v<<A  A  >  ^ A  A  > A
    // ^A   < <  ^ ^  A


    //  [Left, Activate, Activate, Down, Left, Activate, Activate, Right, Right, Up, Activate]

    // [Down, Left, Left, Activate, Right, Right, Up, Activate, Activate, Down, Left, Activate,
    // Left, Activate, Right, Right, Up, Activate, Activate, Down, Activate, Activate, Left, 
    // Up, Activate, Right, Activate]

    let input = "379A
";
    let input = parse_input(input);

    for c in input.codes {
        let presses = c.shortest_presses();
        println!("Sequence {:?} has shortest instruction {:?} with length {:?}", c, presses, presses.len());
    }
}


#[test]
pub fn test() {
    let input = "029A
980A
179A
456A
379A
";
    let input = parse_input(input);



    for c in &input.codes {
        let presses = c.shortest_presses();
        println!("Sequence {:?} has shortest instruction {:?} with length {:?}", c, presses, presses.len());
    }
    // let cheats = input.find_cheats();

    assert_eq!(part_1(&input), 126384);
}
