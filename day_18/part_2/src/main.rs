use std::fs;
use std::collections::HashSet;

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
    let input = fs::read_to_string("test_input.txt")
        .expect("Failed to read input file");

    let mut points: Vec<XYZCoor<i32>> = Vec::new();

    for line in input.lines() {
        let nums: Vec<i32> = line.trim().split(&[' ', ','][..])
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        assert!(nums.len() == 3);
        points.push(XYZCoor::new(nums[0], nums[1], nums[2]));
    }

    let mut lava_table: HashSet<XYZCoor<i32>> = HashSet::new();

    for entry in &points {
        lava_table.insert(entry.clone()); 
    }

    // how to detect/ count interior points????
    // now count up the surface area
    let mut surface_area: u32 = 0;
    let mut test_point: XYZCoor<i32> = XYZCoor::new(0, 0, 0);
    for entry in &points {
        test_point.x = entry.x;
        test_point.y = entry.y;
        test_point.z = entry.z;

        // +/- x
        test_point.x -= 1;
        if !lava_table.contains(&test_point) {
            surface_area += 1;
        }
        test_point.x += 2;
        if !lava_table.contains(&test_point) {
            surface_area += 1;
        }
        test_point.x -= 1;

        // +/- y
        test_point.y -= 1;
        if !lava_table.contains(&test_point) {
            surface_area += 1;
        }
        test_point.y += 2;
        if !lava_table.contains(&test_point) {
            surface_area += 1;
        }
        test_point.y -= 1;
        
        // +/- z
        test_point.z -= 1;
        if !lava_table.contains(&test_point) {
            surface_area += 1;
        }
        test_point.z += 2;
        if !lava_table.contains(&test_point) {
            surface_area += 1;
        }
        test_point.z -= 1;
    }
        
    println!("Approximate surface area: {}", surface_area);
}
