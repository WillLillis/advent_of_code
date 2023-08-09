// ran into lots of issues just trying to remove and insert the elements
// went with the shifting approach next
use std::fs;

fn get_nums(file_name: &str) -> Vec<i32> {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file.");

    let mut nums: Vec<i32> = Vec::new();
    
    for line in input.lines() {
        nums.push(line.trim().parse::<i32>().unwrap());
    }
    
    return nums;
}


fn main() {
    let nums = get_nums("input.txt");

    // - tried several different approaches here and struggled, code always worked on the test input 
    // but gave the wrong answer on the full input
    // - Eventually gave up and tried looking online for help, found this:
    // https://nickymeuleman.netlify.app/garden/aoc2022-day20https://nickymeuleman.netlify.app/garden/aoc2022-day20
    let mut mixed: Vec<usize> = (0..nums.len()).collect();
    for (i, num) in nums.iter().enumerate() {
        let mixed_idx = mixed.iter().position(|idx| *idx == i).unwrap(); // find the index of where the current
                                                                        // number would be in the
                                                                        // shuffled array

        mixed.remove(mixed_idx); // and remove it

        let new_idx = (mixed_idx as i32 + num).rem_euclid(mixed.len() as i32) as usize; // calculate
                                                                                        // the new
                                                                                        // index
        
        mixed.insert(new_idx, i); // and insert it
    }

    let zero_idx = nums // find where 0 is in the original list
        .iter()
        .position(|&num| num == 0)
        .unwrap();

    let zero_mixed_idx = mixed // and then the index in the shuffled list
        .iter()
        .position(|&mix_num| mix_num == zero_idx)
        .unwrap();

    
    let total: i32 = [1000, 2000, 3000]
        .iter()
        .map(|offset| {
            let mixed_idx = (zero_mixed_idx + offset) % mixed.len(); // index 1000, 2000, 3000 up from 0's index...
            let nums_idx = mixed[mixed_idx]; // and then translate that index over to the nums array 
            nums[nums_idx] // and finally get the number stored at that index
        })
        .sum();
    //let sum: i32 = coords.iter().sum();
    //
    //println!("I'm stuck: {}, {}, {}", nums[2752], nums[3752], nums[4752]);

    println!("Total: {:?}", total);
    //println!("Sum: {sum}");
}
