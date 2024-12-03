use winnow::ascii::digit1;
use winnow::combinator::{alt, delimited, repeat, separated_pair};
use winnow::token::take;
use winnow::{PResult, Parser};

#[derive(Copy, Clone, Debug)]
pub enum ItemOfInterest {
    Multiplication(i64, i64),
    Do,
    Dont,
}

pub fn parse_num(input: &mut &str) -> PResult<i64> {
    repeat(1..=3, digit1)
        .map(|d: Vec<&str>| d.concat().parse().unwrap())
        .parse_next(input)
}

pub fn parse_items(input: &mut &str) -> PResult<Vec<Option<ItemOfInterest>>> {
    repeat(
        1..,
        alt((
            delimited("mul(", separated_pair(parse_num, ",", parse_num), ")")
                .map(|(left, right)| Some(ItemOfInterest::Multiplication(left, right))),
            "do()".map(|_| Some(ItemOfInterest::Do)),
            "don't()".map(|_| Some(ItemOfInterest::Dont)),
            take(1usize).map(|_| None),
        )),
    )
    .parse_next(input)
}

pub fn parse_input_full(input: &str) -> Vec<ItemOfInterest> {
    parse_items
        .parse(input.trim())
        .unwrap()
        .into_iter()
        .flatten()
        .collect()
}

pub fn part_1(input: &[ItemOfInterest]) -> i64 {
    input
        .iter()
        .filter_map(|i| {
            if let ItemOfInterest::Multiplication(left, right) = i {
                Some(left * right)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_2(input: &[ItemOfInterest]) -> i64 {
    let mut include = true;
    let mut result = 0;
    for i in input {
        match i {
            ItemOfInterest::Multiplication(left, right) if include => result += left * right,
            ItemOfInterest::Multiplication(_, _) => {}
            ItemOfInterest::Do => include = true,
            ItemOfInterest::Dont => include = false,
        }
    }

    result
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input_full(file);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test() {
    let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    let muls = parse_input_full(input.trim());
    assert_eq!(part_1(&muls), 161);

    let input_2 = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    let muls = parse_input_full(input_2.trim());
    assert_eq!(part_2(&muls), 48);
}
