#![feature(int_roundings)]

use winnow::ascii::digit1;
use winnow::combinator::{alt, delimited, preceded, repeat, separated, separated_pair};
use winnow::token::take;
use winnow::{PResult, Parser};

pub fn parse_button(input: &mut &str) -> PResult<(i64, i64)> {
    delimited(("Button ", take(1u32), ": X+"),
        separated_pair(digit1.parse_to(), ", Y+", digit1.parse_to()),
    "\n").parse_next(input)
}

pub fn parse_prize(input: &mut &str) -> PResult<(i64, i64)> {
    preceded("Prize: X=",
        separated_pair(digit1.parse_to(), ", Y=", digit1.parse_to())).parse_next(input)
}

pub fn parse_machine(input: &mut &str) -> PResult<ClawMachine> {
    let button_a = parse_button(input)?;
    let button_b = parse_button(input)?;
    let prize = parse_prize(input)?;
    Ok(ClawMachine { button_a, button_b, prize })
}

#[derive(Copy, Clone, Debug)]
pub struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64)
}

/// find the values satisfying Bezout's Identity: as + bt = r = gcd(a, b)
/// returns (r, s, t)
pub fn bezout(a: i64, b: i64) -> (i64, i64, i64) {
    let mut r = (a, b);
    let mut s = (1, 0);
    let mut t = (0, 1);

    while r.1 != 0 {
        let q = r.0 / r.1;
        let new_r = r.0 - q * r.1;
        let new_s = s.0 - q * s.1;
        let new_t = t.0 - q * t.1;
        r = (r.1, new_r);
        s = (s.1, new_s);
        t = (t.1, new_t);
    }

    (r.0, s.0, t.0)
}


impl ClawMachine {
    /// returns p_a, p_b, offset where solutions that work for x are (p_a - k*offset, p+b + k*offset) for all k
    pub fn get_x_solutions(&self) -> Option<(i64, i64, (i64, i64))> {
        // we need to find numbers (s, t) such that s * button_a.0 + t * button_b.0 == prize.0
        let (r, s, t) = bezout(self.button_a.0, self.button_b.0);
        if self.prize.0 % r != 0 {
            // only multiples of the gcd are possible solutions
            return None;
        }

        let repeats = self.prize.0 / r;

        let a_presses = repeats * s;
        let b_presses = repeats * t;

        let offset = (self.button_b.0 / r, self.button_a.0 / r);

        if a_presses < 0 && b_presses < 0 {
            // if both values are negative then there are no solutions since we know the prize is positive
            return None;
        }

        Some((a_presses, b_presses, offset))
    }

    pub fn win(&self) -> Option<i64> {
        // get the solution for x (if there is one)
        // solutions are (a_presses - k*offsets.0) presses of a and (b_presses + k*offsets.1) presses of b
        let (mut a_presses, mut b_presses, offsets) = self.get_x_solutions()?;

        // solve for k - figure out which value would give us a working y solution
        // if there actually isn't a solution then we'll get a spurious k answer here that doesn't actually work
        // but if there _is_ a solution, then this is it
        let naive_y_value = a_presses * self.button_a.1 + b_presses * self.button_b.1;
        let k = (self.prize.1 - naive_y_value) / (self.button_b.1 * offsets.1 - self.button_a.1 * offsets.0);
        // adjust the number of presses we'd need by that much
        (a_presses, b_presses) = (a_presses - k * offsets.0, b_presses + k * offsets.1); 


        if a_presses * self.button_a.0 + b_presses * self.button_b.0 != self.prize.0 {
            // should be impossible - all values of k work for x - but just to be safe
            unreachable!();
        }

        if a_presses * self.button_a.1 + b_presses * self.button_b.1 == self.prize.1 {
            // double check that our value of k does indeed work for y
            Some(a_presses * 3 + b_presses)
        } else {
            None
        }
    }

    pub fn adjust(&self) -> Self {
        Self {
            button_a: self.button_a,
            button_b: self.button_b,
            prize: (10_000_000_000_000 + self.prize.0, 10_000_000_000_000 + self.prize.1)
        }
    }
}


pub fn parse_items(input: &mut &str) -> PResult<Vec<ClawMachine>> {
    separated(1.., parse_machine, "\n\n")
    .parse_next(input)
}

pub fn parse_input(input: &str) -> Vec<ClawMachine> {
    parse_items
        .parse(input.trim())
        .unwrap()
}

pub fn part_1(input: &[ClawMachine]) -> i64 {
    input.iter().filter_map(|m| m.win()).sum()
}

pub fn part_2(input: &[ClawMachine]) -> i64 {
    input.iter().map(|m| m.adjust()).filter_map(|m| m.win()).sum()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test_1() {
    let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    let input = parse_input(input);

    assert_eq!(part_1(&input), 480);
}