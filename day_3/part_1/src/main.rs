use std::fs;
use std::collections::HashSet;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");
    let input = input.lines();

    let mut items: HashSet<char> = HashSet::new();
    let mut total_priority: u32 = 0;
    let mut cap;
    let mut dup: char;

    for line in input {
        //println!("Line: {line}");
        items.clear();
        cap = line.len() / 2; // just going to assume even length...

        for item in line.trim().chars().take(cap) {
            items.insert(item);
        }

        dup = line.trim().chars().skip(cap).skip_while(
            |x| !items.contains(x)).next().expect(
                "End of 2nd compartment reached unexpectedly!");
        
        //println!("\tDuplicatd char: {}", dup);
        total_priority += if dup.is_ascii_lowercase() {
            1 + dup as u32 - 'a' as u32
        } else if dup.is_ascii_uppercase() {
            27 + dup as u32 - 'A' as u32
        } else {
            panic!("Invalid character!");
        };
    }

    println!("Total total priority: {}", total_priority);
}
