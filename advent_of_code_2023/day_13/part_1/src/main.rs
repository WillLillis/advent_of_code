use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapItem {
    Ash,  // .
    Rock, // #
}

type MirrorMap = Vec<Vec<MapItem>>;

impl Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapItem::Ash => {
                write!(f, ".")?;
            }
            MapItem::Rock => {
                write!(f, "#")?;
            }
        }

        Ok(())
    }
}

#[allow(dead_code)]
fn print_map(map: &MirrorMap) {
    for row in map.iter() {
        for item in row.iter() {
            print!("{}", item);
        }
        println!();
    }
}

fn get_maps(input: &str) -> Vec<MirrorMap> {
    let mut maps: Vec<MirrorMap> = Vec::new();
    let mut curr_map = MirrorMap::new();

    for line in input.lines() {
        if line.is_empty() {
            maps.push(curr_map.clone());
            curr_map.clear();
        } else {
            curr_map.push(
                line.trim()
                    .chars()
                    .map(|c| match c {
                        '.' => MapItem::Ash,
                        '#' => MapItem::Rock,
                        _ => {
                            panic!("Invalid input character");
                        }
                    })
                    .collect::<Vec<MapItem>>(),
            );
        }
    }

    if !curr_map.is_empty() {
        maps.push(curr_map.clone());
        curr_map.clear();
    }

    maps
}

fn get_vertical_line(map: &MirrorMap) -> Option<usize> {
    let mut is_sym;

    // try each center line
    for left_col in 0..map[0].len() - 1 {
        is_sym = true;
        // check each row
        'row_loop: for row in 0..map.len() {
            for (left, right) in (0..=left_col).rev().zip(left_col + 1..map[row].len()) {
                if map[row][left] != map[row][right] {
                    is_sym = false;
                    break 'row_loop;
                }
            }
        }
        if is_sym {
            return Some(left_col + 1);
        }
    }

    None
}

fn get_horizontal_line(map: &MirrorMap) -> Option<usize> {
    let mut is_sym;

    // try each center line
    for upper_row in 0..map.len() - 1 {
        is_sym = true;
        // check each column
        'col_loop: for col in 0..map[0].len() {
            for (top, bot) in (0..=upper_row).rev().zip(upper_row + 1..map.len()) {
                if map[top][col] != map[bot][col] {
                    is_sym = false;
                    break 'col_loop;
                }
            }
        }
        if is_sym {
            return Some(upper_row + 1);
        }
    }

    None
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read the input file");
    let maps = get_maps(&input);

    let sum: usize = maps
        .iter()
        .map(|map| {
            if let Some(val) = get_vertical_line(map) {
                val
            } else if let Some(val) = get_horizontal_line(map) {
                val * 100
            } else {
                panic!("Didn't find a reflection line!");
            }
        })
        .sum();

    println!("Sum: {}", sum);
}
