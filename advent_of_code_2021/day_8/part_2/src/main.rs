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
                .map(|s| String::from(s))
                .collect(),
        );
    }

    return outputs;
}

fn common_with_mapping(input: &str, mapping: &[bool; 7]) -> u32 {
    return mapping.iter().enumerate().fold(0u32, |accum, (i, flag)| {
        accum
            + if *flag && input.contains((i as u8 + A_OFFSET) as char) {
                1
            } else {
                0
            }
    });
}

fn decode_line(line: &Vec<String>) -> u32 {
    let mut mappings: [[bool; 7]; 9] = [[false; 7]; 9];

    // get the mappings for the numbers with unique numbers of segments
    for (i, mapping) in line.iter().take(10).enumerate() {
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
            7 => {
                for segment in mapping.as_bytes() {
                    mappings[7][(segment - A_OFFSET) as usize] = true;
                }
            }
            8 => {
                for segment in mapping.as_bytes() {
                    mappings[8][(segment - A_OFFSET) as usize] = true;
                }
            }
            _ => {
                continue;
            }
        }
    }

    // figure out the rest
    for (i, mapping) in line.iter().take(10).enumerate() {
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
                    (_, _, _) => {}
                }
            }
        }
    }

    0
}

/*
 * - Can identify 1, 4, 7, and 8 because of their unique lengths
 * - Can identify the rest by the number of common segments with 1, 4, 7
 *      - Don't need to directly compute number in common with 8, as that's just
 *      the number of segments
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
    let entries = get_vals("test_input.txt");

    // - each entry contains the mappings for the numbers 0-9
    // - each sub-entry indicates whether a given letter is used
    //      - a, b, c, d, e, f, g
}
