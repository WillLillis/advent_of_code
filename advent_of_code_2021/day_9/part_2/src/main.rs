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

fn get_basin_size(row: usize, col: usize, map: &Vec<Vec<u32>>) -> u32 {
    
    0
}

fn main() {
    let map = get_height_map("test_input.txt");
    // find basins
    // find corresponding basin sizes
    // only keep the top three
    //let mut basins: Vec<(usize, usize)> = Vec::new();
    let mut basin_sizes: [u32; 4] = [0; 4];

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if is_lowest(i, j, &map) {
                basin_sizes[3] = get_basin_size(i, j, &map);
                basin_sizes.sort();
            }
        }
    }

    let sizes: u32 = basin_sizes.iter().take(3).product();

    println!("Sizes: {sizes}");
}
