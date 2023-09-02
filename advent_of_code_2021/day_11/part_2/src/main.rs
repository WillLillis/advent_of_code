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

fn energize_neighbors(row: usize, col: usize, octopi: &mut Vec<Vec<Option<u8>>>) {
    // Energize up if we can
    if row != 0 {
        match octopi[row - 1][col] {
            Some(energy) => {
                octopi[row - 1][col] = Some(energy + 1);
            }
            None => {}
        }
    }
    // Diagonal up and left
    if row != 0 && col != 0 {
        match octopi[row - 1][col - 1] {
            Some(energy) => {
                octopi[row - 1][col - 1] = Some(energy + 1);
            }
            None => {}
        }
    }

    // Diagonal up and right
    if row != 0 && col < octopi[0].len() - 1 {
        match octopi[row - 1][col + 1] {
            Some(energy) => {
                octopi[row - 1][col + 1] = Some(energy + 1);
            }
            None => {}
        }
    }

    // energize down if we can
    if row < octopi.len() - 1 {
        match octopi[row + 1][col] {
            Some(energy) => {
                octopi[row + 1][col] = Some(energy + 1);
            }
            None => {}
        }
    }
    // Diagonal down and left
    if row < octopi.len() - 1 && col != 0 {
        match octopi[row + 1][col - 1] {
            Some(energy) => {
                octopi[row + 1][col - 1] = Some(energy + 1);
            }
            None => {}
        }
    }

    // Diagonal down and right
    if row < octopi.len() - 1 && col < octopi[0].len() - 1 {
        match octopi[row + 1][col + 1] {
            Some(energy) => {
                octopi[row + 1][col + 1] = Some(energy + 1);
            }
            None => {}
        }
    }

    // Energize left if we can
    if col != 0 {
        match octopi[row][col - 1] {
            Some(energy) => {
                octopi[row][col - 1] = Some(energy + 1);
            }
            None => {}
        }
    }
    // Energize right if we can
    if col < octopi[0].len() - 1 {
        match octopi[row][col + 1] {
            Some(energy) => {
                octopi[row][col + 1] = Some(energy + 1);
            }
            None => {}
        }
    }
}

fn sim_step(octopi: &mut Vec<Vec<Option<u8>>>) -> u32 {
    // First, the energy level of each octopus increases by 1
    octopi.iter_mut().flatten().for_each(|x| {
        let tmp_x = *x;
        *x = match tmp_x {
            Some(num) => Some(num + 1),
            None => Some(1), // octopi that flashed last round have energy 0, marked as None,
                             // increase it to 1
        };
    });

    let mut n_flashes = 0u32;
    let mut flashed = true;
    while flashed {
        flashed = false;
        for row in 0..octopi.len() {
            for col in 0..octopi[0].len() {
                match octopi[row][col] {
                    Some(energy) => {
                        if energy > 9 {
                            flashed = true;
                            n_flashes += 1;
                            octopi[row][col] = None;
                            energize_neighbors(row, col, octopi);
                        }
                    }
                    None => {}
                }
            }
        }
    }

    return n_flashes;
}

fn main() {
    let mut octopi = get_octopi("input.txt");

    let mut n_steps = 0;

    loop {
        n_steps += 1;
        if sim_step(&mut octopi) == (octopi.len() * octopi[0].len()) as u32 {
            break;
        }
    }

    println!("The octopi synchronized after {n_steps} steps.");
}
