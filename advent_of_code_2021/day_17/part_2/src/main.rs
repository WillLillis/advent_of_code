use std::cmp::{self, Ordering};
use std::{fs, ops::AddAssign, process};

#[derive(Debug, Copy, Clone)]
struct XY {
    x: i32,
    y: i32,
}

impl XY {
    fn in_area(&self, area: &Area) -> bool {
        if !(self.x >= area.bottom_left.x && self.x <= area.bottom_right.x) {
            return false;
        }

        if !(self.y >= area.bottom_left.y && self.y <= area.top_left.y) {
            return false;
        }

        return true;
    }
    // how to tell when we've passed by the target area
    // going to make some restrictive assumptions
    //      - since the problem wants the probe to be shot
    //      as high as possible, we'll assume we're approaching
    //      from above
    //      - since we start at (0, 0), we can tell if we're approaching
    //      from the left or right
    //          - some issues if the target area "straddles" but we'll
    //          ignore that for now
    fn area_unreachable(&self, area: &Area) -> bool {
        if self.y < area.bottom_left.y {
            return true;
        }
        // approaching from left to right
        if area.bottom_left.x > 0 {
            if self.x > area.bottom_right.x {
                return true;
            }
        } else if area.bottom_right.x < 0 {
            // approaching right to left
            if self.x < area.bottom_left.x {
                return true;
            }
        } else {
            panic!("Unexplored target area case! Target area straddles x=0");
        }

        return false;
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
struct Area {
    bottom_left: XY,
    top_left: XY,
    bottom_right: XY,
    top_right: XY,
}

// first get the basic sim step working
// then work out a way to efficiently search the possible v_0 vectors

fn get_target_area(file_name: &str) -> Area {
    let input = fs::read_to_string(file_name).unwrap_or_else(|err| {
        eprintln!("Error occurred while opening file 1: {err}");
        process::exit(1);
    });

    let vals: Vec<i32> = input
        .trim()
        .split(&[' ', '.', ',', '='])
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();
    assert!(vals.len() == 4);

    let bottom_left = XY {
        x: vals[0],
        y: vals[2],
    };
    let top_left = XY {
        x: vals[0],
        y: vals[3],
    };
    let bottom_right = XY {
        x: vals[1],
        y: vals[2],
    };
    let top_right = XY {
        x: vals[1],
        y: vals[3],
    };

    return Area {
        bottom_left,
        top_left,
        bottom_right,
        top_right,
    };
}

fn sim_step(pos: &mut XY, velocity: &mut XY) {
    *pos += *velocity;
    match velocity.x.cmp(&0) {
        Ordering::Less => {
            velocity.x += 1;
        }
        Ordering::Equal => {}
        Ordering::Greater => {
            velocity.x -= 1;
        }
    }
    velocity.y -= 1;
}

// returns Some(max_height) if target area is reached,
// None if target area is missed
fn sim_shot(v0: XY, target: &Area) -> Option<i32> {
    let mut max_ht = 0;
    let mut pos = XY { x: 0, y: 0 };
    let mut velocity = v0;

    loop {
        if pos.in_area(target) {
            return Some(max_ht);
        }
        if pos.area_unreachable(target) {
            return None;
        }
        sim_step(&mut pos, &mut velocity);

        max_ht = cmp::max(max_ht, pos.y)
    }
}

fn main() {
    let area = get_target_area("input.txt");

    let init_x_velo = if area.bottom_left.x > 0 { 1 } else { -1 };

    let mut n_velos = 0;

    if init_x_velo > 0 {
        for x_velo in init_x_velo..=area.bottom_right.x {
            // what's a reasonable stopping condition for y velo?
            for y_velo in -1000..1000 {
                let v0 = XY {
                    x: x_velo,
                    y: y_velo,
                };
                match sim_shot(v0, &area) {
                    Some(_) => {
                        n_velos += 1;
                    }
                    None => {}
                }
            }
        }
    } else {
        for x_velo in (init_x_velo..=area.bottom_left.x).rev() {
            // what's a reasonable stopping condition for y velo?
            for y_velo in -1000..1000 {
                let v0 = XY {
                    x: x_velo,
                    y: y_velo,
                };
                match sim_shot(v0, &area) {
                    Some(_) => {
                        n_velos += 1;
                    }
                    None => {}
                }
            }
        }
    }

    println!("Number of velos: {n_velos}");
}
