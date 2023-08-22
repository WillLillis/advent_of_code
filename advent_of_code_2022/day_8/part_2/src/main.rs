use std::{fs, cmp};

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

pub fn get_scenic_score(row: usize, col: usize, forest: &Vec<Vec<TreeVis>>) -> u32 {
    let mut up_len: u32 = 0;
    let mut down_len: u32 = 0;
    let mut left_len: u32 = 0;
    let mut right_len: u32 = 0;

    let forest_height = forest.len();
    let forest_width = forest[0].len();

    let ref_height = forest[row][col].height;

    // trees on the edges will always have a score
    // of 0 in one direction, thus their total score
    // will always be 0
    if row == 0 || row == forest_height - 1
        || col == 0 || col == forest_width - 1 {
        return 0;
    }

    // look up
    for idx in (0..row).rev() {
        up_len += 1;
        if forest[idx][col].height >= ref_height {
            break;
        }
    }
    //println!("Up score: {up_len}");
    // look down
    for idx in row+1..forest_height {
        down_len += 1;
        if forest[idx][col].height >= ref_height {
            break;
        }
    }
    //println!("Down score: {down_len}");
    // look left 
    for idx in (0..col).rev() {
        left_len += 1;
        if forest[row][idx].height >= ref_height {
            break;
        }
    }
    //println!("Left score: {left_len}");
    // look right
    for idx in col+1..forest_width {
        right_len += 1;
        if forest[row][idx].height >= ref_height {
            break;
        }
    }
    //println!("Right score: {right_len}");

    return up_len * down_len * left_len * right_len;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");
    
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut max_score: u32 = 0;
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
   
    // there has to be something better than brute force...?
    for row in 0..height {
        for col in 0..width {
            max_score = cmp::max(max_score, get_scenic_score(row, col, &forest));
        }
    }

    println!("Max scenic score: {max_score}");
}
