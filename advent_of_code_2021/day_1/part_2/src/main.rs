use std::fs;

fn get_sonar_reading(file_name: &str) -> Vec<u32> {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file!");

    let mut depths: Vec<u32> = Vec::new();

    for line in input.lines() {
        depths.push(line.parse::<u32>().unwrap());
    }
    
    return depths;
}

fn main() {
    let depths = get_sonar_reading("input.txt");

    let mut count = 0;

    let mut iter = depths.windows(3);

    let mut last: u32 = iter
        .next()
        .unwrap()
        .iter()
        .sum();

    while let Some(next_win) = iter.next() {
        let next = next_win
            .iter()
            .sum();

        if next > last {
            count += 1;
        }

        last = next;
    }

    println!("The final count is: {count}");
}
