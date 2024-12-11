use std::collections::HashMap;

pub fn blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        vec![1]
    } else {
        let printed_num = format!("{}", stone);
        if printed_num.len() % 2 == 0 {
            vec![printed_num[..printed_num.len() / 2].parse().unwrap(),
            printed_num[printed_num.len() / 2 ..].parse().unwrap()]
        } else {
            vec![stone * 2024]
        }
    }
}

pub fn parse_input(input: &str) -> Vec<u64> {
    input.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect()
}

pub fn blink_25(input: &[u64]) -> Vec<u64> {
    let mut stones = input.to_owned();
    for _ in 1..=25 {
        stones = stones.iter().flat_map(|s| blink(*s)).collect();
        // println!("{:?}", stones);
    }

    stones
}

pub fn part_1(input: &[u64]) -> usize {
    let mut stones = input.to_owned();
    for _ in 1..=25 {
        stones = stones.iter().flat_map(|s| blink(*s)).collect();
        // println!("{:?}", stones);
    }

    stones.len()
}

pub fn part_1_with_cache(input: &[u64], cache: &mut HashMap<u64, usize>) -> usize {
    input.iter().map(|s| num_25_ahead_with_cache(*s, cache)).sum()
}

pub fn num_25_ahead_with_cache(stone: u64, cache: &mut HashMap<u64, usize>) -> usize {
    if let Some(result) = cache.get(&stone) {
        return *result;
    }

    let stones = blink_25(&[stone]);

    let result = stones.len();
    cache.insert(stone, result);
    result
}

pub fn num_50_ahead_with_cache(stone: u64, cache_25: &mut HashMap<u64, usize>, cache_50: &mut HashMap<u64, usize>) -> usize {
    if let Some(result) = cache_50.get(&stone) {
        return *result;
    }

    let stones = blink_25(&[stone]);

    // just in case
    cache_25.insert(stone, stones.len());

    let result = part_1_with_cache(&stones, cache_25);
    cache_50.insert(stone, result);
    result
}

pub fn part_2(input: &[u64], cache_25: &mut HashMap<u64, usize>) -> usize {
    let mut cache_50 = HashMap::new();
    blink_25(input).iter().map(|s| num_50_ahead_with_cache(*s, cache_25, &mut cache_50)).sum()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    let mut cache = HashMap::new();
    println!("Part 1: {}", part_1(&input));
    println!("Part 1: {}", part_1_with_cache(&input, &mut cache));
    println!("Part 2: {}", part_2(&input, &mut cache));
}

#[test]
pub fn test() {
    let input = "125 17";
    let rocks = parse_input(input);
    assert_eq!(part_1(&rocks), 55312);
}
