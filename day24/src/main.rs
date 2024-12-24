use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Gate {
    left: String,
    right: String,
    output: String,
    gate_type: GateType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    starting_values: HashMap<String, bool>,
    gates: Vec<Gate>,
}

impl Input {
    pub fn swap(&mut self, wire_1: String, wire_2: String) {
        for gate in self.gates.iter_mut() {
            if gate.output == wire_1 {
                gate.output = wire_2.clone();
            } else if gate.output == wire_2 {
                gate.output = wire_1.clone();
            }
        }
    }

    pub fn generate_adder_clash(&self) -> Option<(String, String)> {
        let mut correct_outputs = HashSet::new();
        let mut defined_gates = HashMap::new();
        for g in &self.gates {
            defined_gates.insert(
                (g.left.clone(), g.right.clone(), g.gate_type),
                g.output.clone(),
            );
            defined_gates.insert(
                (g.right.clone(), g.left.clone(), g.gate_type),
                g.output.clone(),
            );
        }

        let mut prev_carry: Option<String> = None;

        for digit in 0..=44 {
            // println!("Correct rules so far: {:?}", correct_outputs);
            let x = format!("x{:0>2}", digit);
            let y = format!("y{:0>2}", digit);
            let z = format!("z{:0>2}", digit);

            // find the add rule
            let Some(add_rule) = defined_gates.get(&(x.clone(), y.clone(), GateType::Xor)) else {
                unreachable!();
            };
            // find the carry rule
            let Some(carry_rule) = defined_gates.get(&(x.clone(), y.clone(), GateType::And)) else {
                unreachable!();
            };

            // println!("{} add {} is {}, carry {}", x, y, add_rule, carry_rule);
            // now we need to be a bit more careful - outputs could be swapped
            if let Some(ref prev_carry_rule) = prev_carry {
                // println!("previous digit carry was {}", prev_carry_rule);
                // check we're combining with the previous carry correctly
                if let Some(full_add_rule) =
                    defined_gates.get(&(add_rule.clone(), prev_carry_rule.clone(), GateType::Xor))
                {
                    // we have a correct final add rule - this should be the z digit
                    // println!("{} and {} gives digit {}", add_rule, prev_carry_rule, full_add_rule);
                    if full_add_rule != &z {
                        return Some((full_add_rule.clone(), z));
                    } else {
                        correct_outputs.insert(full_add_rule.clone());
                        correct_outputs.insert(add_rule.clone());
                        correct_outputs.insert(prev_carry_rule.clone());
                    }
                } else {
                    // find the full add rule and hope it isn't also wrong
                    if let Some(real_rule) = self
                        .gates
                        .iter()
                        .find(|g| g.output == z && g.gate_type == GateType::Xor)
                    {
                        if add_rule == &real_rule.left {
                            return Some((prev_carry_rule.clone(), real_rule.right.clone()));
                        } else if add_rule == &real_rule.right {
                            return Some((prev_carry_rule.clone(), real_rule.left.clone()));
                        } else if prev_carry_rule == &real_rule.left {
                            return Some((add_rule.clone(), real_rule.right.clone()));
                        } else if prev_carry_rule == &real_rule.right {
                            return Some((add_rule.clone(), real_rule.left.clone()));
                        } else {
                            // both must be swapped somehow?!
                            unimplemented!();
                        }
                    } else {
                        println!(
                            "No full add rule: one of {} and {} was swapped with something",
                            add_rule, prev_carry_rule
                        );
                        unimplemented!();
                    }
                }

                if let Some(combined_carry_rule) =
                    defined_gates.get(&(add_rule.clone(), prev_carry_rule.clone(), GateType::And))
                {
                    // println!("First carry: {} and {} carries {}", add_rule, prev_carry_rule, combined_carry_rule);
                    correct_outputs.insert(combined_carry_rule.clone());
                    if let Some(full_carry_rule) = defined_gates.get(&(
                        carry_rule.clone(),
                        combined_carry_rule.clone(),
                        GateType::Or,
                    )) {
                        prev_carry = Some(full_carry_rule.clone());
                    } else {
                        println!(
                            "No full carry rule: one of {} and {} was swapped with something",
                            prev_carry_rule, combined_carry_rule
                        );
                        unimplemented!();
                    }
                } else {
                    // couldn't find a matching rule
                    println!(
                        "No combined carry rule: one of {} and {} was swapped with something",
                        add_rule, prev_carry_rule
                    );
                    unimplemented!();
                }
            } else {
                if add_rule != "z00" {
                    return Some((add_rule.clone(), "z00".to_string()));
                } else {
                    correct_outputs.insert(add_rule.clone());
                }

                prev_carry = Some(carry_rule.clone());
            }
        }

        if prev_carry != Some("z45".to_string()) {
            return Some((prev_carry.unwrap(), "z45".to_string()));
        }

        None
    }
}

pub fn parse_input(input: &str) -> Input {
    let (inputs, gates_part) = input.split_once("\n\n").unwrap();
    let mut gates = Vec::new();
    let mut starting_values = HashMap::new();
    for line in inputs.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        let int_value = right.parse::<u32>().unwrap();
        let value = if int_value == 1 {
            true
        } else if int_value == 0 {
            false
        } else {
            unreachable!()
        };
        starting_values.insert(left.to_string(), value);
    }

    for line in gates_part.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();
        // just use other for now
        let output = right.to_string();
        let parts = left.split_ascii_whitespace().collect::<Vec<_>>();
        let left = parts[0].to_string();
        let right = parts[2].to_string();
        let gate_type = match parts[1] {
            "AND" => GateType::And,
            "OR" => GateType::Or,
            "XOR" => GateType::Xor,
            _ => unreachable!(),
        };
        gates.push(Gate {
            left,
            right,
            output,
            gate_type,
        });
    }

    Input {
        starting_values,
        gates,
    }
}

pub fn calculate(input: &Input) -> HashMap<String, bool> {
    let mut values = input.starting_values.clone();
    let mut added = true;
    // todo figure out condition
    while added {
        added = false;
        for rule in &input.gates {
            if let Some(left) = values.get(&rule.left) {
                if let Some(right) = values.get(&rule.right) {
                    // we know both the input values
                    let answer = match rule.gate_type {
                        GateType::And => left & right,
                        GateType::Or => left | right,
                        GateType::Xor => left ^ right,
                    };
                    if values.insert(rule.output.clone(), answer).is_none() {
                        added = true;
                    }
                }
            }
        }
    }

    values
}

pub fn part_1(input: &Input) -> u64 {
    let answers = calculate(input);
    let mut z = 0u64;
    for (key, value) in answers {
        if let Some(number) = key.strip_prefix('z') {
            let digit = number.parse::<usize>().unwrap();
            if value {
                z += 2_u64.pow(digit as u32);
            }
        }
    }

    z
}

pub fn part_2(input: &Input) -> String {
    let mut input = input.clone();
    let mut pairs = Vec::new();
    while let Some((left, right)) = input.generate_adder_clash() {
        pairs.push(left.clone());
        pairs.push(right.clone());
        input.swap(left, right);
    }

    pairs.sort();
    pairs.join(",")
}

fn main() {
    let file = include_str!("../input.txt");
    let input = parse_input(file);
    use std::time::Instant;
    let now = Instant::now();
    println!("Part 1: {}", part_1(&input));
    println!("{:?}", now.elapsed());
    let now = Instant::now();
    println!("Part 2: {}", part_2(&input));
    println!("{:?}", now.elapsed());
}

#[test]
pub fn test_small() {
    let input = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;

    let graph = parse_input(input);
    println!("{:?}", graph);
    assert_eq!(part_1(&graph), 4);
}

#[test]
pub fn test_large() {
    let input = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;

    let graph = parse_input(input);
    assert_eq!(part_1(&graph), 2024);
}
