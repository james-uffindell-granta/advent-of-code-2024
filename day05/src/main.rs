use std::{cmp::Ordering, collections::{HashMap, HashSet}};

pub struct Input {
    rules: HashMap<i64, HashSet<i64>>,
    updates: Vec<Vec<i64>>,
}

pub fn compare_using(rules: &HashMap<i64, HashSet<i64>>, a: i64, b: i64) -> Ordering {
    match rules.get(&a) {
        // there is a rule saying a|b
        Some(a_rules) if a_rules.contains(&b) => Ordering::Less,
        // there aren't any rules saying that a|b; check b's rules
        _ => match rules.get(&b) {
            // there is a rule saying b|a
            Some(b_rules) if b_rules.contains(&a) => Ordering::Greater,
            // otherwise there's no rule relating a to b (would need to toposort to infer relationship)
            _ => unreachable!(),
        }
    }
}

pub fn part_1(input: &Input) -> i64 {
    let mut answer = 0;

    for update in &input.updates {
        if update.is_sorted_by(|a, b| compare_using(&input.rules, *a, *b).is_le()) {
            answer += update[update.len() / 2];
        }
    }

    answer
}

pub fn part_2(input: &Input) -> i64 {
    let mut answer = 0;

    for update in &input.updates {
        let mut copy = update.clone();
        copy.sort_by(|a, b| compare_using(&input.rules, *a, *b));

        if update != &copy {
            answer += copy[copy.len() / 2];
        }
    }

    answer
}

pub fn parse_input(input: &str) ->  Input {
    let (rules_part, updates_part) = input.split_once("\n\n").unwrap();
    let mut rules : HashMap<i64, HashSet<i64>> = HashMap::new();
    for rule in rules_part.lines() {
        let (first, second) = rule.split_once("|").unwrap();
        rules.entry(first.parse().unwrap()).or_default().insert(second.parse().unwrap());
    }

    let updates = updates_part.lines().map(|line| line.split(",").map(|num| num.parse().unwrap()).collect()).collect();

    Input { rules, updates }
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test() {
    let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#;

    let input = parse_input(input);
    assert_eq!(part_1(&input), 143);
    assert_eq!(part_2(&input), 123);
}
