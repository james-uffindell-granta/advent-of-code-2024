use std::collections::{HashSet, HashMap, VecDeque};
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

pub fn bfs(unreachable: &HashSet<Coord>, bounds: Coord) -> HashMap<Coord, Coord> {
    let start = (0, 0).into();
    let mut queue: VecDeque<Coord> = VecDeque::from([start]);
    let mut scores = HashMap::new();
    scores.insert(start, 0);
    let mut predecessors = HashMap::new();

    // do the bfs
    while let Some(next) = queue.pop_front() {
        let neighbours = [next + (0,1), next + (0, -1), next + (1, 0), next + (-1, 0)]
            .into_iter().filter(|n| !unreachable.contains(n) && n.x >= 0 && n.x <= bounds.x && n.y >= 0 && n.y <= bounds.y);

        for n in neighbours {
            if !scores.contains_key(&n) {
                scores.insert(n, scores.get(&next).unwrap() + 1);
                predecessors.insert(n, next);
                queue.push_back(n);
            }
        }
        
        // todo: do we want to break out if we hit the end?
    }

    predecessors
}

pub fn part_1(blocks: &[Coord], bounds: Coord, limit: usize) -> usize {
    let unreachable = blocks.iter().copied().take(limit).collect::<HashSet<_>>();

    let predecessors = bfs(&unreachable, bounds);

    let mut steps_on_path = HashSet::new();
    let mut current_cell = bounds;
    // steps_on_path.insert(current_cell);
    while let Some(pre) = predecessors.get(&current_cell) {
        steps_on_path.insert(*pre);
        current_cell = *pre;
    }

    // steps_on_path.insert(start);

    // for y in 0..=bounds.y {
    //     for x in 0..=bounds.x {
    //         let c: Coord = (x, y).into();
    //         if cells_on_path.contains(&c) {
    //             print!("O");
    //         } else if unreachable.contains(&c) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    steps_on_path.len()
}

pub fn part_2(blocks: &[Coord], bounds: Coord, limit: usize) -> Coord {
    let mut unreachable = blocks.iter().copied().take(limit).collect::<HashSet<_>>();

    for &block in &blocks[limit..] {
        unreachable.insert(block);

        let predecessors = bfs(&unreachable, bounds);
        match predecessors.get(&bounds) {
            Some(_) => continue,
            None => return block
        }

    }

    unreachable!();
}

pub fn parse_input(input: &str) -> Vec<Coord> {
    input.lines().map(|line| {
        let (x, y) = line.trim().split_once(",").unwrap();
        (x.parse().unwrap(), y.parse().unwrap()).into()
    }).collect()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file.trim());
    println!("Part 1: {}", part_1(&input, (70, 70).into(), 1024));
    println!("Part 2: {:?}", part_2(&input, (70, 70).into(), 1024));
}

#[test]
pub fn test() {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
    let input = parse_input(input);

    assert_eq!(part_1(&input, (6, 6).into(), 12), 22);
    assert_eq!(part_2(&input, (6, 6).into(), 12), Coord { x: 6, y: 1 });
    // no clue how to test part 2 here
}
