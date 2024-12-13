use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

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
pub struct Region {
    plots: HashSet<Coord>,
}

impl Region {
    pub fn area(&self) -> usize {
        self.plots.len()
    }

    pub fn perimeter(&self) -> usize {
        self.plots
            .iter()
            .flat_map(|p| p.neighbours())
            .filter(|n| !self.plots.contains(n))
            .count()
    }

    pub fn sides(&self) -> usize {
        let mut sides = 0;

        for direction in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            // get all the cells in the region where the neighbour (in this direction)
            // is not in the region (the 'surface cells' in that direction)
            let upper_sides = self
                .plots
                .iter()
                .copied()
                .filter(|c| !self.plots.contains(&(*c + direction)))
                .collect::<HashSet<_>>();
            // pretend those make a garden and split it into regions -
            // contiguous cells following the opposite axis will form a single side and a single region
            let temp_garden = Garden {
                plots: upper_sides
                    .into_iter()
                    .map(|p| (p, 'X'))
                    .collect::<HashMap<_, _>>(),
            };
            // so the number of sides in that direction is just the number of these regions
            sides += temp_garden.to_regions().len()
        }

        sides
    }
}

#[derive(Clone, Debug)]
pub struct Garden {
    plots: HashMap<Coord, char>,
}

impl Garden {
    pub fn to_regions(&self) -> Vec<Region> {
        let mut garden = self.clone();
        let mut regions = Vec::new();
        while !garden.plots.is_empty() {
            regions.push(garden.remove_region());
        }

        regions
    }

    fn remove_region(&mut self) -> Region {
        let (&coord, &plant) = self.plots.iter().next().unwrap();
        let mut region = HashSet::new();
        let mut coords_still_to_consider = HashSet::from([coord]);

        while let Some(&coord) = coords_still_to_consider.iter().next() {
            let neighbours = coord.neighbours();
            // if the coord has neighbours in the garden with the same plant, they
            // are part of the region - but no point considering them again if
            // we already put them in the region
            for neighbour in neighbours {
                if region.contains(&neighbour) {
                    continue;
                }

                match self.plots.get(&neighbour) {
                    Some(p) if *p == plant => {
                        coords_still_to_consider.insert(neighbour);
                    }
                    _ => {}
                }
            }

            region.insert(coord);
            coords_still_to_consider.remove(&coord);
        }

        // remove the region from the garden
        for coord in &region {
            self.plots.remove(coord);
        }

        Region { plots: region }
    }
}

pub fn parse_input(input: &str) -> Garden {
    Garden {
        plots: input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    let coord = (x as i64, y as i64).into();
                    (coord, c)
                })
            })
            .collect(),
    }
}

pub fn part_1(input: &Garden) -> usize {
    input
        .to_regions()
        .iter()
        .map(|r| r.area() * r.perimeter())
        .sum()
}

pub fn part_2(input: &Garden) -> usize {
    input
        .to_regions()
        .iter()
        .map(|r| r.area() * r.sides())
        .sum()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test_1() {
    let input = "AAAA
BBCD
BBCC
EEEC
";
    let input = parse_input(input);

    assert_eq!(part_1(&input), 140);
    assert_eq!(part_2(&input), 80);
}

#[test]
pub fn test_2() {
    let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
    let input = parse_input(input);

    assert_eq!(part_1(&input), 772);
    assert_eq!(part_2(&input), 436);
}

#[test]
pub fn test_3() {
    let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
    let input = parse_input(input);

    assert_eq!(part_1(&input), 1930);
    assert_eq!(part_2(&input), 1206);
}

#[test]
pub fn test_4() {
    let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
    let input = parse_input(input);

    assert_eq!(part_2(&input), 236);
}

#[test]
pub fn test_5() {
    let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
    let input = parse_input(input);

    assert_eq!(part_2(&input), 368);
}
