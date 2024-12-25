#[derive(Clone, Debug)]
pub struct Input {
    keys: Vec<Vec<usize>>,
    locks: Vec<Vec<usize>>,
    height: usize,
}

pub fn parse_input(input: &str) -> Input {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    let mut height = 0;

    for chunk in input.split("\n\n") {
        if height == 0 {
            height = chunk.lines().count() - 2;
        }

        let lines = chunk.lines().collect::<Vec<_>>();
        let top = *lines.first().unwrap();
        let bottom = *lines.last().unwrap();

        let mut object = vec![0, 0, 0, 0, 0];
        for pin_row in &lines[1..=height] {
            for (index, c) in pin_row.chars().enumerate() {
                if c == '#' {
                    *object.get_mut(index).unwrap() += 1;
                }
            }
        }

        if top.chars().all(|c| c == '#') && bottom.chars().all(|c| c == '.') {
            // this is a lock
            locks.push(object);
        } else if bottom.chars().all(|c| c == '#') && top.chars().all(|c| c == '.') {
            // this is a key
            keys.push(object);
        } else {
            unreachable!();
        }
    }

    Input {
        keys,
        locks,
        height,
    }
}

pub fn part_1(input: &Input) -> usize {
    input
        .keys
        .iter()
        .flat_map(|k| {
            input
                .locks
                .iter()
                .map(move |l| l.iter().zip(k).map(|(lp, kp)| lp + kp).collect::<Vec<_>>())
        })
        .filter(|pins| pins.iter().all(|p| *p <= input.height))
        .count()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    use std::time::Instant;
    let now = Instant::now();
    println!("Part 1: {}", part_1(&input));
    println!("{:?}", now.elapsed());
}

#[test]
pub fn test_small() {
    let input = r#"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
"#;

    let graph = parse_input(input);
    println!("{:?}", graph);
    assert_eq!(part_1(&graph), 3);
}
