use std::iter::Peekable;
use std::ops::Add;
use std::str::FromStr;
use std::{fs, process};

// <SnailfishNum> = <Pair>
// <Pair> = <Pair> | <Num> , <Pair> | <Num>
// <Num> = [1-9]
#[derive(Debug, Clone)]
struct SnailfishNum {
    val: Pair,
}

#[derive(Debug, Clone)]
struct Pair {
    left_val: PairEntry,
    right_val: PairEntry,
}

#[derive(Debug, Clone)]
enum PairEntry {
    Pair(Box<Pair>),
    Val(u32),
}

#[derive(Debug)]
struct ParseSnailfishNumError;

impl FromStr for SnailfishNum {
    type Err = ParseSnailfishNumError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stream = s.chars().peekable();

        let pair = match parse_pair(&mut stream) {
            Some(parsed_pair) => parsed_pair,
            None => {
                return Err(ParseSnailfishNumError);
            }
        };
        let parsed = SnailfishNum { val: pair };

        return Ok(parsed);
    }
}

impl Add for SnailfishNum {
    type Output = Self;
    // wanted to overload the + operator, but having to pass both by value
    // seems memory intensive...idk should be fine for this problem
    fn add(self, rhs: Self) -> Self::Output {
        let lhs = PairEntry::Pair(Box::new(self.val.clone()));
        let rhs = PairEntry::Pair(Box::new(rhs.val.clone()));
        let mut res = SnailfishNum {
            val: Pair {
                left_val: lhs,
                right_val: rhs,
            },
        };
        res.reduce();

        return res;
    }
}

impl SnailfishNum {
    fn reduce(&mut self) {
        loop {
            // If any pair is nested inside four pairs, the leftmost such pair explodes.
            if let PairEntry::Pair(ref mut x) = &mut self.val.left_val {
                // check if a right val is passed up here, and then try to add it to the first
                // right val at the top level
                let test = SnailfishNum::find_nested(x, 1);
            }
            if let PairEntry::Pair(ref mut x) = &mut self.val.right_val {
                // same here with the left...
                // will need to test/ play with it
                let test = SnailfishNum::find_nested(x, 1);
            }
            // If any regular number is 10 or greater, the leftmost such regular number splits.
        }
    }
    // play around with what this returns????
    // (true/false, Option<i32>, Option<i32>) maybe?
    fn find_nested(curr: &mut Pair, depth: u32) -> (bool, Option<u32>, Option<u32>) {
        if depth == 4 {
            let left_val = if let PairEntry::Val(left) = curr.left_val {
                Some(left)
            } else {
                None
            };
            let right_val = if let PairEntry::Val(right) = curr.right_val {
                Some(right)
            } else {
                None
            };
            return match (left_val.is_some(), right_val.is_some()) {
                (true, true) => (true, left_val, right_val),
                _ => {
                    (false, None, None)
                    //panic!("Invalid SnailfishNum!");
                }
            };
        }
        {
            //                                      should the &mut be here?
            let res_left = if let PairEntry::Pair(ref mut x) = &mut curr.left_val {
                SnailfishNum::find_nested(x, depth + 1)
            } else {
                (false, None, None)
            };
            match res_left {
                // if we just changed a left entry we can look to add to the right, if it's a
                // normal number
                (true, left, Some(right)) => {
                    if let PairEntry::Val(ref mut old_right) = &mut curr.right_val {
                        *old_right += right;
                        return (true, left, None);
                    } else {
                        return (true, left, Some(right));
                    }
                }
                (true, _, _) => {
                    return res_left;
                }
                (false, _, _) => (), // keep searching
            }
        }
        {
            let res_right = if let PairEntry::Pair(ref mut x) = &mut curr.right_val {
                SnailfishNum::find_nested(x, depth + 1)
            } else {
                (false, None, None)
            };
            match res_right {
                // if we just changed a right entry we can look to add to the left, if it's a
                // normal number
                (true, Some(left), right) => {
                    if let PairEntry::Val(ref mut old_left) = &mut curr.left_val {
                        *old_left += left;
                        return (true, None, right);
                    } else {
                        return (true, Some(left), right);
                    }
                }
                _ => {
                    return res_right;
                }
            }
        }
    }
}

fn parse_pair<T>(stream: &mut Peekable<T>) -> Option<Pair>
where
    T: Iterator<Item = char>,
{
    // Opening '['
    match stream.next() {
        Some('[') => {}
        _ => {
            return None;
        }
    }

    let left_val = match stream.peek() {
        Some('[') => match parse_pair(stream) {
            Some(val) => PairEntry::Pair(Box::new(val)),
            None => {
                return None;
            }
        },
        Some(x) => {
            if let Some(num) = x.to_digit(10) {
                stream.next(); // have to advance the iterator
                PairEntry::Val(num)
            } else {
                return None;
            }
        }
        _ => {
            return None;
        }
    };

    match stream.next() {
        Some(',') => {}
        _ => {
            return None;
        }
    }

    let right_val = match stream.peek() {
        Some('[') => match parse_pair(stream) {
            Some(val) => PairEntry::Pair(Box::new(val)),
            None => {
                return None;
            }
        },
        Some(x) => {
            if let Some(num) = x.to_digit(10) {
                stream.next(); // have to advance the iterator
                PairEntry::Val(num)
            } else {
                return None;
            }
        }
        _ => {
            return None;
        }
    };

    match stream.next() {
        Some(']') => {}
        _ => {
            return None;
        }
    }

    return Some(Pair {
        left_val,
        right_val,
    });
}

fn get_nums(file_name: &str) -> Vec<SnailfishNum> {
    let input = fs::read_to_string(file_name).unwrap_or_else(|err| {
        eprintln!("Error occurred while opening the input file: {err}");
        process::exit(1);
    });

    return input
        .lines()
        .filter_map(|line| line.trim().parse::<SnailfishNum>().ok())
        .collect::<Vec<SnailfishNum>>();
}

fn main() {
    let nums = get_nums("test_input.txt");

    for num in nums.iter() {
        println!("{:#?}", num);
    }
}
