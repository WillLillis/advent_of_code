use std::fs;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum XYDir {
    Up,
    Down,
    Left,
    Right
}

impl XYDir {
    pub fn new(dir: char) -> Self {
        match dir {
            'U' => XYDir::Up,
            'D' => XYDir::Down,
            'L' => XYDir::Left,
            'R' => XYDir::Right,
            _ => {
                panic!("Invalid character passed to constructor!");
            }
        }
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct XYPos {
    pub x: i32,
    pub y: i32
}

impl XYPos {
    pub fn new(x: i32, y: i32) -> Self {
        XYPos {
            x,
            y
        }
    }

    pub fn move_dir(&mut self, dir: &XYDir) {
        match dir {
            XYDir::Up => self.y += 1,
            XYDir::Down => self.y -= 1,
            XYDir::Left => self.x -= 1,
            XYDir::Right => self.x += 1  
        };
    }

    pub fn move_knot(&mut self, head_pos: &XYPos) {
        // first check if we need to move diagonally
        if (head_pos.x != self.x && head_pos.y != self.y)
            && (i32::abs(head_pos.x - self.x) + i32::abs(head_pos.y - self.y)) > 2 {
            if head_pos.x > self.x {
                self.move_dir(&XYDir::Right);
            } else {
                self.move_dir(&XYDir::Left);
            }
            if head_pos.y > self.y {
                self.move_dir(&XYDir::Up);
            } else {
                self.move_dir(&XYDir::Down);
            }
            return;
        }
        
        // otherwise normal left/right or up/down movements
        if head_pos.x == self.x + 2 {
            self.move_dir(&XYDir::Right);
        } else if head_pos.x == self.x - 2 {
            self.move_dir(&XYDir::Left);
        }
        if head_pos.y == self.y + 2 {
            self.move_dir(&XYDir::Up);
        } else if head_pos.y == self.y - 2 {
            self.move_dir(&XYDir::Down);
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");
    let mut tail_visited: HashSet<XYPos> = HashSet::new();

    // initial position is arbitrary
    let mut knots: Vec<XYPos> = vec![XYPos::new(0, 0); 10];

    tail_visited.insert(knots.last().unwrap().clone());

    let mut steps;

    for instr in input.lines() {
        steps = instr.split_whitespace();
        let dir = steps.next().unwrap().chars().next().unwrap();
        let dir = XYDir::new(dir);
        let len = steps.next().unwrap().parse::<i32>().unwrap();
        assert!(steps.next() == None);
        for _ in 0..len {
            knots[0].move_dir(&dir);
            for i in 1..knots.len() {
                let prev_knot = knots[i-1].clone();
                knots[i].move_knot(&prev_knot);
            }
            tail_visited.insert(knots.last().unwrap().clone());
        }
    }

    println!("Total visited spaces by tail: {}",
             tail_visited.len());
}
