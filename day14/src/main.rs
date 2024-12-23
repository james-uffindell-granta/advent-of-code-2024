use std::collections::HashMap;
use std::fmt::Formatter;
use winnow::ascii::dec_int;
use winnow::combinator::{preceded, separated, separated_pair};
use winnow::{PResult, Parser};

#[derive(Copy, Clone, Debug)]
pub struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

impl Robot {
    pub fn step(self, steps: i64, room_dimensions: (i64, i64)) -> Self {
        let x_steps = steps % room_dimensions.0;
        let y_steps = steps % room_dimensions.1;
        let new_position = (
            (self.position.0 + self.velocity.0 * x_steps) % room_dimensions.0,
            (self.position.1 + self.velocity.1 * y_steps) % room_dimensions.1,
        );

        Self {
            // readjust so always positive
            position: (
                (new_position.0 + room_dimensions.0) % room_dimensions.0,
                (new_position.1 + room_dimensions.1) % room_dimensions.1,
            ),
            velocity: self.velocity,
        }
    }
}

pub fn parse_robot(input: &mut &str) -> PResult<Robot> {
    let ((px, py), (vx, vy)) = separated_pair(
        preceded("p=", separated_pair(dec_int, ",", dec_int)),
        " ",
        preceded("v=", separated_pair(dec_int, ",", dec_int)),
    )
    .parse_next(input)?;
    Ok(Robot {
        position: (px, py),
        velocity: (vx, vy),
    })
}

pub fn parse_items(input: &mut &str) -> PResult<Vec<Robot>> {
    separated(1.., parse_robot, "\n").parse_next(input)
}

pub fn parse_input(input: &str) -> Vec<Robot> {
    parse_items.parse(input.trim()).unwrap()
}

pub fn part_1(input: &[Robot], room_dimensions: (i64, i64)) -> i64 {
    let new_locations = input
        .iter()
        .map(|r| r.step(100, room_dimensions))
        .collect::<Vec<_>>();

    let mut robots_by_location = HashMap::new();
    for robot in new_locations {
        *robots_by_location.entry(robot.position).or_insert(0i64) += 1;
    }

    let forbidden_coords = (room_dimensions.0 / 2, room_dimensions.1 / 2);

    let upper_left: i64 = robots_by_location
        .iter()
        .filter_map(|(k, v)| (k.0 < forbidden_coords.0 && k.1 < forbidden_coords.1).then_some(*v))
        .sum();
    let upper_right: i64 = robots_by_location
        .iter()
        .filter_map(|(k, v)| (k.0 > forbidden_coords.0 && k.1 < forbidden_coords.1).then_some(*v))
        .sum();
    let lower_left: i64 = robots_by_location
        .iter()
        .filter_map(|(k, v)| (k.0 < forbidden_coords.0 && k.1 > forbidden_coords.1).then_some(*v))
        .sum();
    let lower_right: i64 = robots_by_location
        .iter()
        .filter_map(|(k, v)| (k.0 > forbidden_coords.0 && k.1 > forbidden_coords.1).then_some(*v))
        .sum();
    upper_left * upper_right * lower_left * lower_right
}

pub fn part_2(input: &[Robot], room_dimensions: (i64, i64)) -> i64 {
    let mut old_locations = input.to_vec();

    for i in 1..=(room_dimensions.0 * room_dimensions.1) {
        let new_locations = old_locations
            .iter()
            .map(|r| r.step(1, room_dimensions))
            .collect::<Vec<_>>();

        let mut robots_by_location = HashMap::new();
        for robot in &new_locations {
            robots_by_location.insert(robot.position, 1);
        }

        let image = format!("{}", Room {
            robots: robots_by_location,
            room_dimensions
        });

        if image.contains("##########") {
            return i;
        }

        old_locations = new_locations;
    }

    unreachable!();
}

pub struct Room {
    robots: HashMap<(i64, i64), i64>,
    room_dimensions: (i64, i64),
}

impl std::fmt::Display for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for y in 0..self.room_dimensions.0 {
            for x in 0..self.room_dimensions.1 {
                if let Some(_) = self.robots.get(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    println!("Part 1: {}", part_1(&input, (101, 103)));
    println!("Part 2: {}", part_2(&input, (101, 103)));
}

#[test]
pub fn test_1() {
    let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    let input = parse_input(input);

    assert_eq!(part_1(&input, (11, 7)), 12);
    part_2(&input, (11, 7));
}
