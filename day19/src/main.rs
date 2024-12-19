use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Input {
    towels: Vec<String>,
    designs: Vec<String>,
}

pub fn parse_input(input: &str) -> Input {
    let (towels_part, designs_part) = input.split_once("\n\n").unwrap();
    let towels = towels_part
        .split(",")
        .map(|t| t.trim().to_string())
        .collect();
    let designs = designs_part
        .trim()
        .lines()
        .map(|line| line.to_string())
        .collect();
    Input { towels, designs }
}

pub fn ways_to_make(design: &str, towels: &[String], cache: &mut HashMap<String, usize>) -> usize {
    if let Some(known_result) = cache.get(design) {
        return *known_result;
    }

    if design.is_empty() {
        return 1;
    }

    let ways = towels
        .iter()
        .filter(|&t| design.starts_with(t))
        .map(|t| ways_to_make(&design[t.len()..], towels, cache))
        .sum();
    cache.insert(design.to_string(), ways);
    ways
}

pub fn part_1(input: &Input) -> usize {
    let mut cache = HashMap::new();
    input
        .designs
        .iter()
        .filter(|d| ways_to_make(d, &input.towels, &mut cache) > 0)
        .count()
}

pub fn part_2(input: &Input) -> usize {
    let mut cache = HashMap::new();
    input
        .designs
        .iter()
        .map(|d| ways_to_make(d, &input.towels, &mut cache))
        .sum()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file.trim());
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test() {
    let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
    let input = parse_input(input);

    assert_eq!(part_1(&input), 6);
    assert_eq!(part_2(&input), 16);
}
