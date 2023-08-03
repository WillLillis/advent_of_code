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

        //for (j, row) in chamber.iter().enumerate().rev().skip(26250) {
        //    println!("{:5}: {:?}", j, row);
        //}

        // and then the fall
        //
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


        //for (j, row) in chamber.iter().enumerate().rev().skip(5000 - 60) {
        //    println!("{:5}: {:?}", j, row);
        //}
        curr_height -= 1;
    }
}

fn main() {
    let input = fs::read_to_string("test_input.txt").unwrap();
    let mut jets = input.trim().chars().map(|c| PushDir::new(c)).cycle();

    let mut rocks = get_rocks().into_iter().cycle();
    let rock_len = get_rocks().into_iter().count();
    let mut fall_str;
   
    let mut chamber: Vec<Vec<char>> = vec![vec!['.'; 7]; 5000];
    chamber[0] = vec!['#'; 7];
    
    // implement cycle detection by matching "fall lines" of every full set (5) of rocks
        // https://www.reddit.com/r/adventofcode/comments/znykq2/2022_day_17_solutions/jl9j24u/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
    let mut cache: Vec<String> = Vec::new();
    let mut height = 0;
    'rock_loop: for i in 0..2022 {
        (height, fall_str) = sim_fall(&mut chamber, &mut jets, &mut rocks, height);
        cache.push(fall_str);

        if i % rock_len == 0 && i > 5 {
            // check for cycles, checking rock_len fall strings at a time
            let mut cycle_idx = 0;
            let mut old_falls = cache.iter().take(cache.len() - rock_len);

            while cycle_idx < cache.iter().count() - rock_len {
                println!("Another round of comparisons...(cycle_idx = {cycle_idx})");
                if cache.iter().rev().take(rock_len).rev().all(|f| { 
                                                               println!("new_fall = {f}");
                                                               let old_fall = old_falls.next().unwrap();
                                                               println!("\told_fall = {old_fall}");
                                                               f == old_fall
                }) {
                    println!("Found a cycle!!!! {cycle_idx} -> {i}");
                    break 'rock_loop;
                }
                cycle_idx += rock_len;
            }
        }

    }

    //for (i, row) in chamber.iter().enumerate().rev().skip(5000 - 60){
    //    println!("{:5}: {:?}", i, row);
    //}
    


    for (i, fall) in cache.iter().enumerate() {
        println!("{i:3}: {fall}");
    }

    println!("Tower height after 2022 rocks: {height}");
}
