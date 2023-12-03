static ALL_SYMBOLS: &[char] = &['.', '*', '+', '#', '$', '%', '@', '/', '=', '&', '-'];
static SYMBOLS: &[char] = &['*', '+', '#', '$', '%', '@', '/', '=', '&', '-'];

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

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read input file");

    let schematic = get_engine_schematic(&input);
    let mut part_nums: u32 = 0;

    for (i, row) in schematic.iter().enumerate() {
        let mut j = 0usize;
        while j < row.len() {
            let curr_char = row.chars().nth(j).unwrap();
            if ALL_SYMBOLS.contains(&curr_char) {
                j += 1;
            } else {
                let num = row
                    .chars()
                    .skip(j)
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>();
                if is_part_num(&schematic, i, j, j + num.len() - 1) {
                    part_nums += num.parse::<u32>().unwrap();
                }
                j += num.len();
            }
        }
    }

    println!("Parts sum: {}", part_nums);
}
