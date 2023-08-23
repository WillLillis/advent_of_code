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
    let report = get_report("test_input.txt");

    let mut accum: Vec<i32> = vec![0; report.first().unwrap().len()];
    

    for line in report.iter() {
        for (&bit, count) in line.iter().zip(accum.iter_mut()) {
            *count += if bit { 1 } else { -1 };
        }
    }

    let mut place: u32 = 1;
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;

    for count in accum.iter().rev() {
        gamma += if *count > 0 { place }else { 0 }; // == 0 case? 
        epsilon += if *count > 0 { 0 } else { place };
        place *= 2;
    }

    println!("Gamma: {gamma}, Epsilon: {epsilon}");
}
