use std::{fs, cmp};

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
fn sim_fall<T, U>(chamber: &mut Vec<Vec<char>>, jets: &mut T, rocks: &mut U, max_height: usize) -> usize
where
    T: Iterator<Item = char>,
    U: Iterator<Item = Vec<Vec<char>>>
{
    let rock = rocks.next().unwrap();
    let mut curr_height = max_height + 4;
    let rock_height = rock.len();

    // place rock into chamber
    for (i, row) in rock.into_iter().rev().enumerate() {
        chamber[max_height + i + 4] = row;
    }

    loop {
        let push = jets.next().unwrap();
        let push_dir = match push {
            '<' => -1i32,
            '>' => 1i32,
            _ => {
                panic!("Invalid jet character!");
            }
        };

        //for (j, row) in chamber.iter().enumerate().rev().skip(5000 - 60) {
        //    println!("{:5}: {:?}", j, row);
        //}
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

            return cmp::max(max_height, curr_height + rock_height - 1);
        }
        // then move the blocks down
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
    let input = fs::read_to_string("input.txt").unwrap();
    let mut jets = input.trim().chars().cycle();

    let mut rocks = get_rocks().into_iter().cycle();
   
    let mut chamber: Vec<Vec<char>> = vec![vec!['.'; 7]; 5000];
    chamber[0] = vec!['#'; 7];

    let mut height = 0;
    for _ in 0..2022 {
        height = sim_fall(&mut chamber, &mut jets, &mut rocks, height);
        println!("Height: {height}");
    }

    //for (i, row) in chamber.iter().enumerate().rev().skip(5000 - 60){
    //    println!("{:5}: {:?}", i, row);
    //}

    println!("Tower height after 2022 rocks: {height}");
}
