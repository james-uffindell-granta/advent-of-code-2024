use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Calibration {
    target: u64,
    values: Vec<u64>,
}

impl Calibration {
    pub fn is_valid(&self) -> bool {
        let mut results = HashSet::from([self.values[0]]);
        for arg in &self.values[1..] {
            results = results.iter().flat_map(|r| [r + arg, r * arg]).collect()
        }

        results.contains(&self.target)
    }

    pub fn is_valid_with_concat(&self) -> bool {
        let mut results = HashSet::from([self.values[0]]);
        for arg in &self.values[1..] {
            results = results
                .iter()
                .flat_map(|r| [r + arg, r * arg, format!("{}{}", r, arg).parse().unwrap()])
                .filter(|r| *r <= self.target)
                .collect()
        }

        results.contains(&self.target)
    }
}

pub fn parse_input(input: &str) -> Vec<Calibration> {
    input
        .lines()
        .map(|line| {
            let (target, values) = line.split_once(": ").unwrap();
            Calibration {
                target: target.parse().unwrap(),
                values: values
                    .split_ascii_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

pub fn part_1(input: &[Calibration]) -> u64 {
    input
        .iter()
        .filter(|c| c.is_valid())
        .map(|c| c.target)
        .sum()
}

pub fn part_2(input: &[Calibration]) -> u64 {
    input
        .iter()
        .filter(|c| c.is_valid_with_concat())
        .map(|c| c.target)
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
    let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
    let input = parse_input(input);
    assert_eq!(part_1(&input), 3749);
    assert_eq!(part_2(&input), 11387);
}
