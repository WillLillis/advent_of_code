use std::str::FromStr;
use std::{fs, process};
use std::iter::Peekable;
use std::ops::Add;

// <SnailfishNum> = <Pair>
// <Pair> = <Pair> | <Num> , <Pair> | <Num>
// <Num> = [1-9]
#[derive(Debug)]
struct SnailfishNum {
    val: Pair,
}

#[derive(Debug)]
struct Pair {
    left_val: PairEntry,
    right_val: PairEntry,
}

#[derive(Debug)]
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

    fn add(self, rhs: Self) -> Self::Output {
        todo!();
    }
     
}

fn parse_pair<T>(stream: &mut Peekable<T>) -> Option<Pair>
where
    T: Iterator<Item = char>
{
    // Opening '['
    match stream.next() {
        Some('[') => {},
        _ => {
            return None;
        }
    }

    let left_val = match stream.peek() {
        Some('[') => {
            match parse_pair(stream) {
                Some(val) => PairEntry::Pair(Box::new(val)),
                None => { return None; }
            }

        }
        Some(x) => {
            if let Some(num) = x.to_digit(10) {
                stream.next(); // have to advance the iterator
                PairEntry::Val(num) 
            } else {
                return None;
            }
        },
        _ => {
            return None;
        }
    };

    match stream.next() {
        Some(',') => {},
        _ => { return None; }
    }

    let right_val = match stream.peek() {
        Some('[') => {
            match parse_pair(stream) {
                Some(val) => PairEntry::Pair(Box::new(val)),
                None => { return None; }
            }

        }
        Some(x) => {
            if let Some(num) = x.to_digit(10) {
                stream.next(); // have to advance the iterator
                PairEntry::Val(num) 
            } else {
                return None;
            }
        },
        _ => {
            return None;
        }
    };

    match stream.next() {
        Some(']') => {},
        _ => { return None; }
    }

    return Some(Pair { left_val, right_val });
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
