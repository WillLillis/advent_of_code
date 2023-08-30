use std::fs;

const A_OFFSET: u8 = 'a' as u8;

fn get_vals(file_name: &str) -> Vec<Vec<String>> {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file");

    let mut outputs: Vec<Vec<String>> = Vec::new();

    for line in input.lines() {
        outputs.push(
            line.trim()
                .split_whitespace()
                .filter(|s| !s.contains('|'))
                .map(|s| String::from(s.trim()))
                .collect(),
        );
    }

    return outputs;
}

fn common_with_mapping(input: &str, mapping: &[bool; 7]) -> u32 {
    let common = mapping.iter().enumerate().fold(0u32, |accum, (i, flag)| {
        accum
            + if *flag && input.contains((i as u8 + A_OFFSET) as char) {
                1
            } else {
                0
            }
    });

    return common;
}

fn decode_segments(input: &str, mappings: &[[bool; 7]; 10]) -> u32 {
    for (i, mapping) in mappings.iter().enumerate() {
        if mapping.iter().enumerate().fold(true, |accum, (j, flag)| {
            accum
                && if *flag {
                    input.contains((A_OFFSET + j as u8) as char)
                } else {
                    !input.contains((A_OFFSET + j as u8) as char)
                }
        }) {
            return i as u32;
        }
    }

    panic!("Failed to decode the current segment!");
}

fn decode_line(line: &Vec<String>) -> u32 {
    let mut mappings: [[bool; 7]; 10] = [[false; 7]; 10];

    // get the mappings for the numbers with unique numbers of segments
    for mapping in line.iter().take(10) {
        match mapping.len() {
            2 => {
                for segment in mapping.as_bytes() {
                    mappings[1][(segment - A_OFFSET) as usize] = true;
                }
            }
            4 => {
                for segment in mapping.as_bytes() {
                    mappings[4][(segment - A_OFFSET) as usize] = true;
                }
            }
            3 => {
                for segment in mapping.as_bytes() {
                    mappings[7][(segment - A_OFFSET) as usize] = true;
                }
            }
            7 => {
                for segment in mapping.as_bytes() {
                    mappings[8][(segment - A_OFFSET) as usize] = true;
                }
            }
            _ => {
                continue;
            }
        }
    }

    // figure out the rest of the mappings
    for mapping in line.iter().take(10) {
        match mapping.len() {
            2 | 3 | 4 | 7 => {
                continue;
            }
            len => {
                match (
                    len,
                    common_with_mapping(mapping, &mappings[4]),
                    common_with_mapping(mapping, &mappings[7]),
                ) {
                    (6, 3, 3) => {
                        // 0
                        for segment in mapping.as_bytes() {
                            mappings[0][(segment - A_OFFSET) as usize] = true;
                        }
                    }
                    (5, 2, 2) => {
                        // 2
                        for segment in mapping.as_bytes() {
                            mappings[2][(segment - A_OFFSET) as usize] = true;
                        }
                    }
                    (5, 3, 3) => {
                        // 3
                        for segment in mapping.as_bytes() {
                            mappings[3][(segment - A_OFFSET) as usize] = true;
                        }
                    }
                    (5, 3, 2) => {
                        // 5
                        for segment in mapping.as_bytes() {
                            mappings[5][(segment - A_OFFSET) as usize] = true;
                        }
                    }
                    (6, 3, 2) => {
                        // 6
                        for segment in mapping.as_bytes() {
                            mappings[6][(segment - A_OFFSET) as usize] = true;
                        }
                    }
                    (6, 4, 3) => {
                        // 9
                        for segment in mapping.as_bytes() {
                            mappings[9][(segment - A_OFFSET) as usize] = true;
                        }
                    }
                    (_, _, _) => {
                        panic!("Failed to decipher mappings");
                    }
                }
            }
        }
    }

    let mut place: u32 = 1;
    return line.iter().skip(10).rev().fold(0, |accum, num| {
        let tmp = accum + place * decode_segments(num, &mappings);
        place *= 10;
        tmp
    });
}

/*
 * - Can identify 1, 4, 7, and 8 because of their unique lengths
 * - Can identify the rest by the number of common segments with 4 and 7
 *
 *  #|Size|Seg. common w/ 1|Seg. common w/ 4| Seg. common w/ 7|
 *  0|  6 |        2       |        3       |         3       |
 *  1|  2 |        2       |        2       |         2       |
 *  2|  5 |        1       |        5       |         2       |
 *  3|  5 |        2       |        3       |         3       |
 *  4|  4 |        2       |        4       |         2       |
 *  5|  5 |        1       |        3       |         2       |
 *  6|  6 |        1       |        3       |         2       |
 *  7|  3 |        2       |        2       |         3       |
 *  8|  7 |        2       |        4       |         3       |
 *  9|  6 |        2       |        4       |         3       |
 */
fn main() {
    let entries = get_vals("input.txt");

    let sum = entries
        .iter()
        .fold(0, |accum, entry| accum + decode_line(entry));

    println!("Sum: {sum}");
}
