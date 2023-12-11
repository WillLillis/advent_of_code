fn get_oasis_report(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

fn pred_next_val(vals: &Vec<i32>) -> i32 {
    // if all 0's, return 0
    if vals.iter().find(|x| **x != 0).is_none() {
        return 0;
    }

    let mut diffs: Vec<i32> = Vec::new();
    diffs.reserve(vals.len() - 1);

    for i in 0..vals.len() - 1 {
        diffs.push(vals[i + 1] - vals[i]);
    }

    let old_last = vals.last().unwrap();
    let next_diff = pred_next_val(&diffs);

    old_last + next_diff
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read the input file.");

    let report = get_oasis_report(&input);

    let sum: i32 = report.iter().map(|rpt| pred_next_val(&rpt)).sum();

    println!("Sum: {}", sum);
}
