use std::fs;

fn get_output_vals(file_name: &str) -> Vec<Vec<String>> {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file");

    let mut outputs: Vec<Vec<String>> = Vec::new();

    for line in input.lines() {
        outputs.push(
            line.trim()
                .split_whitespace()
                .skip_while(|s| !s.contains('|'))
                .skip(1)
                .map(|s| String::from(s))
                .collect(),
        );
    }

    return outputs;
}

fn main() {
    let num_occurences = get_output_vals("input.txt")
        .iter()
        .flatten()
        .fold(0, |accum, s| {
            accum
                + if [2, 3, 4, 7].contains(&s.len()) {
                    1
                } else {
                    0
                }
        });

    println!("Num occurences: {num_occurences}");
}
