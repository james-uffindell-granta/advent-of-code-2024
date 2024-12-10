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
    pub fn neighbours(self) -> [Coord; 4] {
        let Coord { x, y } = self;
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
    pub fn to_trails(&self) -> HashMap<Coord, HashSet<Vec<Coord>>> {
        let mut trails = HashMap::new();
        // trails from a 9 upwards are just single points
        for coord in self
            .heights
            .iter()
            .filter_map(|(c, h)| (*h == 9).then_some(*c))
        {
            trails.insert(coord, HashSet::from([vec![coord]]));
        }

        for height in (0..=8).rev() {
            for coord in self
                .heights
                .iter()
                .filter_map(|(c, h)| (*h == height).then_some(*c))
            {
                let mut new_trails = HashSet::new();

                // get the neighbors that are one away upwards
                let relevant_neighbours = coord
                    .neighbours()
                    .into_iter()
                    .filter(|c| self.heights.get(c).filter(|h| **h == height + 1).is_some());

                // then, for all trails that start from neighbours one above, the trails from this point are all of those with this point added
                for neighbour in relevant_neighbours {
                    if let Some(trails_from_neighbour) = trails.get(&neighbour) {
                        for trail in trails_from_neighbour {
                            let mut longer_trail = trail.clone();
                            longer_trail.push(coord);
                            new_trails.insert(longer_trail);
                        }
                    }
                }

                trails.insert(coord, new_trails);
            }
        }

        trails
    }
}

pub fn parse_input(input: &str) -> Input {
    Input {
        heights: input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as i64, y as i64).into(), c.to_digit(10).unwrap()))
            })
            .collect(),
    }
}

pub fn part_1(input: &Input) -> usize {
    let starts = input
        .heights
        .iter()
        .filter_map(|(c, h)| (*h == 0).then_some(*c))
        .collect::<HashSet<_>>();
    // we just care about the distinct ends of the trails (which are the 'starts' in our representation)
    input.to_trails()
        .iter()
        .filter_map(|(c, set)| {
            starts
                .contains(c)
                .then_some(set.iter().map(|v| v[0]).collect::<HashSet<_>>().len())
        })
        .sum()
}

pub fn part_2(input: &Input) -> usize {
    let starts = input
        .heights
        .iter()
        .filter_map(|(c, h)| (*h == 0).then_some(*c))
        .collect::<HashSet<_>>();
    input.to_trails()
        .iter()
        .filter_map(|(c, set)| starts.contains(c).then_some(set.len()))
        .sum()
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
