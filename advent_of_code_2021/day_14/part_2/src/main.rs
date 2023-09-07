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

fn update_counts(counts: &mut HashMap<(char, char), i32>, mappings: &HashMap<(char, char), char>) {
    let mut delta_counts: HashMap<(char, char), i32> = HashMap::new();
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

// actual string manipulation approach is too slow!
// TODO: Count pairs-> each insertion subtracts 1 from the count of the matched pair
// and increases the count of the the two pairs (1,2) and (2,3)
fn main() {
    let (start, mappings) = match get_polymer_info("test_input.txt") {
        Ok((a, b)) => (a, b),
        Err(a) => {
            println!("uh oh {:?}", a);
            panic!("Failed to read the input file!");
        }
    };

    let mut pair_counts: HashMap<(char, char), i32> = HashMap::new();

    for pair in start.windows(2) {
        pair_counts.entry((pair[0], pair[1])).and_modify(|count| *count += 1).or_insert(1);
    }
    
    println!("Initial pair counts:\n {:#?}", pair_counts);

    let n_steps = 10;
    for i in 0..n_steps {
        update_counts(&mut pair_counts, &mappings);
        println!("\n\nPair counts after {} steps: {:#?}", i + 1, pair_counts);
    }

    println!("{:#?}", pair_counts);

    let mut counts: HashMap<char, i32> = HashMap::new();

    for (&(c1, c2), pair_count) in pair_counts.iter() {
        counts.entry(c1).and_modify(|count| *count += pair_count).or_insert(*pair_count);
        counts.entry(c2).and_modify(|count| *count += pair_count).or_insert(*pair_count);
    }

    println!("{:#?}", counts);

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    println!("Quantity of interest: {}", *max - *min);
}
