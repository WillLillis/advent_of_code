use std::fs;

fn get_height_map(file_name: &str) -> Vec<Vec<u32>> {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file");

    let mut map: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        map.push(line.trim().chars().filter_map(|c| c.to_digit(10)).collect());
    }

    return map;
}

fn is_lowest(row: usize, col: usize, map: &Vec<Vec<u32>>) -> bool {
    let val = map[row][col];

    // check up if we can
    if row != 0 && map[row - 1][col] <= val {
        return false;
    }
    // check down if we can
    if (row < map.len() - 1) && map[row + 1][col] <= val {
        return false;
    }
    // check left if we can
    if col != 0 && map[row][col - 1] <= val {
        return false;
    }
    // check right
    if (col < map[0].len() - 1) && map[row][col + 1] <= val {
        return false;
    }

    return true;
}

fn main() {
    let map = get_height_map("input.txt");

    let mut sum = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            sum += if is_lowest(i, j, &map) {
                1 + map[i][j]
            } else {
                0
            };
        }
    }

    println!("Sum of risk levels of low points: {sum}");
}
