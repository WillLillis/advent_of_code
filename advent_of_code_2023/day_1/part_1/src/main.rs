fn get_calib(line: &str) -> Option<u32> {
    let first_digit = line.chars().find(|c| c.is_digit(10));
    let last_digit = line.chars().rev().find(|c| c.is_digit(10));

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => {
            Some(10u32 * first.to_digit(10).unwrap() + last.to_digit(10).unwrap())
        }
        _ => None,
    }
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read input file");

    let sum: u32 = input.lines().map(|line| get_calib(line).unwrap()).sum();

    println!("Calibration sum: {sum}");
}
