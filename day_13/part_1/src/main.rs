// I was hard-stuck on this, eventually gave up and referred to 
// https://maebli.github.io/rust/2023/01/22/100rust-75.html for help
// 
// I've never worked with/ implemented a parser before, and have barely messed with 
// Rust's compare traits, so I feel ok-ish following someone else's solution to 
// try and figure out the best practices
//
// I definitely can't claim this work as my own, and instead am going through it
// line by line to try and figure it out
//
// <List> = '[' <Items> ']'
// <Items> = <Element> | Element> ',' <Items>
// <Element> = <Integer> | <List>
// <Integer> = [0-9]*
use std::fs;

#[derive(Debug, Eq, Ord)]
struct List {
    items: Vec<Element>
}

#[derive(Debug, Eq, Ord)]
enum Element {
    Integer(u32),
    List(Box<List>)
}

impl PartialEq for Element {
    fn eq(&self, other: &Element) -> bool {
        match (self, other) {
            // if they're both the same type, compare them
            (&Element::Integer(a), &Element::Integer(b)) => a == b,
            (&Element::List(ref a), &Element::List(ref b)) => a == b,
            // if they're mixed types, they're not equal
            _ => false
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.items.partial_cmp(&other.items)
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Element) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // if they're the same type, just compare
            // if one is a list and the other an int, coerce the int into a list 
            (&Element::Integer(a), &Element::Integer(ref b)) => a.partial_cmp(b),
            (&Element::List(ref a), &Element::List(ref b)) => a.partial_cmp(b),
            (&Element::List(ref a), &Element::Integer(b)) => a.partial_cmp(
                &Box::new( List { items: vec![Element::Integer(b)] })
            ),
            (&Element::Integer(a), &Element::List(ref b)) => 
                List { items: vec![Element::Integer(a)] }.partial_cmp(b) 
        }
    }
}

fn parse_integer(input: &str) -> (Element, &str) {
   let mut cut_idx = 0;
   while cut_idx < input.len() && 
       input.as_bytes()[cut_idx] >= b'0' && 
       input.as_bytes()[cut_idx] <= b'9' {
           cut_idx += 1;
       }

   let (num, rest) = input.split_at(cut_idx);
   let num = num.parse::<u32>().unwrap();

   (Element::Integer(num), rest)
}

fn parse_next_element(input: &str) -> (Element, &str) {
    if input.starts_with('[') {
        let (list, rest) = parse_as_list(input);
        (Element::List(Box::new(list)), rest)
    } else {
        parse_integer(input)
    }
}

fn parse_as_list(input: &str) -> (List, &str) {
    let mut items = Vec::new();
    let mut input = input.trim_start();
    
    input = input.strip_prefix('[').unwrap_or("Expected leading '['!");

    while !input.starts_with(']') {
        let (next, rest) = parse_next_element(input);
        items.push(next);
        input = rest.trim_start();
        
        if input.starts_with(',') {
            input = input.strip_prefix(',').unwrap_or("Expectd ','!");
        }

    }
    input = input.strip_prefix(']').expect("Expected ']'!");
        

    (List {items}, input)
}

fn main() {
    let input = fs:: read_to_string("input.txt").unwrap();
    let mut input = input.lines();
    let mut group_num = 1;
    let mut sum = 0;

    loop {
        let left = match input.next() {
            Some(x) => x,
            None => {
                break;
            }
        };
        let right = input.next().unwrap();
        input.next(); // skip the empty line separating groups
        //println!("Left: {left}");
        //println!("Right: {right}");

        let (left_parsed, _) = parse_as_list(left);
        let (right_parsed, _) = parse_as_list(right);

        if left_parsed <= right_parsed {
            sum += group_num;
        }

        group_num += 1;
    }


    println!("Sum: {sum}");    
}
