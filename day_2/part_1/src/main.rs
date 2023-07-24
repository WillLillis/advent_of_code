use std::fs;

#[derive(Debug)]
enum RPSMove {
    Rock,
    Paper,
    Scissors
}

impl RPSMove {
    pub fn new(x: char) -> Self {
        match x {
            'A'|'X' => RPSMove::Rock,
            'B'|'Y' => RPSMove::Paper,
            'C'|'Z' => RPSMove::Scissors,
            _ => {
                panic!("Invalid character passed to constructor");
            }
        }
    }
}


fn total_score(opp_move: RPSMove, my_move: RPSMove) -> u32 {
    match my_move {
        RPSMove::Rock => { 1 + match opp_move {
                RPSMove::Rock => 3,
                RPSMove::Paper => 0,
                RPSMove::Scissors => 6
            }
        },
        RPSMove::Paper => { 2 + match opp_move {                
                RPSMove::Rock => 6,
                RPSMove::Paper => 3,
                RPSMove::Scissors => 0
            }
        },
        RPSMove::Scissors => { 3 + match opp_move {   
                RPSMove::Rock => 0,
                RPSMove::Paper => 6,
                RPSMove::Scissors => 3
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read the file");
    let input = input.lines();

    let mut opp_move: RPSMove;
    let mut my_move: RPSMove;
    let mut score: u32 = 0;

    for line in input {
        my_move = RPSMove::new(line.trim().chars().last().unwrap());
        opp_move = RPSMove::new(line.trim().chars().nth(0).unwrap());

        score += total_score(opp_move, my_move);

        //println!("opp_move: {:?}, my_move: {:?}", opp_move, my_move);
        //println!("Score for this matchup: {}",
        //         total_score(opp_move, my_move));
    }

    println!("Total score: {}", score);
}
