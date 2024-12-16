use std::collections::{BTreeMap, HashSet, HashMap};
use std::ops::Add;
use itertools::Itertools;

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
    pub fn neighbours(self) -> [Coord; 4] {
        let Coord { x, y } = self;
        [
            (x + 1, y).into(),
            (x - 1, y).into(),
            (x, y + 1).into(),
            (x, y - 1).into(),
        ]
    }

    pub fn moves(self, direction: Direction) -> [(Coord, Direction); 3] {
        match direction {
            Direction::Up => {
                [(self, Direction::Left),
                (self, Direction::Right),
                (self + (0, -1), Direction::Up)]
            },
            Direction::Left => {
                [(self, Direction::Up),
                (self, Direction::Down),
                (self + (-1, 0), Direction::Left)]
            },
            Direction::Right => {
                [(self, Direction::Up),
                (self, Direction::Down),
                (self + (1, 0), Direction::Right)]
            },
            Direction::Down => {
                [(self, Direction::Left),
                (self, Direction::Right),
                (self + (0, 1), Direction::Down)]
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

pub struct Input {
    spaces: HashSet<Coord>,
    start: Coord,
    end: Coord
}

impl Input {
    pub fn to_scores(&self) -> (HashMap<(Coord, Direction), u64>, HashMap<(Coord, Direction), HashSet<(Coord, Direction)>>) {
        let mut scores = HashMap::new();
        let mut best_predecessors = HashMap::new();
        let mut unvisited = self.spaces.iter()
        .cartesian_product(&[Direction::Up, Direction::Left, Direction::Right, Direction::Down])
        .map(|(&c, &d)| (c, d)).collect::<HashSet<(Coord, Direction)>>();

        // map from lowest known risk so far to coords that satisfy
        let mut next_to_examine: BTreeMap<u64, HashSet<(Coord, Direction)>> = BTreeMap::new();

        // map from coordinate to lowest known risk so far
        let mut lowest_scores = HashMap::<(Coord, Direction), _>::new();
        let start = (self.start, Direction::Right);

        // distance to the start is 0
        lowest_scores.insert(start, 0);

        let mut next_node_to_consider = Some((start, 0u64));

        while let Some((cell, score)) = next_node_to_consider {

            let unvisited_neighbours = cell.0.moves(cell.1).into_iter()
                .filter(|n| self.spaces.contains(&n.0) && unvisited.contains(n))
                .collect::<Vec<_>>();

            // let unvisited_neighbours = neighbours.intersection(&unvisited);
            for neighbour in unvisited_neighbours {

                let score_for_move_this_way = score + if neighbour.0 == cell.0 { 1000 } else { 1 };

                match lowest_scores.get(&neighbour) {
                    Some(existing_score) => {
                        if existing_score > &score_for_move_this_way {
                            let minimum_score = existing_score.min(&score_for_move_this_way);
                            // next_to_examine.entry(*existing_score).and_modify(|set| _ = set.remove(&neighbour));
                            match next_to_examine.entry(*existing_score) {
                                std::collections::btree_map::Entry::Occupied(mut o) => {
                                    o.get_mut().remove(&neighbour);
                                    if o.get().is_empty() {
                                        o.remove();
                                    }
                                },
                                std::collections::btree_map::Entry::Vacant(_) => {
                                    // nothing - might be first time through
                                }
                            }
                            next_to_examine.entry(*minimum_score).or_default().insert(neighbour);
                            lowest_scores.insert(neighbour, *minimum_score);

                            // the best way we've found to get here so far is not as good as this
                            best_predecessors.insert(neighbour, HashSet::from([cell]));
                        } else if existing_score == &score_for_move_this_way {
                            // we've found another equally good way - remember this too
                            best_predecessors.entry(neighbour).or_insert(HashSet::new()).insert(cell);
                        }
                        // otherwise the existing minimum distance is still the min
                        // no need to adjust anything
                    },
                    None => {
                        best_predecessors.entry(neighbour).or_insert(HashSet::new()).insert(cell);
                        lowest_scores.insert(neighbour, score_for_move_this_way);
                        next_to_examine.entry(score_for_move_this_way).or_default().insert(neighbour);
                    }
                }
            }

            unvisited.remove(&cell);
            match next_to_examine.entry(score) {
                std::collections::btree_map::Entry::Occupied(mut o) => {
                    o.get_mut().remove(&cell);
                    if o.get().is_empty() {
                        o.remove();
                    }
                },
                std::collections::btree_map::Entry::Vacant(_) => {
                    // nothing - might be first time through
                }
            }
            scores.insert(cell, score);

            // println!("Visited {:?}, assigned score {:?}, moving on to next cell", cell, score);


            next_node_to_consider = next_to_examine.first_entry().map(|e| {
                (*e.get().iter().next().unwrap(), *e.key())
            });

            // next_node_to_consider = next_to_examine.iter().filter_map(|(k, v)| (!v.is_empty()).then_some((*v.iter().next().unwrap(), *k))).next();

        }

        (scores, best_predecessors)
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
                '.' => { spaces.insert(coord); }
                'S' => {
                    spaces.insert(coord);
                    start = coord;
                },
                'E' => {
                    spaces.insert(coord);
                    end = coord;
                },
                _ => { },
            }
        }
    }
    
    Input { spaces, start, end }
}

pub fn part_1(input: &Input) -> u64 {
    let (scores, _) = input.to_scores();

    [
        *scores.get(&(input.end, Direction::Up)).unwrap(),
        *scores.get(&(input.end, Direction::Left)).unwrap(),
        *scores.get(&(input.end, Direction::Right)).unwrap(),
        *scores.get(&(input.end, Direction::Down)).unwrap(),
    ].into_iter().min().unwrap()
}


pub fn part_2(input: &Input) -> usize {
    let (scores, predecessors) = input.to_scores();
    let min_score = part_1(input);
    let ends = [
        (input.end, Direction::Up),
        (input.end, Direction::Left),
        (input.end, Direction::Right),
        (input.end, Direction::Down)];
    

    let mut steps_on_best_paths = HashSet::new();

    let mut cells_to_check = HashSet::new();
    for end in ends {
        if scores.get(&end).unwrap() == &min_score {
            cells_to_check.insert(end);
        }
    }

    while let Some(&cell) = cells_to_check.iter().next() {
        steps_on_best_paths.insert(cell);

        match predecessors.get(&cell) {
            Some(best_predecessors) => {
                for p in best_predecessors {
                    if !steps_on_best_paths.contains(p) {
                        cells_to_check.insert(*p);
                    }
                }
            },
            None => {
                if cell.0 != input.start {
                    unreachable!();
                }
            }
        }

        cells_to_check.remove(&cell);
    }

    let cells_on_best_paths = steps_on_best_paths.iter().map(|c| c.0).collect::<HashSet<_>>();

    cells_on_best_paths.len()
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
    // println!("Part 1: {}", part_1(&input));
    // println!("Part 2: {}", part_2(&input));
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