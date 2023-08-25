use std::collections::HashMap;
use std::{cmp, fs};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct XY {
    x: i32,
    y: i32,
}

impl XY {
    fn new(x: i32, y: i32) -> Self {
        XY { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    p1: XY,
    p2: XY,
}

impl Line {
    fn new(p1: &XY, p2: &XY) -> Self {
        Line {
            p1: p1.clone(),
            p2: p2.clone(),
        }
    }

    fn get_points(&self) -> Vec<XY> {
        let mut points: Vec<XY> = Vec::new();
        if self.is_vert() {
            let x = self.p1.x;
            if self.p1.y == self.p2.y {
                points.push(XY::new(x, self.p1.y));
            } else if self.p1.y <= self.p2.y {
                for y in self.p1.y..=self.p2.y {
                    points.push(XY::new(x, y));
                }
            } else {
                for y in self.p2.y..=self.p1.y {
                    points.push(XY::new(x, y));
                }
            }
        } else if self.is_horiz() {
            let y = self.p1.y;
            if self.p1.x == self.p2.x {
                points.push(XY::new(self.p1.x, y));
            } else if self.p1.x <= self.p2.x {
                for x in self.p1.x..=self.p2.x {
                    points.push(XY::new(x, y));
                }
            } else {
                for x in self.p2.x..=self.p1.x {
                    points.push(XY::new(x, y));
                }
            }
        } else {
            if self.p1 == self.p2 {
                points.push(self.p1.clone());
            } else {
                if self.p1.x <= self.p2.x {
                    if self.p1.y <= self.p2.y {
                        for (x, y) in (self.p1.x..=self.p2.x).zip(self.p1.y..=self.p2.y) {
                            points.push(XY::new(x, y));
                        }
                    } else {
                        for (x, y) in (self.p1.x..=self.p2.x).zip((self.p2.y..=self.p1.y).rev()) {
                            points.push(XY::new(x, y));
                        }
                    }
                } else {
                    if self.p1.y <= self.p2.y {
                        for (x, y) in (self.p2.x..=self.p1.x).zip((self.p1.y..=self.p2.y).rev()) {
                            points.push(XY::new(x, y));
                        }
                    } else {
                        for (x, y) in (self.p2.x..=self.p1.x).zip(self.p2.y..=self.p1.y) {
                            points.push(XY::new(x, y));
                        }
                    }
                }
            }
        }
        
        return points;
    }

    fn is_vert(&self) -> bool {
        return self.p1.x == self.p2.x;
    }

    fn is_horiz(&self) -> bool {
        return self.p1.y == self.p2.y;
    }
}

fn get_lines(file_name: &str) -> Vec<Line> {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file");
    let mut lines: Vec<Line> = Vec::new();

    for line in input.lines() {
        let nums: Vec<i32> = line
            .trim()
            .split(&['-', '>', ',', ' '][..])
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        assert!(nums.len() == 4);
        let p1 = XY::new(nums[0], nums[1]);
        let p2 = XY::new(nums[2], nums[3]);
        lines.push(Line::new(&p1, &p2));
    }

    return lines;
}

fn main() {
    let lines = get_lines("input.txt");

    let mut counts: HashMap<XY, u32> = HashMap::new();

    for line in lines.iter() {
        let points = line.get_points();
        for point in points {
            counts
                .entry(point)
                .and_modify(|count| *count += 1)
                .or_insert(1u32);
        }
    }

    let num_overlaps = counts.iter().fold(0, |accum, (_, &count)| {
        accum + if count > 1 { 1 } else { 0 }
    });

    println!("Number of overlaps: {num_overlaps}");
}
