use std::collections::HashMap;


#[derive(Clone, Debug, Hash)]
pub struct Input {
    towels: Vec<String>,
    designs: Vec<String>,
}

impl Input {

}

pub fn parse_input(input: &str) -> Input {
    let (towels_part, designs_part) = input.split_once("\n\n").unwrap();

    let towels = towels_part.split(",").map(|s| s.trim().to_string()).collect();
    let designs = designs_part.trim().lines().map(|line| line.to_string()).collect();

    Input { towels, designs }
}

pub fn can_make(design: &str, towels_by_char: &HashMap<u8, Vec<String>>, cache: &mut HashMap<String, bool>) -> bool {
    if let Some(known_result)  = cache.get(design) {
        return *known_result;
    }

    if design.is_empty() {
        return true;
    }

    // otherwise see what we can do
    let first_byte = design.as_bytes()[0];
    if let Some(possible_towels) = towels_by_char.get(&first_byte) {
        for t in possible_towels {
            if design.starts_with(t) {
                let rest = &design[t.len()..];
                let answer_rest = can_make(rest, towels_by_char, cache);
                if answer_rest {
                    // we can make it starting with this towel
                    cache.insert(design.to_string(), true);
                    return true;
                }

                // otherwise we can't - try the next one
            }
        }
    }

    cache.insert(design.to_string(), false);
    false
}

pub fn ways_to_make(design: &str, towels_by_char: &HashMap<u8, Vec<String>>, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(known_result)  = cache.get(design) {
        return *known_result;
    }

    if design.is_empty() {
        return 1;
    }

    // otherwise see what we can do
    let mut ways = 0;
    let first_byte = design.as_bytes()[0];
    if let Some(possible_towels) = towels_by_char.get(&first_byte) {
        for t in possible_towels {
            if design.starts_with(t) {
                let rest = &design[t.len()..];
                let answer_rest = ways_to_make(rest, towels_by_char, cache);
                if answer_rest > 0 {
                    // we can make it starting with this towel
                    ways += answer_rest
                }

                // otherwise we can't - try the next one
            }
        }
    }

    cache.insert(design.to_string(), ways);
    ways
}

pub fn part_1(input: &Input) -> usize {
    let mut towels_by_char = HashMap::new();
    for towel in &input.towels {
        towels_by_char.entry(towel.as_bytes()[0]).or_insert(Vec::new()).push(towel.clone());
    }

    let mut cache = HashMap::new();

    let mut result = 0;
    for design in &input.designs {
        if can_make(&design, &towels_by_char, &mut cache) {
            result += 1;
        }
    }

    result
}

pub fn part_2(input: &Input) -> usize {
    let mut towels_by_char = HashMap::new();
    for towel in &input.towels {
        towels_by_char.entry(towel.as_bytes()[0]).or_insert(Vec::new()).push(towel.clone());
    }

    let mut cache = HashMap::new();

    let mut result = 0;
    for design in &input.designs {
        result += ways_to_make(design, &towels_by_char, &mut cache)
    }

    result
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file.trim());
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {:?}", part_2(&input));
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
