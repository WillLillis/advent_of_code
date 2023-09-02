use std::fs;

#[derive(Copy, Clone, Debug)]
enum SubsystemChar {
    LParen,
    LBrack,
    LCurly,
    LCaret,
    RParen,
    RBrack,
    RCurly,
    RCaret,
}

impl SubsystemChar {
    fn new(input: char) -> Option<Self> {
        match input {
            '(' => Some(Self::LParen),
            '[' => Some(Self::LBrack),
            '{' => Some(Self::LCurly),
            '<' => Some(Self::LCaret),
            ')' => Some(Self::RParen),
            ']' => Some(Self::RBrack),
            '}' => Some(Self::RCurly),
            '>' => Some(Self::RCaret),
            _ => None,
        }
    }

    fn syntax_error_score(&self) -> u32 {
        match self {
            Self::RParen => 3,
            Self::RBrack => 57,
            Self::RCurly => 1197,
            Self::RCaret => 25137,
            _ => {
                panic!("Syntax error scores are only defined for 'right' characters");
            }
        }
    }

    fn completion_score(&self) -> u128 {
        match self {
            Self::RParen => 1,
            Self::RBrack => 2,
            Self::RCurly => 3,
            Self::RCaret => 4,
            _ => {
                panic!("Completion scores are only defined for 'right' characters");
            }
        }
    }

    fn is_left(&self) -> bool {
        return match self {
            SubsystemChar::LParen
            | SubsystemChar::LBrack
            | SubsystemChar::LCurly
            | SubsystemChar::LCaret => true,
            SubsystemChar::RParen
            | SubsystemChar::RBrack
            | SubsystemChar::RCurly
            | SubsystemChar::RCaret => false,
        };
    }

    fn is_right(&self) -> bool {
        return match self {
            SubsystemChar::RParen
            | SubsystemChar::RBrack
            | SubsystemChar::RCurly
            | SubsystemChar::RCaret => true,
            SubsystemChar::LParen
            | SubsystemChar::LBrack
            | SubsystemChar::LCurly
            | SubsystemChar::LCaret => false,
        };
    }
    fn matches(&self, other: Self) -> bool {
        let (left, right) = match (self.is_left(), other.is_right()) {
            (true, true) => (*self, other),
            (false, false) => (other, *self),
            _ => {
                return false;
            }
        };

        match (left, right) {
            (Self::LParen, Self::RParen)
            | (Self::LBrack, Self::RBrack)
            | (Self::LCurly, Self::RCurly)
            | (Self::LCaret, Self::RCaret) => true,
            _ => false,
        }
    }

    fn get_matching(&self) -> Self {
        match self {
            SubsystemChar::LParen => Self::RParen,
            SubsystemChar::LBrack => Self::RBrack,
            SubsystemChar::LCurly => Self::RCurly,
            SubsystemChar::LCaret => Self::RCaret,
            SubsystemChar::RParen => Self::LParen,
            SubsystemChar::RBrack => Self::LBrack,
            SubsystemChar::RCurly => Self::RCurly,
            SubsystemChar::RCaret => Self::RCaret,
        }
    }
}

fn get_subsystem(file_name: &str) -> Vec<Vec<SubsystemChar>> {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file");

    let mut subsystem: Vec<Vec<SubsystemChar>> = Vec::new();

    for line in input.lines() {
        subsystem.push(
            line.trim()
                .chars()
                .filter_map(|c| SubsystemChar::new(c))
                .collect(),
        );
    }

    return subsystem;
}

// returns None if the line is incomplete or Ok, Some(c) if it encountered c
// while expected another character
fn is_corrupted(line: &Vec<SubsystemChar>) -> Option<SubsystemChar> {
    let mut stk: Vec<SubsystemChar> = Vec::new();

    for &c in line.iter() {
        if c.is_left() {
            stk.push(c);
        } else if c.is_right() {
            match stk.pop() {
                Some(left_char) => {
                    if !c.matches(left_char) {
                        return Some(c);
                    }
                }
                None => {
                    return None;
                }
            }
        } else {
            unreachable!();
        }
    }

    None
}

// assumes the passed in line isn't corrupted
fn get_missing_chars(line: &Vec<SubsystemChar>) -> Vec<SubsystemChar> {
    let mut missing_chars: Vec<SubsystemChar> = Vec::new();
    let mut stk: Vec<SubsystemChar> = Vec::new();

    for &c in line.iter() {
        if c.is_left() {
            stk.push(c);
        } else if c.is_right() {
            stk.pop();
        } else {
            unreachable!();
        }
    }

    for extra in stk.iter().rev() {
        missing_chars.push(extra.get_matching());
    }

    return missing_chars;
}

fn get_completion_score(line: &Vec<SubsystemChar>) -> u128 {
    let mut score = 0u128;

    for c in line.iter() {
        score *= 5u128;
        score += c.completion_score();
    }

    return score;
}

fn main() {
    let subsystem = get_subsystem("input.txt");

    let incomplete: Vec<Vec<SubsystemChar>> = subsystem
        .iter()
        .filter(|line| match is_corrupted(line) {
            Some(_) => false,
            None => true,
        })
        .map(|line| line.clone())
        .collect();

    drop(subsystem);

    let mut missing_scores: Vec<u128> = Vec::new();
    for line in incomplete.iter() {
        missing_scores.push(get_completion_score(&get_missing_chars(line)));
    }

    missing_scores.sort();

    println!("Median completion score: {}", missing_scores[missing_scores.len() / 2usize]);
}
