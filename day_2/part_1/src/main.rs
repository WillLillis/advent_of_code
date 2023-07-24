use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");
    let input = input.lines();

    let mut opp_move: char;
    let mut my_move: char;

    for line in input {
        opp_move = line.trim().chars().last().unwrap();
        my_move = line.trim().chars().nth(0).unwrap();

        println!("{opp_move} {my_move}");
    }
                
}
