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

// going to assume inclusive for the sake of simplicity
struct Ranges {
    ranges: Vec<(i64, i64)>
}

impl Ranges {
    fn new(lower: i64, upper: i64) -> Self {
        Ranges { ranges: vec![(lower, upper)] }
    }
    fn insert_new_range(lower: i64, upper: i64) {
        // scan through entries, find point of overlap
        // tricky...
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
    let mut x_min = i64::MAX;
    let mut x_max = i64::MIN;
    let mut x_ranges: Vec<(i64, i64)> = vec![(i64::MAX, i64::MIN); 4000000 + 1];
    let mut coord = XYPos::new(0, 0);

    for row in 0..=4000000 {
        for (sensor, beacon) in sensors.iter().zip(beacons.iter()) {
            let beacon_dist = sensor.man_dist(beacon);

            if i64::abs(sensor.y - row) <= beacon_dist {
                x_min = cmp::min(x_min, sensor.x - i64::abs(beacon_dist - i64::abs(sensor.y - row)));
                x_min = cmp::max(0, x_min); // Apply problem's constraints
                x_max = cmp::max(x_max, sensor.x + i64::abs(beacon_dist - i64::abs(sensor.y - row)));
                x_max = cmp::max(4000000, x_max);
            }
        }
        x_ranges[row as usize].0 = cmp::min(x_ranges[row as usize].0, x_min);
        x_ranges[row as usize].1 = cmp::max(x_ranges[row as usize].1, x_max);
    }

    for (row, range) in x_ranges.iter().enumerate() {
        if range.0 != 0 || range.1 != 4000000 {
            coord = XYPos {
                x: range.0,
                y: row as i64
            };

            break;
        }
    }

    println!("Coordinate: {:?}", coord);
}
