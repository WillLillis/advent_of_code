use std::fs;

struct Submarine {
    horizontal_pos: i32,
    depth: i32
}

impl Submarine {
    fn new() -> Self {
        Submarine { horizontal_pos: 0, depth: 0 }
    }

    fn follow_instruction(&mut self, instr: &Instruction) {
        match instr {
            &Instruction::Forward(val) => {
                self.horizontal_pos += val as i32;
            },
            &Instruction::Down(val) => {
                self.depth += val as i32;
            },
            &Instruction::Up(val) => {
                self.depth -= val as i32;
            }
        }
    }
}

enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32)
}

impl Instruction {
    fn new(instr: &str, val: u32) -> Self {
        match instr {
            "forward" => {
                Instruction::Forward(val)
            },
            "down" => {
                Instruction::Down(val)
            },
            "up" => {
                Instruction::Up(val)
            },
            _ => {
                panic!("Invalid string passed to Instruction::new()!");
            }
        }
    }
}


fn get_instructions(file_name: &str) -> Vec<Instruction> {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file");

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() == 2);
        instructions.push(Instruction::new(parts[0], parts[1].parse::<u32>().unwrap()));
    }

    return instructions;
}


fn main() {
    let mut sub = Submarine::new();
    let instructions = get_instructions("input.txt");
    
    for instr in instructions.iter() {
        sub.follow_instruction(instr);
    }

    println!("Final horizontal position: {}\nFinal Depth: {}",
             sub.horizontal_pos, sub.depth);

    println!("Multiplied result: {}", sub.horizontal_pos * sub.depth);
}
