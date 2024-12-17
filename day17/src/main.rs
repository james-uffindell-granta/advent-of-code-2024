use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    numbers: Vec<u8>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Computer {
    program: Program,
    instruction_pointer: usize,
    registers: Registers,
    output: Vec<u64>,
}

impl Computer {
    pub fn step(&self) -> Option<Computer> {
        if self.instruction_pointer >= self.program.numbers.len() - 1 {
            return None;
        }

        let mut new_state = self.clone();

        let opcode = self.program.numbers[self.instruction_pointer];
        let literal_operand = self.program.numbers[self.instruction_pointer + 1] as u64;

        let combo_operand = match literal_operand {
            0..=3 => literal_operand,
            4 => self.registers.a,
            5 => self.registers.b,
            6 => self.registers.c,
            _ => unreachable!(),
        };

        let mut jumped = false;
        match opcode {
            0 => {
                new_state.registers.a = self.registers.a >> combo_operand;
            }
            1 => {
                new_state.registers.b = self.registers.b ^ literal_operand;
            }
            2 => {
                new_state.registers.b = combo_operand & 0b0111;
            }
            3 => {
                if self.registers.a != 0 {
                    new_state.instruction_pointer = literal_operand.try_into().unwrap();
                    jumped = true;
                }
            }
            4 => {
                new_state.registers.b = self.registers.b ^ self.registers.c;
            }
            5 => {
                new_state.output.push(combo_operand % 8);
            }
            6 => {
                new_state.registers.b = self.registers.a >> combo_operand;
            }
            7 => {
                new_state.registers.c = self.registers.a >> combo_operand;
            }
            _ => unreachable!(),
        }

        if !jumped {
            new_state.instruction_pointer += 2;
        }

        Some(new_state)
    }

    pub fn run(&self) -> Vec<u64> {
        let mut state = self.clone();
        while let Some(new_state) = state.step() {
            state = new_state;
        }
        state.output.clone()
    }
}

pub fn parse_input(input: &str) -> Computer {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let mut register_lines = registers.lines();
    let (_, reg_a) = register_lines.next().unwrap().split_once(": ").unwrap();
    let (_, reg_b) = register_lines.next().unwrap().split_once(": ").unwrap();
    let (_, reg_c) = register_lines.next().unwrap().split_once(": ").unwrap();
    let (_, program) = program.split_once(": ").unwrap();

    Computer {
        program: Program {
            numbers: program
                .trim()
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect(),
        },
        instruction_pointer: 0,
        registers: Registers {
            a: reg_a.parse().unwrap(),
            b: reg_b.parse().unwrap(),
            c: reg_c.parse().unwrap(),
        },
        output: Vec::new(),
    }
}

pub fn part_1(input: &Computer) -> String {
    let output = input.run();
    output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn part_2(input: &Computer) -> u64 {
    // my program: 2,4,1,3,7,5,4,0,1,3,0,3,5,5,3,0
    // this program is:
    // let b = a & 111;     2,4   b == final three digits of a
    // b ^= 3;              1,3   (*) twiddle b somehow (still 0 <= b <= 7)
    // c = a >> b;          7,5   c == a shifted right between 0 and 7 places
    // b ^= c;              4,0   (**) twiddle b somehow with c
    // b ^= 3;              1,3   twiddle b somehow (cancels previous ^= 3)
    // a >>= 3;             0,3   (***) shift a right by three, loses last three digits
    // print b % 8          5,5   print number formed by last three digits of b
    // if a != 0 repeat     3,0   go back to start
    // so:
    // each loop iteration we print one digit, and then knock the final three digits off a.
    // by (*) b is between 0 and 7 inclusive
    // c then shifts this many steps - and we will then use the final three digits of what's left to ^ with b
    // so the final 10 binary digits of a are relevant each loop

    // the rest of the program does stuff to b and prints its final three digits - but note:
    // only the last ten (binary) digits of a are relevant this loop

    // after this loop we knock three digits off a and repeat - which means the next loop's final 7 digits
    // of 10 need to match this loop's first 7 digits of 10 (for a)

    // figure out the results of all possible 'last 10 digits of a' - we can then lift these up and combine
    // them into the real a
    let mut result_map = HashMap::new();
    for possible_a in 0..=1023u64 {
        let computer = Computer {
            registers: Registers {
                a: possible_a,
                b: 0,
                c: 0,
            },
            output: Vec::new(),
            instruction_pointer: 0,
            program: input.program.clone(),
        };
        let output = computer.run();
        result_map
            .entry(output[0])
            .or_insert(BTreeSet::new())
            .insert(possible_a);
    }

    let mut possible_answers = HashSet::new();
    let things_to_print = input
        .program
        .numbers
        .iter()
        .map(|n| *n as u64)
        .collect::<Vec<_>>();

    // go through, figure out for each digit what a values would work to print that
    for (magnitude, digit) in things_to_print.iter().enumerate() {
        let answers_for_this_digit = result_map.get(digit).unwrap();
        if magnitude == 0 {
            for a in answers_for_this_digit {
                possible_answers.insert(*a);
            }
        } else {
            // if we're not on the first digit, we need to figure out what values of a would cause the previous digits
            // to be printed and also for this digit to be printed next
            // so we know that we want a to be ending .....ddddddd for this digit to be printed next
            // and we know what possible ...ddddxxx... values would have caused previous values to be printed
            let mut new_possible_answers = HashSet::new();
            // the values of a which cause the previous digits to be printed - we need to shift these left
            for old_possibility in possible_answers {
                let ignoring_end = old_possibility >> (3 * magnitude);
                for extension in answers_for_this_digit {
                    // the values of 'last 10 digits of a' which would cause this digit to be printed next -
                    // we need to keep all possibilities where the first seven (binary) digits of this extension
                    // match the last seven (binary) digits of the old possibility for previous output
                    if (*extension ^ ignoring_end) % 128 == 0 {
                        // and then combine them by overlapping those digits
                        let new_possibility = (*extension << (3 * magnitude)) | old_possibility;
                        new_possible_answers.insert(new_possibility);
                    }
                }
            }

            possible_answers = new_possible_answers;
        }
    }

    // and we have to have stopped after this point precisely:
    // if there are any digits left after this they would lead to extra output,
    // and if the first three digits of a are zeroes then they wouldn't print the final output value
    possible_answers.retain(|answer| {
        answer >> (3 * things_to_print.len()) == 0
            && answer >> (3 * (things_to_print.len() - 1)) != 0
    });

    possible_answers.into_iter().min().unwrap()
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file.trim());
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[test]
pub fn test() {
    let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
    let input = parse_input(input);

    assert_eq!(part_1(&input), "4,6,3,5,6,3,5,2,1,0");
    // no clue how to test part 2 here
}

#[test]
pub fn test_1() {
    let computer = Computer {
        registers: Registers { a: 0, b: 0, c: 9 },
        program: Program {
            numbers: vec![2, 6],
        },
        instruction_pointer: 0,
        output: Vec::new(),
    };

    let new_state = computer.step();

    assert_eq!(
        new_state,
        Some(Computer {
            registers: Registers { a: 0, b: 1, c: 9 },
            program: Program {
                numbers: vec![2, 6]
            },
            instruction_pointer: 2,
            output: Vec::new(),
        })
    );

    let newer_state = new_state.unwrap().step();
    assert_eq!(newer_state, None);
}

#[test]
pub fn test_2() {
    let computer = Computer {
        registers: Registers { a: 10, b: 0, c: 0 },
        program: Program {
            numbers: vec![5, 0, 5, 1, 5, 4],
        },
        instruction_pointer: 0,
        output: Vec::new(),
    };

    let output = computer.run();

    assert_eq!(output, vec![0, 1, 2]);
}

#[test]
pub fn test_3() {
    let computer = Computer {
        registers: Registers {
            a: 2024,
            b: 0,
            c: 0,
        },
        program: Program {
            numbers: vec![0, 1, 5, 4, 3, 0],
        },
        instruction_pointer: 0,
        output: Vec::new(),
    };

    let output = computer.run();

    assert_eq!(output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
}
