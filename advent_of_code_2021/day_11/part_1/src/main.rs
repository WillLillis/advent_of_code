use std::fs;

fn get_octopi(file_name: &str) -> Vec<Vec<Option<u8>>> {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file");

    let mut octopi: Vec<Vec<Option<u8>>> = Vec::new();

    for line in input.lines() {
        octopi.push(
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|x| Some(x as u8))
                .collect(),
        );
    }

    return octopi;
}

// todo: Finish this
fn energize_neighbors(row: usize, col: usize, octopi: &mut Vec<Vec<Option<u8>>>) {
    // energize up if we can
    if row != 0 {
        match octopi[row - 1][col] {
            Some
        }
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
}

fn sim_step(octopi: &mut Vec<Vec<Option<u8>>>) {
    // First, the energy level of each octopus increases by 1
    octopi.iter_mut().flatten().for_each(|x| {
        let tmp_x = *x;
        *x = match tmp_x {
            Some(num) => Some(num + 1),
            None => Some(1),
        };
    });

    // 
}

fn main() {
    let mut octopi = get_octopi("test_input.txt");

    for row in octopi.iter() {
        println!("{:?}", row);
    }
}
