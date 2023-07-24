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
            'A' => RPSMove::Rock,
            'B' => RPSMove::Paper,
            'C' => RPSMove::Scissors,
            _ => {
                panic!("Invalid character passed to RPSMove constructor");
            }
        }
    }
}

#[derive(Debug)]
enum RPSOutcome {
    Loss,
    Draw,
    Win
}

impl RPSOutcome {
    pub fn new(x: char) -> Self {
        match x {
            'X' => RPSOutcome::Loss,
            'Y' => RPSOutcome::Draw,
            'Z' => RPSOutcome::Win,
            _ => {
                panic!("Invalid character passed to RPSOutcome constructor.");
            }
        }
    }
    pub fn get_proper_move(&self, opp_move: &RPSMove) -> RPSMove {
        match *self {
            RPSOutcome::Loss => { match *opp_move {
                    RPSMove::Rock => RPSMove::Scissors,
                    RPSMove::Paper => RPSMove::Rock,
                    RPSMove::Scissors => RPSMove::Paper
                }
            },
            RPSOutcome::Draw => { match *opp_move {
                    RPSMove::Rock => RPSMove::Rock,
                    RPSMove::Paper => RPSMove::Paper,
                    RPSMove::Scissors => RPSMove::Scissors
                } 
            },
            RPSOutcome::Win => { match *opp_move {
                    RPSMove::Rock => RPSMove::Paper,
                    RPSMove::Paper => RPSMove::Scissors,
                    RPSMove::Scissors => RPSMove::Rock
                }
            }
        }
    }
}


fn total_score(opp_move: &RPSMove, my_move: &RPSMove) -> u32 {
    match *my_move {
        RPSMove::Rock => { 1 + match *opp_move {
                RPSMove::Rock => 3,
                RPSMove::Paper => 0,
                RPSMove::Scissors => 6
            }
        },
        RPSMove::Paper => { 2 + match *opp_move {                
                RPSMove::Rock => 6,
                RPSMove::Paper => 3,
                RPSMove::Scissors => 0
            }
        },
        RPSMove::Scissors => { 3 + match *opp_move {   
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
    let mut outcome: RPSOutcome;
    let mut score: u32 = 0;

    for line in input {
        opp_move = RPSMove::new(line.trim().chars().nth(0).unwrap());
        outcome = RPSOutcome::new(line.trim().chars().last().unwrap());
        my_move = outcome.get_proper_move(&opp_move);

        //println!("opp_move: {:?}, desired outcome: {:?}, my_move: {:?}",
        //         opp_move, outcome, my_move);

        score += total_score(&opp_move, &my_move);
    }

    println!("Total score: {}", score);
}
