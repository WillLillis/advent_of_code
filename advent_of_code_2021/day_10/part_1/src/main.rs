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

fn main() {
    let subsystem = get_subsystem("input.txt");

    let error_score = subsystem.iter().fold(0, |score, line| {
        score
            + match is_corrupted(line) {
                Some(bad_char) => bad_char.syntax_error_score(),
                None => 0,
            }
    });

    println!("Total syntax error score: {error_score}");
}
