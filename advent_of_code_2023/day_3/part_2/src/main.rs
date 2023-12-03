use std::collections::HashMap;

static SYMBOLS: &[char] = &['*', '+', '#', '$', '%', '@', '/', '=', '&', '-'];

struct Part {
    val: u64,
    id: u64,
}

type PartTracker = HashMap<(usize, usize), Part>;

impl Part {
    fn new(val: u64, id: u64) -> Self {
        Part { val, id }
    }
}

fn get_engine_schematic(input: &str) -> Vec<String> {
    let mut schematic = Vec::new();
    for line in input.lines() {
        schematic.push(line.trim().to_string());
    }

    schematic
}

fn is_part_num(schematic: &Vec<String>, row: usize, start_idx: usize, end_idx: usize) -> bool {
    let left_idx = if start_idx == 0 { 0 } else { start_idx - 1 };
    let right_idx = if end_idx == schematic[row].len() - 1 {
        end_idx
    } else {
        end_idx + 1
    };
    // check above
    if row > 0 {
        if schematic[row - 1][left_idx..=right_idx].contains(SYMBOLS) {
            return true;
        }
    }
    // check left and right
    if schematic[row][left_idx..=right_idx].contains(SYMBOLS) {
        return true;
    }
    // check below
    if row != schematic.len() - 1 {
        if schematic[row + 1][left_idx..=right_idx].contains(SYMBOLS) {
            return true;
        }
    }

    false
}

fn get_gear_ratio(parts: &PartTracker, row: usize, idx: usize) -> Option<u64> {
    let mut first_num = None;
    let mut second_num = None;

    // up and left
    if let Some(num) = parts.get(&(row - 1, idx - 1)) {
        first_num = Some(num);
    }
    // up
    if let Some(num) = parts.get(&(row - 1, idx)) {
        match first_num {
            Some(first) if first.id != num.id => {
                second_num = Some(num);
            }
            None => {
                first_num = Some(num);
            }
            _ => {}
        }
    }
    // up and right
    if let Some(num) = parts.get(&(row - 1, idx + 1)) {
        match (first_num, second_num) {
            // too many adjacent part numbers
            (Some(first), Some(second)) if first.id != num.id && second.id != num.id => {
                return None;
            }
            // second adjacent part number
            (Some(first), None) if first.id != num.id => {
                second_num = Some(num);
            }
            // first adjacent part number
            (None, _) => {
                first_num = Some(num);
            }
            // else
            _ => {}
        }
    }
    // left
    if let Some(num) = parts.get(&(row, idx - 1)) {
        match (first_num, second_num) {
            // too many adjacent part numbers
            (Some(first), Some(second)) if first.id != num.id && second.id != num.id => {
                return None;
            }
            // second adjacent part number
            (Some(first), None) if first.id != num.id => {
                second_num = Some(num);
            }
            // first adjacent part number
            (None, _) => {
                first_num = Some(num);
            }
            // else
            _ => {}
        }
    }
    // right
    if let Some(num) = parts.get(&(row, idx + 1)) {
        match (first_num, second_num) {
            // too many adjacent part numbers
            (Some(first), Some(second)) if first.id != num.id && second.id != num.id => {
                return None;
            }
            // second adjacent part number
            (Some(first), None) if first.id != num.id => {
                second_num = Some(num);
            }
            // first adjacent part number
            (None, _) => {
                first_num = Some(num);
            }
            // else
            _ => {}
        }
    }
    // down and left
    if let Some(num) = parts.get(&(row + 1, idx - 1)) {
        match (first_num, second_num) {
            // too many adjacent part numbers
            (Some(first), Some(second)) if first.id != num.id && second.id != num.id => {
                return None;
            }
            // second adjacent part number
            (Some(first), None) if first.id != num.id => {
                second_num = Some(num);
            }
            // first adjacent part number
            (None, _) => {
                first_num = Some(num);
            }
            // else
            _ => {}
        }
    }
    // down
    if let Some(num) = parts.get(&(row + 1, idx)) {
        match (first_num, second_num) {
            // too many adjacent part numbers
            (Some(first), Some(second)) if first.id != num.id && second.id != num.id => {
                return None;
            }
            // second adjacent part number
            (Some(first), None) if first.id != num.id => {
                second_num = Some(num);
            }
            // first adjacent part number
            (None, _) => {
                first_num = Some(num);
            }
            // else
            _ => {}
        }
    }
    // down and right
    if let Some(num) = parts.get(&(row + 1, idx + 1)) {
        match (first_num, second_num) {
            // too many adjacent part numbers
            (Some(first), Some(second)) if first.id != num.id && second.id != num.id => {
                return None;
            }
            // second adjacent part number
            (Some(first), None) if first.id != num.id => {
                second_num = Some(num);
            }
            // first adjacent part number
            (None, _) => {
                first_num = Some(num);
            }
            // else
            _ => {}
        }
    }

    match (first_num, second_num) {
        (Some(first), Some(second)) => {
            return Some(first.val * second.val);
        }
        _ => {
            return None;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read input file");

    let schematic = get_engine_schematic(&input);
    let mut parts: PartTracker = HashMap::new();
    let mut part_id = 0u64;
    let mut gear_ratio = 0u64;

    for (i, row) in schematic.iter().enumerate() {
        let mut j = 0usize;
        while j < row.len() {
            let curr_char = row.chars().nth(j).unwrap();
            if curr_char.is_ascii_digit() {
                let num = row
                    .chars()
                    .skip(j)
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>();
                if is_part_num(&schematic, i, j, j + num.len() - 1) {
                    let num_val = num.parse::<u64>().unwrap();
                    for k in j..=j + num.len() - 1 {
                        parts.insert((i, k), Part::new(num_val, part_id));
                    }
                    part_id += 1;
                }
                j += num.len();
            } else {
                j += 1;
            }
        }
    }

    for (i, row) in schematic.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '*' {
                if let Some(ratio) = get_gear_ratio(&parts, i, j) {
                    gear_ratio += ratio;
                }
            }
        }
    }

    println!("Gear ratio sum: {}", gear_ratio);
}
