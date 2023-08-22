use std::{fs, cmp};
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct XYZCoor<T> {
    x: T,
    y: T,
    z: T
}

impl<T> XYZCoor<T> {
    fn new(x: T, y: T, z: T) -> Self {
        XYZCoor {
            x,
            y,
            z
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    let mut points: Vec<XYZCoor<i32>> = Vec::new();

    for line in input.lines() {
        let nums: Vec<i32> = line.trim().split(&[' ', ','][..])
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        assert!(nums.len() == 3);
        points.push(XYZCoor::new(nums[0], nums[1], nums[2]));
    }

    // Not the most efficient way, but I didn't want to look at anyone else's answer
    //
    // - calculate total surface area as per part 1
    // - fill in HashSet of all non-lava blocks (within some bounding box) to  
    // represent all air blocks
    // - mark outermost "layer" of air blocks as open air, rest as "unsure"(?)
    // - iterate over all "unsure" blocks, if they're directly connected to 
    // an open air block, transfer it over to an open air block
        // - iterate until no new blocks are added
    // - remaining blocks should be all interior air blocks
    // - iterate over interior air blocks, checking each direction (+/-x,y,z)
        // - if they're adjacent to a lava block, subtract one from the surface area

    let mut x_min = i32::MAX;
    let mut y_min = i32::MAX;
    let mut z_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_max = i32::MIN;
    let mut z_max = i32::MIN;

    let mut lava_table: HashSet<XYZCoor<i32>> = HashSet::new();
    let mut open_air_table: HashSet<XYZCoor<i32>> = HashSet::new();
    let mut inner_air_table: HashSet<XYZCoor<i32>> = HashSet::new();

    for entry in &points {
        lava_table.insert(entry.clone()); 
        x_min = cmp::min(x_min, entry.x);
        y_min = cmp::min(y_min, entry.y);
        z_min = cmp::min(z_min, entry.z);

        x_max = cmp::max(x_max, entry.x);
        y_max = cmp::max(y_max, entry.y);
        z_max = cmp::max(z_max, entry.z);
    }

    x_min -= 1;
    y_min -= 1;
    z_min -= 1;
    x_max += 1;
    y_max += 1;
    z_max += 1;

    // first count up the total surface area
    let mut total_surface_area: u32 = 0;
    let mut test_point: XYZCoor<i32> = XYZCoor::new(0, 0, 0);
    for entry in &points {
        test_point.x = entry.x;
        test_point.y = entry.y;
        test_point.z = entry.z;

        // +/- x
        test_point.x -= 1;
        if !lava_table.contains(&test_point) {
            total_surface_area += 1;
        }
        test_point.x += 2;
        if !lava_table.contains(&test_point) {
            total_surface_area += 1;
        }
        test_point.x -= 1;

        // +/- y
        test_point.y -= 1;
        if !lava_table.contains(&test_point) {
            total_surface_area += 1;
        }
        test_point.y += 2;
        if !lava_table.contains(&test_point) {
            total_surface_area += 1;
        }
        test_point.y -= 1;
        
        // +/- z
        test_point.z -= 1;
        if !lava_table.contains(&test_point) {
            total_surface_area += 1;
        }
        test_point.z += 2;
        if !lava_table.contains(&test_point) {
            total_surface_area += 1;
        }
        test_point.z -= 1;
    }

    // insert the bounding box
    for y in y_min..=y_max {
        for z in z_min..=z_max {
            open_air_table.insert(XYZCoor::new(x_min, y, z));
            open_air_table.insert(XYZCoor::new(x_max, y, z));
        }
    }

    for x in x_min..=x_max {
        for z in z_min..=z_max {
            open_air_table.insert(XYZCoor::new(x, y_min, z));
            open_air_table.insert(XYZCoor::new(x, y_max, z));
        }
    }

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            open_air_table.insert(XYZCoor::new(x, y, z_min));
            open_air_table.insert(XYZCoor::new(x, y, z_max));
        }
    }

    // now all other points
    for x in x_min+1..x_max {
        for y in y_min+1..y_max {
            for z in z_min+1..z_max {
                if !lava_table.contains(&XYZCoor::new(x, y, z)) {
                    inner_air_table.insert(XYZCoor::new(x, y, z));
                }
            }
        }
    }

    let mut changed = true;
    let mut to_remove: Vec<XYZCoor<i32>> = Vec::new();
    while changed {
        changed = false;
        to_remove.clear();
        for entry in inner_air_table.iter() {
            test_point.x = entry.x;
            test_point.y = entry.y;
            test_point.z = entry.z;
    
            // +/- x
            test_point.x -= 1;
            if open_air_table.contains(&test_point) {
                to_remove.push(entry.clone());
                changed = true;
                continue;
            }
            test_point.x += 2;
            if open_air_table.contains(&test_point) {
                to_remove.push(entry.clone());
                changed = true;
                continue;
            }
            test_point.x -= 1;

            // +/- y
            test_point.y -= 1;
            if open_air_table.contains(&test_point) {
                to_remove.push(entry.clone());
                changed = true;
                continue;
            }
            test_point.y += 2;
            if open_air_table.contains(&test_point) {
                to_remove.push(entry.clone());
                changed = true;
                continue;
            }
            test_point.y -= 1;
            
            // +/- z
            test_point.z -= 1;
            if open_air_table.contains(&test_point) { 
                to_remove.push(entry.clone());
                changed = true;
                continue;
            }
            test_point.z += 2;
            if open_air_table.contains(&test_point) {
                to_remove.push(entry.clone());
                changed = true;
                continue;
            }
            test_point.z -= 1;
        }

        for entry in &to_remove {
            inner_air_table.remove(&entry);
            open_air_table.insert(entry.clone());
        }
    }

    for entry in &inner_air_table {
        test_point.x = entry.x;
        test_point.y = entry.y;
        test_point.z = entry.z;

        // +/- x
        test_point.x -= 1;
        if lava_table.contains(&test_point) {
            total_surface_area -= 1;
        }
        test_point.x += 2;
        if lava_table.contains(&test_point) {
            total_surface_area -= 1;
        }
        test_point.x -= 1;

        // +/- y
        test_point.y -= 1;
        if lava_table.contains(&test_point) {
            total_surface_area -= 1;
        }
        test_point.y += 2;
        if lava_table.contains(&test_point) {
            total_surface_area -= 1;
        }
        test_point.y -= 1;
        
        // +/- z
        test_point.z -= 1;
        if lava_table.contains(&test_point) {
            total_surface_area -= 1;
        }
        test_point.z += 2;
        if lava_table.contains(&test_point) {
            total_surface_area -= 1;
        }
        test_point.z -= 1;

    }
        
    println!("Approximate surface area: {}", total_surface_area);
}
