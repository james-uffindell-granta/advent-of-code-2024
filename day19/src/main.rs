use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Input {
    towels: HashMap<u8, Vec<String>>,
    designs: Vec<String>,
}

pub fn parse_input(input: &str) -> Input {
    let (towels_part, designs_part) = input.split_once("\n\n").unwrap();

    let mut towels = HashMap::new();
    for towel in towels_part.split(",") {
        let towel = towel.trim();
        towels.entry(towel.as_bytes()[0]).or_insert(Vec::new()).push(towel.to_string());
    }

    let designs = designs_part.trim().lines().map(|line| line.to_string()).collect();

    Input { towels, designs }
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
    let mut cache = HashMap::new();
    input.designs.iter().filter(|d| ways_to_make(d, &input.towels, &mut cache) > 0).count()
}

pub fn part_2(input: &Input) -> usize {
    let mut cache = HashMap::new();
    input.designs.iter().map(|d| ways_to_make(d, &input.towels, &mut cache)).sum()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file.trim());
    use std::time::Instant;
    let now = Instant::now();
    println!("Part 1: {}", part_1(&input));
    println!("{:?}", now.elapsed());
    let now = Instant::now();
    println!("Part 2: {}", part_2(&input));
    println!("{:?}", now.elapsed());
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
