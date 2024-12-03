use winnow::ascii::digit1;
use winnow::combinator::{delimited, repeat, separated_pair};
use winnow::error::InputError;
use winnow::token::{take, literal};
use winnow::{PResult, Parser};

#[derive(Copy, Clone, Debug)]
pub struct Multiplication {
    left: i64,
    right: i64
}

pub fn parse_num(input: &mut &str) -> PResult<i64> {
    let digits: Vec<&str> = repeat(1..=3, digit1).parse_next(input)?;
    Ok(digits.concat().parse().unwrap())
}

pub fn parse_mul(input: &mut &str) -> PResult<Multiplication> {
    let (left, right) = delimited("mul(", separated_pair(parse_num, ",", parse_num), ")").parse_next(input)?;
    Ok(Multiplication{ left, right })
}

pub fn parse_muls(input: &mut &str) -> PResult<Vec<Multiplication>> {
    let mut results = Vec::new();
    while !input.is_empty() {
        if let Ok(mul) = parse_mul(input) {
            results.push(mul);
        } else {
            _ = take::<_, _, InputError<_>>(1usize).parse_next(input).unwrap();
        }
    }

    Ok(results)
}

pub fn parse_do<'a>(input: &mut &'a str) -> PResult<&'a str> {
    "do()".parse_next(input)
}

pub fn parse_muls_part_2(input: &mut &str) -> PResult<Vec<Multiplication>> {
    let mut results = Vec::new();
    let mut include = true;
    while !input.is_empty() {
        if let Ok(mul) = parse_mul(input) {
            if include {
                results.push(mul);
            }
        } else if literal::<_, _, InputError<_>>("do()").parse_next(input).is_ok() {
            include = true;
        } else if literal::<_, _, InputError<_>>("don't()").parse_next(input).is_ok() {
            include = false;
        } else {
            _ = take::<_, _, InputError<_>>(1usize).parse_next(input).unwrap();
        }
    }

    Ok(results)
}

pub fn parse_input(input: &str) -> Vec<Multiplication> {
    parse_muls.parse(input.trim()).unwrap()
}

pub fn parse_input_part_2(input: &str) -> Vec<Multiplication> {
    parse_muls_part_2.parse(input.trim()).unwrap()
}

pub fn calculate(muls: &[Multiplication]) -> i64 {
    muls.iter().map(|m| m.left * m.right).sum()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    println!("Part 1: {}", calculate(&input));
    let input_2 = parse_input_part_2(file);
    println!("Part 2: {}", calculate(&input_2));
}

#[test]
pub fn test() {
    let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    let muls = parse_input(input.trim());
    println!("{:?}", muls);
    assert_eq!(calculate(&muls), 161);

    let input_2 = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    let muls = parse_input_part_2(input_2.trim());
    println!("{:?}", muls);
    assert_eq!(calculate(&muls), 48);
}