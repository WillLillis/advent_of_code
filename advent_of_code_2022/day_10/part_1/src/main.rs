use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let mut reg_v = 1;
    let mut clock_cycle = 1;
    let mut sig_strength = 0;

    for line in input.lines() {
        let mut instr = line.trim().split_whitespace();
        let op = instr.next().unwrap();

        match op {
            "noop" => {
                assert!(instr.next() == None);
                clock_cycle += 1;
            },
            "addx" => {
                clock_cycle += 1;
                if clock_cycle == 20 || clock_cycle == 60 || clock_cycle == 100
                    || clock_cycle == 140 || clock_cycle == 180 || clock_cycle == 220 {
                    sig_strength += reg_v * clock_cycle;
                }
                let x = instr.next().unwrap().parse::<i32>().unwrap();
                reg_v += x;
                clock_cycle += 1;
            },
            _ => {
                panic!("Invalid instruction!");
            }
        }

        if clock_cycle == 20 || clock_cycle == 60 || clock_cycle == 100
            || clock_cycle == 140 || clock_cycle == 180 || clock_cycle == 220 {
            sig_strength += reg_v * clock_cycle;
        }
    }

    println!("Signal strength: {sig_strength}");
}
