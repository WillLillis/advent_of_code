use std::{fs, cmp};
use std::collections::HashSet;
use std::ops::{Add, AddAssign};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct XY {
    x: i32,
    y: i32
}

impl XY {
    fn new(x: i32, y: i32) -> Self {
        XY { x, y }    
    }
}

impl Add for XY {
    type Output = Self;

    fn add(self, other: XY) -> XY {
        XY::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for XY {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug)]
struct Elf {
    curr_pos: XY,
    plan_move: Option<XY>
}

const ALL_DIRS: [XY; 8] = [XY{x: 1, y: 0}, XY{x: -1, y: 0}, XY{x: 0, y: 1}, 
      XY{x: 0, y: -1}, XY{x: 1, y: 1}, XY{x: 1, y: -1}, XY{x: -1, y: 1},
      XY{x: -1, y: -1}];

const NORTH_DIRS: [XY; 3] = [XY{x: 0, y: 1}, XY{x: 1, y: 1}, XY{x: -1, y: 1}];
const SOUTH_DIRS: [XY; 3] = [XY{x: 0, y: -1}, XY{x: 1, y: -1}, XY{x: -1, y: -1}];
const WEST_DIRS:  [XY; 3] = [XY{x: -1, y: 0}, XY{x: -1, y: 1}, XY{x: -1, y: -1}];
const EAST_DIRS:  [XY; 3] = [XY{x: 1, y: 0}, XY{x: 1, y: 1}, XY{x: 1, y: -1}];

const CHECK_DIRS: [[XY; 3]; 4] = [NORTH_DIRS, SOUTH_DIRS, WEST_DIRS, EAST_DIRS];


fn get_elves(file_name: &str) -> Vec<Elf> {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file!");

    let mut elves: Vec<Elf> = Vec::new();

    for (i, line) in input.trim().lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    elves.push(Elf {
                        curr_pos: XY::new(j as i32, 0i32 - (i as i32)),
                        plan_move: None
                    });
                },
                '.' => { continue; },
                _ => {
                    panic!("Parsing error!");
                }
            }
        }
    }

    return elves;
}

fn get_next_move(elf: &Elf, curr_elves: &HashSet<XY>, first_check_idx: usize) -> Option<XY> {
    // first check if the elf will move at all
    let mut adjacent = false;
    for dir in ALL_DIRS {
        let tmp_pos = elf.curr_pos + dir;
        if curr_elves.contains(&tmp_pos) {
            adjacent = true;
            break;
        }
    }

    if !adjacent {
        return None;
    }

    // next we have to check the directions in order
    //for set in CHECK_DIRS.iter().skip(first_check_idx).cycle().take(CHECK_DIRS.len()) {
    for set in CHECK_DIRS.iter().cycle().skip(first_check_idx).take(CHECK_DIRS.len()) {
        if set.iter().fold(true, |accum, dir| accum && !curr_elves.contains(&(elf.curr_pos + *dir))) {
            return Some(set[0]);
        }
    }

    // if no moves are available...
    return None;
}

// update curr elves hashset here??
fn do_moves(elves: &mut Vec<Elf>, curr_elves: &mut HashSet<XY>) {
    for elf in elves.iter_mut() {
       match elf.plan_move {
            Some(pos) => {
                curr_elves.remove(&elf.curr_pos);
                elf.curr_pos += pos;
                curr_elves.insert(elf.curr_pos);
            },
            None => {}
       }
    }
}

// iterate through all elves
    // if they're planning on moving, insert that position into the plan_elves hashset
    // if there's a collision, add it to the collisions hashset
// iterate through all elves
    // if they're planning on moving and their planned position isn't in the collisions hashset,
    // it's fine to move them
    // otherwise don't move them
fn set_planned_pos(elves: &mut Vec<Elf>, curr_elves: &HashSet<XY>, first_check_idx: usize) -> bool {
    let mut plan_elves: HashSet<XY> = HashSet::new();
    let mut collisions: HashSet<XY> = HashSet::new();

    for elf in elves.iter_mut() {
        match get_next_move(elf, curr_elves, first_check_idx) {
            Some(dir) => {
                if !plan_elves.insert(elf.curr_pos + dir) {
                    collisions.insert(elf.curr_pos + dir);
                    elf.plan_move = None; // unecessary, will mark as None later
                } else {
                    elf.plan_move = Some(dir);
                }
            },
            None => {
                elf.plan_move = None;
            }
        }
    }

    for elf in elves.iter_mut() {
        match elf.plan_move {
            Some(dir) => {
                if collisions.contains(&(elf.curr_pos + dir)) {
                    elf.plan_move = None;
                }
            },
            None => {}
        }
    }

    let mut moved = false;

    for elf in elves.iter() {
        if elf.plan_move != None {
            moved = true;
            break;
        }
    }

    return moved;
}

fn main() {
    let mut elves = get_elves("input.txt");

    let mut curr_elves: HashSet<XY> = HashSet::new();

    for elf in &elves {
        curr_elves.insert(elf.curr_pos);
    }
    let mut first_check_idx: usize = 0;
    let mut rounds_completed: u32 = 0;

    while set_planned_pos(&mut elves, &curr_elves, first_check_idx) {
        first_check_idx += 1 % CHECK_DIRS.len();
        do_moves(&mut elves, &mut curr_elves);
        rounds_completed += 1;
        if rounds_completed == 10 {
            break;
        }
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for pos in curr_elves.iter() {
        min_x = cmp::min(min_x, pos.x);
        max_x = cmp::max(max_x, pos.x);
        min_y = cmp::min(min_y, pos.y);
        max_y = cmp::max(max_y, pos.y);
    }

    let rect_area = ((max_x - min_x) + 1) * ((max_y - min_y) + 1);
    let free_spaces = rect_area - curr_elves.len() as i32;
    
    println!("Free spaces: {}", free_spaces);
}
