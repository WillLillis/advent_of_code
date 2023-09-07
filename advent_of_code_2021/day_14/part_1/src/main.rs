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

fn pair_insertion(polymer: &mut Vec<char>, mappings: &HashMap<(char, char), char>) {
    let mut insertions: Vec<(usize, char)> = Vec::new();

    // gather all the insertions to do
    for (i, pair) in polymer.windows(2).enumerate() {
        match mappings.get(&(pair[0], pair[1])) {
            Some(&c) => {
                insertions.push((i + 1, c));
            },
            None => {}
        }
    }
    
    // do the insertions
    loop { 
        let (idx, c) = match insertions.pop() {
            Some((a, b)) => (a, b),
            None => { break; }
        };
        polymer.insert(idx, c);
        for (_idx, _) in insertions.iter_mut() {
            if *_idx > idx {
                *_idx += 1;
            }
        }
    }

}


fn main() {
    let (mut start, mappings) = match get_polymer_info("input.txt") {
        Ok((a, b)) => (a, b),
        Err(a) => {
            println!("uh oh {:?}", a);
            panic!("Failed to read the input file!");
        }
    };

    let n_steps = 10;
    for _ in 0..n_steps {
        pair_insertion(&mut start, &mappings);
    }

    let mut counts: HashMap<char, u32> = HashMap::new();

    for c in start.iter() {
        counts.entry(*c).and_modify(|count| *count += 1).or_insert(1);
    }

    let min = counts.values().min().unwrap();
    let max = counts.values().max().unwrap();

    println!("Quantity of interest: {}", *max - *min);
}
