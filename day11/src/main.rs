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

pub fn run(input: &[u64], blinks: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if blinks == 0 {
        return input.len();
    }

    input.iter().map(|s| {
        if let Some(answer) = cache.get(&(*s, blinks)) {
            return *answer;
        }

        let new_rocks = blink(*s).iter().map(|s| run(&[*s], blinks - 1, cache)).sum();
        cache.insert((*s, blinks), new_rocks);
        new_rocks
    }).sum()
 }

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    let mut cache = HashMap::new();
    println!("Part 1: {}", run(&input, 25, &mut cache));
    println!("Part 2: {}", run(&input, 75, &mut cache));
}

#[test]
pub fn test() {
    let input = "125 17";
    let rocks = parse_input(input);
    let mut cache = HashMap::new();
    assert_eq!(run(&rocks, 25, &mut cache), 55312);
}