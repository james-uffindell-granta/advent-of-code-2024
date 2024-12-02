use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut result = (Vec::new(), Vec::new());
    for line in input.lines() {
        if !line.is_empty() {
            let mut components = line.split_ascii_whitespace();
            // assume only two for now
            result.0.push(components.next().unwrap().parse().unwrap());
            result.1.push(components.next().unwrap().parse().unwrap());
        }
    }

    result.0.sort();
    result.1.sort();

    result
}

fn part_1((left, right): &(Vec<i32>, Vec<i32>)) -> u32 {
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn part_2((left, right): &(Vec<i32>, Vec<i32>)) -> i32 {
    let mut rhs_counts = HashMap::new();
    for r in right {
        *rhs_counts.entry(r).or_insert(0) += 1;
    }

    left.iter()
        .map(|num| num * rhs_counts.get(num).unwrap_or(&0))
        .sum()
}

fn main() {
    let input = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test() {
    let input = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;

    let input = parse_input(input);
    assert_eq!(part_1(&input), 11);
    assert_eq!(part_2(&input), 31);
}
