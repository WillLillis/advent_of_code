use std::{fs, cmp};

enum PushDir {
    Left,
    Right
}

impl PushDir {
    fn new(input: char) -> Self {
        match input {
            '<' => PushDir::Left,
            '>' => PushDir::Right,
            _ => {
                panic!("Invalid push character!");
            }
        }
    }
}

fn get_rocks() -> Vec<Vec<Vec<char>>> {
    let mut rocks: Vec<Vec<Vec<char>>> = Vec::new();

    {
        /*
         * ..@@@@.
         */
        let rock_1 = vec![vec!['.', '.', '@', '@', '@', '@', '.']];
        rocks.push(rock_1);
    }
    {
        /*
         * ...@...
         * ..@@@..
         * ...@...
         */
        let mut rock_2 = vec![vec!['.'; 7]; 3];
        rock_2[0][3] = '@';
        rock_2[1][2] = '@';
        rock_2[1][3] = '@';
        rock_2[1][4] = '@';
        rock_2[2][3] = '@';
        rocks.push(rock_2);
    }
    {
        /*
         * ....#..
         * ....#..
         * ..###..
         */
        let mut rock_3 = vec![vec!['.'; 7]; 3];
        rock_3[0][4] = '@';
        rock_3[1][4] = '@';
        rock_3[2][2] = '@';
        rock_3[2][3] = '@';
        rock_3[2][4] = '@';
        rocks.push(rock_3);
    }
    {
        /*
         * ..#....
         * ..#....
         * ..#....
         * ..#....
         */
        let mut rock_4 = vec![vec!['.'; 7]; 4];
        rock_4[0][2] = '@';
        rock_4[1][2] = '@';
        rock_4[2][2] = '@';
        rock_4[3][2] = '@';
        rocks.push(rock_4);
    }
    {
        /*
         * ..##...
         * ..##...
         */
        let mut rock_5 = vec![vec!['.'; 7]; 2];
        rock_5[0][2] = '@';
        rock_5[0][3] = '@';
        rock_5[1][2] = '@';
        rock_5[1][3] = '@';
        rocks.push(rock_5);
    }
    
    rocks
}

// I'm sure there's a more efficient way to do this, but this way of 
// propagating the rock through the chamber should be very debuggable, 
// allow for the addition of other shapes, and maybe allow for a cool
// visualization
fn sim_fall<T, U>(chamber: &mut Vec<Vec<char>>, jets: &mut T, rocks: &mut U, max_height: usize) -> (usize, String)
where
    T: Iterator<Item = PushDir>,
    U: Iterator<Item = Vec<Vec<char>>>
{
    let rock = rocks.next().unwrap();
    let mut curr_height = max_height + 4;
    let rock_height = rock.len();
    let mut fall_str = String::new();

    // place rock into chamber
    for (i, row) in rock.into_iter().rev().enumerate() {
        chamber[max_height + i + 4] = row;
    }

    loop {
        let push = jets.next().unwrap();
        let push_dir = match push {
            PushDir::Left => -1i32,
            PushDir::Right => 1i32
        };

        let mut in_bounds = true;
        'outer: for i in curr_height..curr_height + rock_height {
            for (j, char) in chamber[i].iter().enumerate() {
                match char {
                    '.'|'#' => {},
                    '@' => {
                        if (push_dir + (j as i32)) < 0 || (push_dir + (j as i32)) >= 7 {
                            in_bounds = false;
                            break 'outer;
                        }
                        if chamber[i][(j as i32 + push_dir) as usize] == '#' {
                            in_bounds = false;
                            break 'outer;
                        } 
                    },
                    _ => {
                        panic!("Invalid rock character!");
                    }
                }
            }
        }
        if in_bounds {
            // do the push
            let fall_char = match push {
                PushDir::Left => '<',
                PushDir::Right => '>'
            };
            fall_str.push(fall_char);
            for i in curr_height..curr_height + rock_height {
                if push_dir == -1 {
                    for (j, item) in chamber[i].clone().iter().enumerate() {
                        match item {
                            '.'|'#' => {},
                            '@' => {
                                chamber[i][(j as i32 + push_dir) as usize] = '@';
                            },
                            _ => {
                                panic!("Invalid rock character!");
                            }
                        }
                    }
                } else if push_dir == 1 {
                    for (j, item) in chamber[i].clone().iter().enumerate().rev() {
                        match item {
                            '.'|'#' => {},
                            '@' => {
                                chamber[i][(j as i32 + push_dir) as usize] = '@';
                            },
                            _ => {
                                panic!("Invalid rock character!");
                            }
                        }
                    }

                }
                // push rock pieces over 1
                // erase the old tail
                if push_dir == -1 {
                    for (j, item) in chamber[i].clone().iter().enumerate().rev() {
                        match item {
                            '.'|'#' => {},
                            '@' => {
                                chamber[i][j] = '.';
                                break;
                            },
                            _ => {
                                panic!("Invalid rock character!");
                            }
                        } 
                    }
                } else if push_dir == 1 { 
                    for (j, item) in chamber[i].clone().iter().enumerate() {
                        match item {
                            '.'|'#' => {},
                            '@' => {
                                chamber[i][j] = '.';
                                break;
                            },
                            _ => {
                                panic!("Invalid rock character!");
                            }
                        } 
                    }
                }
            }
        } else {
            fall_str.push('X'); // 'X' to indicate push didn't occur
        }

        // and then the fall
            // validate first
        in_bounds = true;
        'outer: for i in curr_height..curr_height + rock_height {    
             for (j, item) in chamber[i].clone().iter().enumerate() {
                match item {
                    '.'|'#' => {},
                    '@' => {
                       if chamber[i - 1][j] == '#' {
                            in_bounds = false;
                            break 'outer;
                       }
                    },
                    _ => {
                        panic!("Invalid falling rock character");
                    }

                }
            }
        }
        if !in_bounds {
            for i in curr_height..curr_height + rock_height {
                for (j, item) in chamber[i].clone().iter().enumerate() {
                    match item {
                        '.'|'#' => {},
                        '@' => {
                           chamber[i][j] = '#';
                        },
                        _ => {
                            panic!("Invalid falling rock character");
                        }

                    }
                }
            }
            
            fall_str.push('E'); // 'E' to indicate End of a fall
            return (cmp::max(max_height, curr_height + rock_height - 1), fall_str);
        }
        // then move the blocks down
        fall_str.push('v');
        for i in curr_height..curr_height + rock_height {
            for (j, item) in chamber[i].clone().iter().enumerate() {
                match item {
                    '.'|'#' => {},
                    '@' => {
                        chamber[i - 1][j] = '@'; 
                    },
                    _ => {
                        panic!("Invalid rock character!");
                    }
                }
            }
            // erase the old row
            for (j, item) in chamber[i].clone().iter().enumerate() {
                match item {
                    '.'|'#' => {},
                    '@' => {
                        chamber[i][j] = '.';
                    },
                    _ => {
                        panic!("Invalid rock character!");
                    }
                } 
            }
        }


        curr_height -= 1;
    }
}

fn main() {
    const NUM_ROCKS: usize = 1_000_000_000_000usize;
    let input = fs::read_to_string("input.txt").unwrap();
    let mut jets = input.trim().chars().map(|c| PushDir::new(c)).cycle();

    let mut rocks = get_rocks().into_iter().cycle();
    let rock_len = get_rocks().into_iter().count();
    let mut fall_str;
   
    let mut chamber: Vec<Vec<char>> = vec![vec!['.'; 7]; 5000];
    chamber[0] = vec!['#'; 7];
    
    // implement cycle detection by matching "fall lines" of every full set (5) of rocks
        // https://www.reddit.com/r/adventofcode/comments/znykq2/2022_day_17_solutions/jl9j24u/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    // variables to track the current sim
    let mut fall_cache = String::new();
    let mut height_cache = vec![0];
    let mut latest = String::new();
    let mut latest_count = 0;
    let mut height = 0;
    // stuff we want to track after breaking out of the first sim loop
    let mut cycle_height = 0;
    let mut cycle_len = 0;
    let mut end_rock_num = 0;

    'rock_loop: for i in 0..NUM_ROCKS {
        (height, fall_str) = sim_fall(&mut chamber, &mut jets, &mut rocks, height);
        latest.push_str(&fall_str);
        latest_count += 1;
        height_cache.push(height);

        if latest_count == rock_len {
            // if we've already seen this 'latest' sequence, we've found a cycle!
            if fall_cache.contains(&latest) {
                let parts: Vec<&str> = fall_cache.split(&latest).collect(); // before and after the
                                                                            // cycle's start
                assert!(parts.len() == 2); // sanity check
                let start_rock_num = parts[0].chars().fold(0, |count, c| {
                    count + if c == 'E' {1} else {0}
                })             // marks the start of the rock cycle...
                   + rock_len; // ...we need the end so we don't double count
                end_rock_num = i;
                cycle_height = height - height_cache[start_rock_num];
                cycle_len = parts[1].chars().fold(0, |count, c| {
                    count + if c == 'E' {1} else {0}
                }) + rock_len;
                break 'rock_loop;        
            } else {
                fall_cache.push_str(&latest);
                latest.clear();
                latest_count = 0;
            }
        }
    }

    let num_cycles = (NUM_ROCKS - end_rock_num) / cycle_len;
    let remaining = NUM_ROCKS - end_rock_num - 1 - (num_cycles * cycle_len);
    
    for _ in 0..remaining {
        (height, _) = sim_fall(&mut chamber, &mut jets, &mut rocks, height); 
    }
    // add the total due to the repeated cycles
    height += num_cycles * cycle_height;
    println!("Tower height after {NUM_ROCKS} rocks: {height}");
}
