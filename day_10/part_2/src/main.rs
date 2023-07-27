use std::fs;

pub fn draw_curr_pixel(sprite_pos: i32, clock_cycle: i32) {
    if sprite_pos == ((clock_cycle - 1) % 40) {
        print!("#");
    } else if (sprite_pos + 1) == ((clock_cycle - 1) % 40) {
        print!("#");
    } else if (sprite_pos - 1) == ((clock_cycle - 1) % 40) {
        print!("#");
    } else {
        print!(".");
    }
    if clock_cycle % 40 == 0 {
        println!("");
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let mut reg_v = 1;
    let mut clock_cycle = 1;

    draw_curr_pixel(reg_v, clock_cycle); // need to draw the initial state

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
                draw_curr_pixel(reg_v, clock_cycle);
                let x = instr.next().unwrap().parse::<i32>().unwrap();
                reg_v += x;
                clock_cycle += 1;
            },
            _ => {
                panic!("Invalid instruction!");
            }
        }
        
        draw_curr_pixel(reg_v, clock_cycle)
    }

}
