use std::collections::{HashMap, HashSet};
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

    fn step(&self, dir: Orientation) -> Self {
        match dir {
            Orientation::Up => XY::new(self.x, self.y + 1),
            Orientation::Down => XY::new(self.x, self.y - 1),
            Orientation::Left => XY::new(self.x - 1, self.y),
            Orientation::Right => XY::new(self.x + 1, self.y),
        }
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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Bliz {
    pos: XY,
    dir: Orientation,
}

// returns:
//      - a Vec of all the blizzards (location and direction)
//      - length of the valley (including walls)
//      - height of the valley (including walls)
fn get_valley(file_name: &str) -> (HashMap<XY, Bliz>, i32, i32) {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file");

    let mut blizzs: HashMap<XY, Bliz> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let curr_pos = XY::new(x as i32, -(y as i32));
            match c {
                '.' | '#' => {
                    continue;
                }
                '^' => {
                    blizzs.insert(
                        curr_pos,
                        Bliz {
                            pos: curr_pos,
                            dir: Orientation::Up,
                        },
                    );
                }
                'v' => {
                    blizzs.insert(
                        curr_pos,
                        Bliz {
                            pos: XY::new(x as i32, -(y as i32)),
                            dir: Orientation::Down,
                        },
                    );
                }
                '<' => {
                    blizzs.insert(
                        curr_pos,
                        Bliz {
                            pos: XY::new(x as i32, -(y as i32)),
                            dir: Orientation::Left,
                        },
                    );
                }
                '>' => {
                    blizzs.insert(
                        curr_pos,
                        Bliz {
                            pos: XY::new(x as i32, -(y as i32)),
                            dir: Orientation::Right,
                        },
                    );
                }
                _ => {
                    panic!("Parsing error: Unexpected character!");
                }
            }
        }
    }

    let height = input.lines().count() as i32;
    let length = input.chars().take_while(|c| *c != '\n').count() as i32;

    return (blizzs, length, height);
}

// would like to do these updates in place but the borrow checker makes this impossible
// so instead each time step will yield a new hashset of blizzards
//      - there seems to be a choice here between using a Vec and HashSet:
//          - With a Vec we can update each blizzard's location in place, but have O(n)
//          lookup when we're checking if there's a blizzard in a given location
//          - With a HashSet, there's constant time lookup for a blizzard in a given
//          location, but we have to do a new allocation for each simulation time step
//      - going with the HashSet here but I could very well be wrong
fn update_blizzs(blizzs: &HashMap<XY, Bliz>, val_len: i32, val_ht: i32) -> HashMap<XY, Bliz> {
    let mut new_blizzs: HashMap<XY, Bliz> = HashMap::new();
    new_blizzs.reserve(blizzs.len());

    for (pos, blizz) in blizzs.iter() {
        let mut next_pos = pos.step(blizz.dir);
        if next_pos.x <= 0 {
            next_pos.x = val_len - 2; // -1 for 0-based indexing, andother -1 because of the far wall
        } else if next_pos.x >= val_len - 2 {
            next_pos.x = 1;
        } else if next_pos.y >= 0 {
            next_pos.y = -(val_ht - 2);
        } else if next_pos.y <= -(val_len - 1) {
            next_pos.y = -1;
        }

        new_blizzs.insert(
            next_pos,
            Bliz {
                pos: next_pos,
                dir: blizz.dir,
            },
        );
    }

    return new_blizzs;
}

fn in_bounds(pos: &XY, val_len: i32, val_ht: i32) -> bool {
    if pos.x <= 0 || pos.x >= val_len - 1 {
        return false;
    }

    // edge case: starting and ending position
    if (pos.y == 0 && pos.x == 1) || (pos.y == -(val_ht - 1) && pos.x == val_len - 2) {
        return true;
    }

    if pos.y <= 0 || pos.y <= -(val_ht - 1) {
        return false;
    }

    return true;
}

fn update_travel_points(
    travel_points: &HashSet<XY>,
    blizzs: &HashMap<XY, Bliz>,
    val_len: i32,
    val_ht: i32,
) -> HashSet<XY> {
    let mut new_points: HashSet<XY> = HashSet::new();
    new_points.reserve(travel_points.len());

    for point in travel_points {
        // check if we can wait
        if !blizzs.contains_key(&point) {
            new_points.insert(*point);
        }
        // check if we can move up
        let mut tmp_point = *point + XY { x: 0, y: 1 };
        if !blizzs.contains_key(&tmp_point) && in_bounds(&tmp_point, val_len, val_ht) {
            new_points.insert(tmp_point);
        }
        // check if we can move down
        tmp_point = *point + XY { x: 0, y: -1 };
        if !blizzs.contains_key(&tmp_point) && in_bounds(&tmp_point, val_len, val_ht) {
            new_points.insert(tmp_point);
        }
        // check if we can move left
        tmp_point = *point + XY { x: -1, y: 0 };
        if !blizzs.contains_key(&tmp_point) && in_bounds(&tmp_point, val_len, val_ht) {
            new_points.insert(tmp_point);
        }
        // check if we can move right
        tmp_point = *point + XY { x: 1, y: 0 };
        if !blizzs.contains_key(&tmp_point) && in_bounds(&tmp_point, val_len, val_ht) {
            new_points.insert(tmp_point);
        }
    }

    println!("{:#?}\n\n\n", new_points);

    return new_points;
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

// - change blizzs from Vec<> to HashSet<>?
// - how to store travel points?
//      - same question as blizzs

fn main() {
    let (blizzs, val_len, val_ht) = get_valley("test_input.txt");
    let mut travel_points: HashSet<XY> = HashSet::new();
    travel_points.insert(XY { x: 1, y: 0 });

    let mut count: i32 = 0;

    loop {
        //println!("{:#?}\n\n", travel_points);

        let blizzs = update_blizzs(&blizzs, val_len, val_ht);
        let travel_points = update_travel_points(&travel_points, &blizzs, val_len, val_ht);
        count += 1;

        if travel_points.contains(&XY {
            x: val_len - 2,
            y: -(val_ht - 1),
        }) {
            break;
        }
    }

    println!("Final count: {count}");
}

// TODO:
//      - Debug
