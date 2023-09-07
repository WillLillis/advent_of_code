use std::collections::HashMap;
use anyhow::Result;
use std::fs;

fn get_polymer_info(file_name: &str) -> Result<(Vec<char>, HashMap<(char, char), char>)> {
    let input = fs::read_to_string(file_name)?;

    let start: Vec<char> = input.chars().take_while(|&c| c != '\n').collect();
    let mut mappings: HashMap<(char, char), char> = HashMap::new();

    for line in input.lines().skip(2) {
        let parts: Vec<&str> = line.split(&[' ', '-', '>']).filter(|s| s.len() > 0).collect();
        assert!(parts.len() == 2);
        assert!(parts[0].len() == 2);
        assert!(parts[1].len() == 1);
        let first = parts[0].as_bytes()[0] as char;
        let second = parts[0].as_bytes()[1] as char;
        let insert = parts[1].as_bytes()[0] as char;
        mappings.insert((first, second), insert);
    }

    return Ok((start, mappings));
}

fn update_counts(counts: &mut HashMap<(char, char), i64>, mappings: &HashMap<(char, char), char>) {
    let mut delta_counts: HashMap<(char, char), i64> = HashMap::new();
    for (&(c1, c2), old_count) in counts.iter().filter(|(_, count)| **count > 0) {
        match mappings.get(&(c1, c2)) {
            Some(&c) => {
                delta_counts.entry((c1, c2)).and_modify(|count| *count -= old_count).or_insert(-old_count);
                delta_counts.entry((c1, c)).and_modify(|count| *count += old_count).or_insert(*old_count);
                delta_counts.entry((c, c2)).and_modify(|count| *count += old_count).or_insert(*old_count);
            },
            None => {}
        }
    }

    for (&(c1, c2), delta) in delta_counts.iter() {
        counts.entry((c1, c2)).and_modify(|count| *count += delta).or_insert(*delta);
    }
}

fn main() {
    let (start, mappings) = match get_polymer_info("input.txt") {
        Ok((a, b)) => (a, b),
        Err(a) => {
            println!("uh oh {:?}", a);
            panic!("Failed to read the input file!");
        }
    };

    let mut pair_counts: HashMap<(char, char), i64> = HashMap::new();

    for pair in start.windows(2) {
        pair_counts.entry((pair[0], pair[1])).and_modify(|count| *count += 1).or_insert(1);
    }
    
    let n_steps = 40;
    for _ in 0..n_steps {
        update_counts(&mut pair_counts, &mappings);
    }

    let mut counts: HashMap<char, i64> = HashMap::new();

    for (&(c1, c2), pair_count) in pair_counts.iter() {
        counts.entry(c1).and_modify(|count| *count += pair_count).or_insert(*pair_count);
        counts.entry(c2).and_modify(|count| *count += pair_count).or_insert(*pair_count);
    }

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();
    
    // divide by 2 and round to the nearest integer to eliminate double counts
    let adjusted = f64::round((*max as f64 - *min as f64) / 2.0) as i64;

    println!("Quantity of interest: {}", adjusted);
}
