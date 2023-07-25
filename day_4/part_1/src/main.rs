use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");
    let input = input.lines();

    let mut nums: Vec<&str>;
    let mut num_overlaps: u32 = 0;
    let mut elf_1_start: u32;
    let mut elf_1_end: u32;
    let mut elf_2_start: u32;
    let mut elf_2_end: u32;

    for line in input {
        nums = line.trim().split(&['-', ','][..]).collect();

        //println!("{:?}", nums);

        if nums.len() != 4 {
            panic!("Parsing error!");
        }

        elf_1_start = nums[0].parse().expect("Failed to parse index 0!"); 
        elf_1_end = nums[1].parse().expect("Failed to parse index 1!");
        elf_2_start = nums[2].parse().expect("Failed to parse index 2!");
        elf_2_end = nums[3].parse().expect("Failed to parse index 3!");


        if (elf_1_start >= elf_2_start && elf_1_end <= elf_2_end)
            || (elf_2_start >= elf_1_start && elf_2_end <= elf_1_end) {
            //println!("\tOverlap: {}-{} and {}-{}",
            //         nums[0], nums[1], nums[2], nums[3]);
            num_overlaps += 1;    
        }
    }


    println!("Number of overlaps: {}", num_overlaps);
}
