use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum SpringCondition {
    Operational, // .
    Damaged,     // #
    Unknown,     // ?
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct SpringRecord {
    springs: Vec<SpringCondition>,
    contig_sizes: Vec<u32>,
}

fn get_records(input: &str) -> Vec<SpringRecord> {
    let mut records = Vec::new();

    for line in input.lines() {
        let pieces: Vec<&str> = line.split_whitespace().collect();
        assert!(pieces.len() == 2);
        let springs = pieces[0]
            .chars()
            .map(|c| match c {
                '.' => SpringCondition::Operational,
                '#' => SpringCondition::Damaged,
                '?' => SpringCondition::Unknown,
                _ => {
                    panic!("Invalid character");
                }
            })
            .collect();
        let contig_sizes: Vec<u32> = pieces[1]
            .split(',')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();

        records.push(SpringRecord {
            springs,
            contig_sizes,
        });
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

// spent all day trying to solve this and dug myself in a hole
// found this to be very illuminating:
// https://github.com/matheusstutzel/adventOfCode/blob/main/2023/12/p1.py
fn recur(springs: &mut Vec<SpringCondition>, contig_sizes: &Vec<u32>) -> usize {
    if !springs.contains(&SpringCondition::Unknown) {
        return if validate_springs(springs, contig_sizes) {
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
    springs[idx] = SpringCondition::Damaged;
    let damaged_count = recur(springs, contig_sizes);
    springs[idx] = SpringCondition::Operational;
    let op_count = recur(springs, contig_sizes);
    springs[idx] = SpringCondition::Unknown;

    return damaged_count + op_count;
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read the input file");
    let mut records = get_records(&input);
    let mut memo = HashMap::new();

    let sum: usize = records
        .iter_mut()
        .map(|r| {
            if let Some(count) = memo.get(&r) {
                *count
            } else {
                let count = recur(&mut r.springs, &r.contig_sizes);
                memo.insert(r, count);

                count
            }
        })
        .sum();

    println!("Sum: {}", sum);
}
