use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn get_instr_nodes(input: &str) -> (Vec<Instruction>, HashMap<String, Node>) {
    let instructions: Vec<Instruction> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => unreachable!(),
        })
        .collect();

    let mut nodes: HashMap<String, Node> = HashMap::new();
    let node_reg =
        Regex::new(r"(?P<Name>[A-Z]{3}) = \((?P<Left>[A-Z]{3}), (?P<Right>[A-Z]{3})\)").unwrap();

    for line in input.lines().skip(2) {
        let caps = node_reg.captures(line).unwrap();

        let name = caps.name("Name").unwrap().as_str().to_string();
        let left = caps.name("Left").unwrap().as_str().to_string();
        let right = caps.name("Right").unwrap().as_str().to_string();
        nodes.insert(name.clone(), Node { left, right });
    }

    (instructions, nodes)
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read the input file.");

    let (instr, nodes) = get_instr_nodes(&input);

    let mut n_steps = 0usize;
    let mut instr_idx = 0usize;
    let mut curr_node = "AAA";

    loop {
        if curr_node.eq("ZZZ") {
            break;
        }

        match instr[instr_idx] {
            Instruction::Left => {
                curr_node = &nodes.get(curr_node).unwrap().left;
            }
            Instruction::Right => {
                curr_node = &nodes.get(curr_node).unwrap().right;
            }
        }

        instr_idx = (n_steps + 1) % instr.len();
        n_steps += 1;
    }

    println!("{} steps", n_steps);
}
