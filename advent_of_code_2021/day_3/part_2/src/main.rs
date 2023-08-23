use std::fs;

fn get_report(file_name: &str) -> Vec<Vec<bool>> {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file");

    let mut report: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        let mut bits: Vec<bool> = Vec::new();
        for c in line.trim().chars() {
            bits.push( match c {
                '0' => false,
                '1' => true,
                _ => {
                    panic!("Parsing error!");
                }
            });
        }

        report.push(bits);
    }
    
    return report;
}

fn main() {
    let report: Vec<Vec<bool>> = get_report("input.txt");

    let mut accum: Vec<i32> = vec![0; report.first().unwrap().len()];
    

    for line in report.iter() {
        for (&bit, count) in line.iter().zip(accum.iter_mut()) {
            *count += if bit { 1 } else { -1 };
        }
    }
    
    let mut final_oxygen = None;
    {
        let mut prev_oxygen = report.clone();
        let num_digits = prev_oxygen.first().unwrap().len();

        for curr_bit in 0..num_digits {
            // find the bit criteria of the remaining lines
            let mut count = 0;
            for line in prev_oxygen.iter() {
                count += if line[curr_bit] { 1 } else { -1 }
            }
            let bit_criteria = if count >= 0 { true } else { false };
            // filter out the ones that don't match
            let next_oxygen: Vec<Vec<bool>> = prev_oxygen
                .iter()
                .filter_map(|line| if line[curr_bit] == bit_criteria { Some(line.clone()) } else { None } )
                .collect();

            // check if we have 1 left
            if next_oxygen.len() == 1 {
                final_oxygen = Some(next_oxygen[0].clone());
                break;
            } else {
                prev_oxygen = next_oxygen;
            }
        }
    }

    let mut final_c02 = None; 
    {
        let mut prev_c02 = report.clone();
        let num_digits = prev_c02.first().unwrap().len();

        for i in 0..num_digits {
            // find the bit criteria of the remaining lines
            let mut count = 0;
            for line in prev_c02.iter() {
                count += if line[i] { 1 } else { -1 }
            }
            let bit_criteria = if count >= 0 { false } else { true };

            // filter out the ones that don't match
            let next_c02: Vec<Vec<bool>> = prev_c02
                .iter()
                .filter_map(|line| if line[i] == bit_criteria { Some(line.clone()) } else { None } )
                .collect();
            // check if we have 1 left
            if next_c02.len() == 1 {
                final_c02 = Some(next_c02[0].clone());
                break;
            } else {
                prev_c02 = next_c02;
            }
        }
    }
    
    if final_oxygen == None || final_c02 == None {
        eprintln!("Failed to find the correct ratings");
        return;
    }

    let final_oxygen = final_oxygen.unwrap();
    let final_c02 = final_c02.unwrap();

    let mut place: u32 = 1;
    let mut oxygen: u32 = 0;
    let mut c02: u32 = 0;

    for (oxygen_flag, c02_flag) in final_oxygen.iter().rev().zip(final_c02.iter().rev()) {
        oxygen += if *oxygen_flag { place } else { 0 }; 
        c02 += if *c02_flag { place } else { 0 };
        place *= 2;
    }

    println!("Oxygen: {oxygen}, c02: {c02}");
    println!("Life support rating: {}", oxygen * c02);
}
