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

impl Orientation {
    fn to_char(&self) -> char {
        match *self {
            Orientation::Up => '^',
            Orientation::Down => 'v',
            Orientation::Left => '<',
            Orientation::Right => '>',
        }
    }
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
fn get_valley(file_name: &str) -> (Vec<Bliz>, i32, i32) {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file");

    let mut blizzs: Vec<Bliz> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let curr_pos = XY::new(x as i32, -(y as i32));
            match c {
                '.' | '#' => {
                    continue;
                }
                '^' => {
                    blizzs.push(Bliz {
                        pos: curr_pos,
                        dir: Orientation::Up,
                    });
                }
                'v' => {
                    blizzs.push(Bliz {
                        pos: XY::new(x as i32, -(y as i32)),
                        dir: Orientation::Down,
                    });
                }
                '<' => {
                    blizzs.push(Bliz {
                        pos: XY::new(x as i32, -(y as i32)),
                        dir: Orientation::Left,
                    });
                }
                '>' => {
                    blizzs.push(Bliz {
                        pos: XY::new(x as i32, -(y as i32)),
                        dir: Orientation::Right,
                    });
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

fn update_blizzs(blizzs: &mut Vec<Bliz>, val_len: i32, val_ht: i32) {
    for blizz in blizzs.iter_mut() {
        let mut next_pos = blizz.pos.step(blizz.dir);
        if next_pos.x <= 0 {
            next_pos.x = val_len - 2; // -1 for 0-based indexing, andother -1 because of the far wall
        } else if next_pos.x >= val_len - 1 {
            next_pos.x = 1;
        } else if next_pos.y >= 0 {
            next_pos.y = -(val_ht - 2);
        } else if next_pos.y <= -(val_ht - 1) {
            next_pos.y = -1;
        }

        blizz.pos = next_pos;
    }
}

fn in_bounds(pos: &XY, val_len: i32, val_ht: i32) -> bool {
    if (pos.x <= 0) || (pos.x >= val_len - 1) {
        return false;
    }

    // edge case: starting and ending position
    if (pos.y == 0 && pos.x == 1) || ((pos.y == -(val_ht - 1)) && (pos.x == val_len - 2)) {
        return true;
    }

    if pos.y >= 0 || (pos.y <= -(val_ht - 1)) {
        return false;
    }

    return true;
}

fn update_travel_points(
    travel_points: &HashSet<XY>,
    blizzs: &Vec<Bliz>,
    val_len: i32,
    val_ht: i32,
) -> HashSet<XY> {
    let tmp_blizzs_loc: HashMap<XY, Orientation> = blizzs
        .iter()
        .map(|blizz| (blizz.pos, blizz.dir))
        .collect::<HashMap<XY, Orientation>>();

    let mut new_points: HashSet<XY> = HashSet::new();
    new_points.reserve(travel_points.len());

    for point in travel_points {
        // check if we can wait
        if !tmp_blizzs_loc.contains_key(&point) {
            new_points.insert(*point);
        }
        // check if we can move up
        let mut tmp_point = *point + XY { x: 0, y: 1 };
        if !tmp_blizzs_loc.contains_key(&tmp_point) && in_bounds(&tmp_point, val_len, val_ht) {
            new_points.insert(tmp_point);
        }
        // check if we can move down
        tmp_point = *point + XY { x: 0, y: -1 };
        if !tmp_blizzs_loc.contains_key(&tmp_point) && in_bounds(&tmp_point, val_len, val_ht) {
            new_points.insert(tmp_point);
        }
        // check if we can move left
        tmp_point = *point + XY { x: -1, y: 0 };
        if !tmp_blizzs_loc.contains_key(&tmp_point) && in_bounds(&tmp_point, val_len, val_ht) {
            new_points.insert(tmp_point);
        }
        // check if we can move right
        tmp_point = *point + XY { x: 1, y: 0 };
        if !tmp_blizzs_loc.contains_key(&tmp_point) && in_bounds(&tmp_point, val_len, val_ht) {
            new_points.insert(tmp_point);
        }
    }

    return new_points;
}

// For debugging
fn print_state(blizzs: &Vec<Bliz>, travel_points: &HashSet<XY>, val_len: i32, val_ht: i32) {
    let tmp_blizzs: HashMap<XY, Orientation> = blizzs
        .iter()
        .map(|blizz| (blizz.pos, blizz.dir))
        .collect::<HashMap<XY, Orientation>>();

    print!("#");
    if travel_points.contains(&XY { x: 1, y: 0 }) {
        print!("E");
    } else {
        print!(".");
    }
    for _ in 2..val_len {
        print!("#");
    }
    println!("");

    for y in (-(val_ht - 2)..=-1).rev() {
        print!("#");
        for x in 1..val_len - 1 {
            let tmp_point = XY { x, y };
            if travel_points.contains(&tmp_point) {
                print!("E");
            } else if let Some(dir) = tmp_blizzs.get(&tmp_point) {
                // will only show one of
                // overlapping points
                print!("{}", dir.to_char());
            } else {
                print!(".");
            }
        }
        println!("#");
    }

    for _ in 0..val_len - 2 {
        print!("#");
    }
    if travel_points.contains(&XY {
        x: val_len - 2,
        y: -(val_ht - 1),
    }) {
        print!("E");
    } else {
        print!(".");
    }
    println!("#");
}

fn main() {
    let (mut blizzs, val_len, val_ht) = get_valley("input.txt");
    let mut travel_points: HashSet<XY> = HashSet::new();
    travel_points.insert(XY { x: 1, y: 0 });

    let mut count: i32 = 0;

    loop {
        update_blizzs(&mut blizzs, val_len, val_ht);
        let new_travel_points = update_travel_points(&travel_points, &blizzs, val_len, val_ht);
        travel_points = new_travel_points;
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
