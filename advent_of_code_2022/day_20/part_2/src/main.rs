use std::fs;

fn get_nums(file_name: &str) -> Vec<i64> {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file.");

    let mut nums: Vec<i64> = Vec::new();
    
    for line in input.lines() {
        nums.push(line.trim().parse::<i64>().unwrap());
    }
    
    return nums;
}


fn main() {
    const SECRET_KEY: i64 = 811_589_153;
    let nums: Vec<i64> = get_nums("input.txt").iter().map(|x| x * SECRET_KEY).collect();

    // - tried several different approaches here and struggled, code always worked on the test input 
    // but gave the wrong answer on the full input
    // - Eventually gave up and tried looking online for help, found this:
    // https://nickymeuleman.netlify.app/garden/aoc2022-day20https://nickymeuleman.netlify.app/garden/aoc2022-day20
    let mut mixed: Vec<usize> = (0..nums.len()).collect();
    for _ in 0..10 {
        for (i, num) in nums.iter().enumerate() {
            // find the index of the current number in the shuffled list
            let mixed_idx = mixed.iter().position(|num| *num == i).unwrap();
            mixed.remove(mixed_idx); // and remove it
            let new_idx = (mixed_idx as i64 + num).rem_euclid(mixed.len() as i64) as usize;
            mixed.insert(new_idx, i); // and re-insert it
        }
    }

    let zero_idx = nums // find where 0 is in the original list
        .iter()
        .position(|&num| num == 0)
        .unwrap();

    let zero_mixed_idx = mixed // and then the index in the shuffled list
        .iter()
        .position(|&mix_num| mix_num == zero_idx)
        .unwrap();

    
    let total: i64 = [1000, 2000, 3000]
        .iter()
        .map(|offset| {
            let mixed_idx = (zero_mixed_idx + offset) % mixed.len(); // index 1000, 2000, 3000 up from 0's index...
            let nums_idx = mixed[mixed_idx]; // and then translate that index over to the nums array 
            nums[nums_idx] // and finally get the number stored at that index
        })
        .sum();

    println!("Total: {:?}", total);
}
