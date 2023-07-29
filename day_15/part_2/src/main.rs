use std::{fs, cmp};
// use std::ops::Range;

#[derive(Debug, Hash, Eq, PartialEq)]
struct XYPos {
    x: i64,
    y: i64
}

impl XYPos {
    fn new(x: i64, y: i64) -> Self {
        XYPos {
            x,
            y
        }
    }

    fn man_dist(&self, other: &XYPos) -> i64 {
        i64::abs(self.x - other.x) + i64::abs(self.y - other.y) 
    }
}

#[derive(Debug, Clone)]
struct Ranges {
    pub ranges: Vec<std::ops::RangeInclusive<i64>>
}

impl Ranges {
    fn new(lower: i64, upper: i64) -> Self {
        if lower > upper {
            panic!("Invalid range passed to constructor!");
        }
        Ranges { ranges: vec![lower..=upper] }
    }
    // Modified from https://www.geeksforgeeks.org/merging-intervals/#
    // The code on the website did not work lol
    fn insert_new_range(&mut self, lower: i64, upper: i64) {
        if lower > upper {
            panic!("Invalid range passed to constructor!");
        }

        let mut inserted = false;
        // assume the Vec is sorted, insert new entry sorted according to lower entry
        for idx in 0..self.ranges.len() {
            if self.ranges[idx].start() > &lower {
                self.ranges.insert(idx, lower..=upper);
                inserted = true;
                break;
            }
        }
        if !inserted {
            self.ranges.push(lower..=upper);
        }
        
        let mut idx: usize = 0;
        let mut i: usize = 1;
        while i < self.ranges.len() {
            if (self.ranges[idx].end() >= self.ranges[i].start()) ||
            (*self.ranges[idx].end() == (self.ranges[i].start() - 1)) {
                let new_start = self.ranges[idx].start();
                let new_end = cmp::max(self.ranges[idx].end(), self.ranges[i].end());

                //println!("{new_start}..={new_end}");
                self.ranges[idx] = *new_start..=*new_end;
                self.ranges.remove(i);
            } else {
                idx += 1;
                i += 1;
            }
        } 
    }
}


fn parse_data(file_name: &str) -> (Vec<XYPos>, Vec<XYPos>) {
    let input = fs::read_to_string(file_name).unwrap();
    let mut sensors: Vec<XYPos> = Vec::new();
    let mut beacons: Vec<XYPos> = Vec::new();


    for line in input.lines() {
        let line: Vec<i64> = line.split(&['=', ',', ':'][..])
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        assert!(line.len() == 4);
        sensors.push(XYPos::new(line[0], line[1]));
        beacons.push(XYPos::new(line[2], line[3]));
    }

    (sensors, beacons)
}

fn main() {
    let (sensors, beacons) = parse_data("input.txt");
    let max_x: i64 = 4000000;
    let max_y: i64 = 4000000;
    let mut x_ranges: Vec<Ranges> = vec![Ranges::new(0, 0); max_y as usize + 1];

    for row in 0..=max_y {
        for (sensor, beacon) in sensors.iter().zip(beacons.iter()) {
            let mut x_min = max_x; 
            let mut x_max = 0; 
            let beacon_dist = sensor.man_dist(beacon);

            if i64::abs(sensor.y - row) <= beacon_dist {
                x_min = cmp::min(x_min, sensor.x - i64::abs(beacon_dist - i64::abs(sensor.y - row)));
                x_min = cmp::max(0, x_min); // Apply problem's constraints
                x_max = cmp::max(x_max, sensor.x + i64::abs(beacon_dist - i64::abs(sensor.y - row)));
                x_max = cmp::min(max_x, x_max);
                x_ranges[row as usize].insert_new_range(x_min, x_max);
            }
        }
    }

    for (row, range) in x_ranges.iter().enumerate() {
        if range.ranges.len() > 1 { 
            println!("Row: {row}, Range: {:?}", range);
            let x: i64 = range.ranges[0].end() + 1;
            let y: i64 = row as i64;
            println!("Coordinate: {x}, {y}");
            println!("Tuning frequency: {}", x * 4000000i64 + y);
        }
    }
}
