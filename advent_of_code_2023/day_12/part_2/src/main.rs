use std::collections::VecDeque;

use itertools::Itertools;
use memoize::memoize;
use rayon::prelude::*;

// make this global to simplify things a bit for the main run
static mut CONTIG_SIZES: Vec<Vec<u32>> = Vec::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum SpringCondition {
    Operational, // .
    Damaged,     // #
    Unknown,     // ?
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SpringRecord {
    id: usize,
    springs: Vec<SpringCondition>,
    alt_springs: VecDeque<SpringCondition>,
    contig_sizes: Vec<u32>,
    alt_contig_sizes: VecDeque<u32>,
}

fn get_records(input: &str) -> Vec<SpringRecord> {
    let mut records = Vec::new();
    let mut id = 0usize;

    for line in input.lines() {
        let pieces: Vec<&str> = line.split_whitespace().collect();
        assert!(pieces.len() == 2);
        let mut springs = pieces[0]
            .chars()
            .map(|c| match c {
                '.' => SpringCondition::Operational,
                '#' => SpringCondition::Damaged,
                '?' => SpringCondition::Unknown,
                _ => {
                    panic!("Invalid character");
                }
            })
            .collect::<Vec<SpringCondition>>();
        springs.push(SpringCondition::Unknown);
        let springs = springs.repeat(5);
        let contig_sizes: Vec<u32> = pieces[1]
            .split(',')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<u32>>()
            .repeat(5);

        records.push(SpringRecord {
            id,
            springs: springs.clone(),
            alt_springs: springs.into(),
            contig_sizes: contig_sizes.clone(),
            alt_contig_sizes: contig_sizes.into(),
        });

        id += 1;
    }

    records
}

fn validate_springs(springs: &Vec<SpringCondition>, contig_sizes: &Vec<u32>) -> bool {
    let mut curr_count = 0u32;
    let mut found_sizes = Vec::new();

    for spring in springs.iter() {
        match spring {
            SpringCondition::Operational => {
                if curr_count > 0 {
                    found_sizes.push(curr_count);
                    curr_count = 0;
                }
            }
            SpringCondition::Damaged => {
                curr_count += 1;
            }
            SpringCondition::Unknown => {
                unreachable!()
            }
        }
    }

    if curr_count > 0 {
        found_sizes.push(curr_count);
    }

    if found_sizes.ne(contig_sizes) {
        return false;
    }

    true
}

fn validate_springs2(springs: &Vec<SpringCondition>, id: usize) -> bool {
    let mut curr_count = 0u32;
    let mut found_sizes = Vec::new();

    for spring in springs.iter() {
        match spring {
            SpringCondition::Operational => {
                if curr_count > 0 {
                    found_sizes.push(curr_count);
                    curr_count = 0;
                }
            }
            SpringCondition::Damaged => {
                curr_count += 1;
            }
            SpringCondition::Unknown => {
                unreachable!()
            }
        }
    }

    if curr_count > 0 {
        found_sizes.push(curr_count);
    }

    unsafe {
        if found_sizes.ne(&CONTIG_SIZES[id]) {
            return false;
        }
    }

    true
}

// spent all day trying to solve this and dug myself in a hole
// found this to be very illuminating:
// https://github.com/matheusstutzel/adventOfCode/blob/main/2023/12/p1.py
fn recur(record: &mut SpringRecord) -> usize {
    if !record.springs.contains(&SpringCondition::Unknown) {
        return if validate_springs(&record.springs, &record.contig_sizes) {
            1
        } else {
            0
        };
    }

    let idx = record
        .springs
        .iter()
        .find_position(|x| **x == SpringCondition::Unknown)
        .unwrap()
        .0;
    // simplify here?
    record.springs[idx] = SpringCondition::Damaged;
    let damaged_count = recur(record);
    record.springs[idx] = SpringCondition::Operational;
    let op_count = recur(record);
    record.springs[idx] = SpringCondition::Unknown;

    return damaged_count + op_count;
}

#[memoize]
fn recursion_again(springs: VecDeque<SpringCondition>, sizes: VecDeque<u32>) -> usize {
    // check for base case
    match (springs.is_empty(), sizes.is_empty()) {
        (true, true) => {
            return 1;
        }
        (true, false) => {
            return 0;
        }
        _ => {}
    }

    let mut springs = springs.clone();
    match springs.front() {
        Some(SpringCondition::Operational) => {
            springs.pop_front();
            return recursion_again(springs, sizes);
        }
        Some(SpringCondition::Unknown) => {
            // Try swapping the unknown node with an operational node, and then a damaged one

            // Operational first
            springs[0] = SpringCondition::Operational;
            let mut count = recursion_again(springs.clone(), sizes.clone());

            // Now for damaged. make sure doing so could lead to a valid config
            if !sizes.is_empty() && springs.len() >= sizes[0] as usize {
                // ensure the next group size is "doable"
                springs[0] = SpringCondition::Damaged;
                if !springs
                    .iter()
                    .take(sizes[0] as usize)
                    .contains(&SpringCondition::Operational)
                {
                    // mark the next group as damaged
                    for i in 0..sizes[0] as usize {
                        springs[i] = SpringCondition::Damaged;
                    }
                    // check one past to avoid an edge case
                    if springs.len() > sizes[0] as usize {
                        match springs[sizes[0] as usize] {
                            SpringCondition::Damaged => {
                                // dead end
                            }
                            SpringCondition::Operational => {
                                // ok
                                count += recursion_again(springs.clone(), sizes.clone());
                            }
                            SpringCondition::Unknown => {
                                // mark as operational
                                springs[sizes[0] as usize] = SpringCondition::Operational;
                                count += recursion_again(springs.clone(), sizes.clone());
                            }
                        }
                    } else {
                        // otherwise if there's nothing past our group, we're find to just recurse
                        count += recursion_again(springs.clone(), sizes.clone());
                    }
                }
            }
            return count;
        }
        Some(SpringCondition::Damaged) => {
            // make sure we can make the next group
            if sizes.is_empty() {
                return 0;
            }
            let mut sizes = sizes.clone();
            let group_size = sizes.pop_front().unwrap();
            if springs.len() < group_size as usize {
                return 0;
            }

            let group = springs.drain(0..group_size as usize);
            if group.into_iter().contains(&SpringCondition::Operational) {
                return 0;
            }
            // check one past our group, similar edge case to the SpringCondition::Unknown case
            if !springs.is_empty() {
                match springs[0] {
                    SpringCondition::Operational => {}
                    SpringCondition::Unknown => {
                        springs[0] = SpringCondition::Operational;
                    }
                    SpringCondition::Damaged => {
                        return 0;
                    }
                }
            }

            return recursion_again(springs.clone(), sizes.clone());
        }
        None => {
            unreachable!()
        }
    }
}

// using SharedCache option leads to a LOT of kernel time for each CPU thread,
// seems to slow everything down...
#[memoize(Ignore: id, Capacity: 1000000)]
fn recur_memo2(springs: Vec<SpringCondition>, id: usize) -> usize {
    let mut springs = springs.clone();
    if !springs.contains(&SpringCondition::Unknown) {
        return if validate_springs2(&springs, id) {
            1
        } else {
            0
        };
    }

    let idx = springs
        .iter()
        .find_position(|x| **x == SpringCondition::Unknown)
        .unwrap()
        .0;

    // simplify here?
    springs[idx] = SpringCondition::Damaged;
    let damaged_count = recur_memo2(springs.clone(), id);
    springs[idx] = SpringCondition::Operational;
    let op_count = recur_memo2(springs.clone(), id);
    springs[idx] = SpringCondition::Unknown;

    return damaged_count + op_count;
}

fn main() {
    let input = std::fs::read_to_string("../test_input").expect("Failed to read the input file");
    //let mut records = get_records(&input);
    let records = get_records(&input);
    unsafe {
        for record in records.iter() {
            CONTIG_SIZES.push(record.contig_sizes.clone());
        }
    }

    let sum: usize = records
        //.par_iter_mut()
        //.par_iter()
        .iter()
        //.iter_mut()
        .enumerate()
        .map(|(i, r)| {
            //.map(|(i, mut r)| {
            //let count = recur(&mut r);
            //let count = recur_memo2(r.springs.clone(), r.id);
            println!("{:?}", r.alt_springs);
            let count = recursion_again(r.alt_springs.clone(), r.alt_contig_sizes.clone());
            println!("{i}: {count}");
            count
        })
        .sum();
    println!("Sum: {}", sum);

    /*println!("Test: {:#?}", records[2]);
    println!(
        "Combinations: {}",
        recursion_again(
            records[2].alt_springs.clone(),
            records[2].alt_contig_sizes.clone(),
            0
        )
    );*/
}
