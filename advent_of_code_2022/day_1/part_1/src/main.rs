use std::fs;
use std::cmp;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");
    let input = input.lines();

    let mut max_cal: u32 = 0;
    let mut curr_cal:u32 = 0;

    for line in input {
        match line.trim().parse::<u32>() {
            Ok(cal) => {
                curr_cal += cal;
                println!("Adding {cal}");
            },
            Err(_) => {
                println!("Total for this elf: {curr_cal}");
                max_cal = cmp::max(max_cal, curr_cal);
                curr_cal = 0;
            }
        };
    }
                
    println!("The elf with the most calories has {max_cal} calories\n");
}
