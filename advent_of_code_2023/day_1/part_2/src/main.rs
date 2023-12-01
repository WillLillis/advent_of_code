static ONE: &str = "one";
static TWO: &str = "two";
static THREE: &str = "three";
static FOUR: &str = "four";
static FIVE: &str = "five";
static SIX: &str = "six";
static SEVEN: &str = "seven";
static EIGHT: &str = "eight";
static NINE: &str = "nine";

static WORDS_TO_NUMS: &'static [&'static str] =
    &[ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

fn get_calib(line: &str) -> Option<u32> {
    let first_digit = line.chars().enumerate().find(|(_, c)| c.is_digit(10));

    let last_digit = line
        .chars()
        .rev()
        .enumerate()
        .find(|(_, c)| c.is_digit(10))
        .map(|(i, c)| (line.len() - i - 1, c));

    let mut first_word_idx = usize::MAX;
    let mut first_word_val = None;
    let mut last_word_idx = usize::MIN;
    let mut last_word_val = None;

    for (i, word) in WORDS_TO_NUMS.iter().enumerate() {
        if let Some(idx) = line.find(word) {
            if idx < first_word_idx {
                first_word_idx = idx;
                first_word_val = Some(i + 1);
            }
        }
        if let Some(idx) = line.rfind(word) {
            if idx > last_word_idx {
                last_word_idx = idx;
                last_word_val = Some(i + 1);
            }
        }
    }

    let digit_1 = match (first_digit, first_word_val) {
        // Both have matches
        (Some((dig_idx, dig)), Some(word)) => {
            if dig_idx < first_word_idx {
                dig.to_digit(10).unwrap()
            } else {
                word as u32
            }
        }
        // word but no number
        (Some((_, dig)), None) => dig.to_digit(10).unwrap(),
        // number but no word
        (None, Some(word)) => word as u32,
        // nothing
        (None, None) => {
            panic!("Bad puzzle input!");
        }
    };

    let digit_2 = match (last_digit, last_word_val) {
        // Both have matches
        (Some((dig_idx, dig)), Some(word)) => {
            if dig_idx > last_word_idx {
                dig.to_digit(10).unwrap()
            } else {
                word as u32
            }
        }
        // word but no number
        (Some((_, dig)), None) => dig.to_digit(10).unwrap(),
        // number but no word
        (None, Some(word)) => word as u32,
        // nothing
        (None, None) => {
            return None;
        }
    };

    Some(10 * digit_1 + digit_2)
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read input file");

    let sum: u32 = input.lines().map(|line| get_calib(line).unwrap()).sum();

    println!("Calibration sum: {sum}");
}
