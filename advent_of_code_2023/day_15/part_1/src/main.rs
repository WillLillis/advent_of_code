fn hash(init_seq: &Vec<char>) -> Option<u8> {
    let mut curr_val = 0u128;

    for c in init_seq.iter() {
        curr_val += (*c as u8) as u128;
        curr_val *= 17;
        curr_val %= 256;
    }

    if curr_val <= 255 {
        Some(curr_val as u8)
    } else {
        None
    }
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read the input file.");
    let init_seq: Vec<Vec<char>> = input
        .trim()
        .split(',')
        .map(|s| s.chars().collect())
        .collect();

    let hash_sum: u128 = init_seq.iter().map(|x| hash(x).unwrap() as u128).sum();

    println!("Hash: {}", hash_sum);
}
