use std::{fs, cmp};
use std::ops::Range;
use std::collections::HashSet;

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
    let row = 2000000i64;
    let mut x_min = i64::MAX;
    let mut x_max = i64::MIN; 

    for (sensor, beacon) in sensors.iter().zip(beacons.iter()) {
        let beacon_dist = sensor.man_dist(beacon);

        if i64::abs(sensor.y - row) <= beacon_dist {
            x_min = cmp::min(x_min, sensor.x - i64::abs(beacon_dist - i64::abs(sensor.y - row)));
            x_max = cmp::max(x_max, sensor.x + i64::abs(beacon_dist - i64::abs(sensor.y - row)));
        }
    }

    let mut removed_beacons: HashSet<XYPos> = HashSet::new();

    let mut count = x_max - x_min + 1;
    let range = Range { start: x_min, end: x_max + 1 };

    // remove actual beacon locations from the hash set, if they're there
        // (avoiding duplicate beacons) 
    for beacon in beacons {
        if beacon.y == row && range.contains(&beacon.x) && 
            !removed_beacons.contains(&beacon) {
            count -= 1;
            removed_beacons.insert(beacon);
        }
    }

    println!("Number of positions: {}", count);
}
