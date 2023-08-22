use std::fs;

#[derive(Debug, Clone)]
pub struct TreeVis {
    pub height: u8,
    pub is_visible: bool
}

impl TreeVis {
    pub fn new(height: char, is_visible: bool) -> Self {
        let height_num: u32;
        if ('0'..='9').contains(&height) {
            height_num = height.to_digit(10).unwrap();
        } else {
            panic!("Invalid character passed to new()");
        }

        TreeVis {
            height: height_num as u8,
            is_visible
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");
    
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut n_trees: u32 = 0;
    let mut forest: Vec<Vec<TreeVis>> = vec![Vec::new(); height];

    for (i, line) in input.lines().enumerate() {
        for (j, tree) in line.chars().enumerate() {
            if (i == 0) || (i == height - 1)
                || (j == 0) || (j == width - 1) {
                forest[i].push(TreeVis::new(tree, true));
            } else {
                forest[i].push(TreeVis::new(tree, false));
            }
        }
    }
    // left to right and right to left 
    let mut highest;
    let mut farthest_vis: usize;
    for row in 1..height-1 {
        // left to right
        highest = forest[row].first().unwrap().height;
        farthest_vis = 0;
        for col in 1..width-1 {
            if forest[row][col].height > highest {
                forest[row][col].is_visible = true;
                highest = forest[row][col].height;
                farthest_vis = col;
            }
            if highest == 9 {
                break;
            }
        }

        // right to left
        highest = forest[row].last().unwrap().height;
        for col in (farthest_vis+1..width).rev() {
            if forest[row][col].height > highest {
                forest[row][col].is_visible = true;
                highest = forest[row][col].height;
            }
            if highest == 9 {
                break;
            }
        } 
    }

    // top to bottom and bottom to top
    for col in 1..width-1 {
        // top to bottom
        highest = forest[0][col].height;
        farthest_vis = 0;
        for row in 1..height-1 {
            if forest[row][col].height > highest {
                forest[row][col].is_visible = true;
                highest = forest[row][col].height;
                farthest_vis = row;
            }
            if highest == 9 {
                break;
            }
        }

        // bottom to top
        highest = forest[height - 1][col].height;
        for row in (farthest_vis+1..height).rev() {
            if forest[row][col].height > highest {
                forest[row][col].is_visible = true;
                highest = forest[row][col].height;
            }
            if highest == 9 {
                break;
            }
        }
    }

    // could avoid counting the outer layer...but it shouldn't be that
    // much extra time
    for row in 0..height {
        for col in 0..width {
            if forest[row][col].is_visible {
                n_trees += 1;
            }
        }
    }

    //for col in 0..width {
    //    println!("{:?}", forest[1][col]);
    //}

    println!("Visible Trees: {n_trees}");
}
