use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");

    let mut sig: HashMap<char, u32> = HashMap::new();
    let mut trailing_iter = input.chars();
    let mut start_idx = 0;

    // start off with initial state
    for x in input.chars().take(14) {
        sig.entry(x).and_modify(|count| *count += 1).or_insert(1);
    }



    // now we go with the sliding window...
    for (idx, x) in input.chars().enumerate().skip(14) {
        // check if we have 4 unique chars
        if sig.len() == 14 {
            println!("{:?}", sig);
            start_idx = idx;
            break;
        }

        // otherwise we decrement the count of/ remove the oldest char
        let oldest_char = trailing_iter.next().expect("Ran out of chars early!");
        if sig.get(&oldest_char).unwrap() == &1 {
            sig.remove(&oldest_char);
        } else {
            sig.entry(oldest_char).and_modify(|count| *count -= 1);
        }

        // and add the newest one
        sig.entry(x).and_modify(|count| *count += 1).or_insert(1);
    }
    
    println!("Starting index: {start_idx}");
}
