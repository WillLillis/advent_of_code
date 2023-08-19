use std::fs;
use std::ops::{Add, AddAssign};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct XY {
    x: i32,
    y: i32,
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

enum Orientation {
    Up,
    Down,
    Left,
    Right
}

struct Bliz {
    pos: XY,
    dir: Orientation
}

// returns:
//      - a Vec of all the blizzards (location and direction)
//      - length of the valley (including walls)
//      - height of the valley (including walls)
fn get_valley(file_name: &str) -> (Vec<Bliz>, usize, usize) {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file");

    let mut blizzs: Vec<Bliz> = Vec::new();


    return (blizzs, 0, 0);
}



// - Initial idea:
//      - First get all the blizzards working properly
//      - Start tracking position in the starting spot in a vector of positions to track
//      - At each time step, "spawn" a new position to track if a current spot can either move/
//      stay there (without collding with blizzards)
//          - Not sure if this will happen, but if there are no places where the spot can move to,
//          remove it from the list
//          - If any blizzards overlap, remove the extra ones
//      - Eventually one should reach the end, and we'll have the time step tracked all along
//  - While this probably isn't a very efficient approach, at least it's bounded by the size/
//  state of the "valley"
//  - there will be plenty of overhead for the requisite bookkeeping, but at least no recursion

fn main() {
    let (mut blizzs, val_len, val_ht) = get_valley("test_input.txt");

}
