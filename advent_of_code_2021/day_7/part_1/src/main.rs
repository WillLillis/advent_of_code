use std::fs;

fn get_crabs(file_name: &str) -> Vec<u32> {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file.");

    return input
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<u32>>();
}

fn main() {
    let crabs = get_crabs("input.txt");

    let mut min_fuel = u32::MAX;
    let mut best_pos = u32::MAX;

    let min_crab = *crabs.iter().min().unwrap();
    let max_crab = *crabs.iter().max().unwrap();

    for mid_pos in min_crab..=max_crab {
        let curr_fuel = crabs.iter().fold(0, |accum, &crab_pos| {
            accum + u32::abs_diff(mid_pos, crab_pos)
        });
        if curr_fuel < min_fuel {
            best_pos = mid_pos;
            min_fuel = curr_fuel;
        }
    }

    println!("Best position: {best_pos}, fuel cost: {min_fuel}");
}
