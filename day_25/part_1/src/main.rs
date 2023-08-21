use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct SNAFU {
    num: String
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSNAFUError;

impl SNAFU {
    fn new() -> Self {
        SNAFU {
            num: String::new()
        }
    }
}

impl FromStr for SNAFU {
    type Err = ParseSNAFUError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for c in s.chars() { // should there be a .trim() in there or do we just let it error?
            match c {
                '2'|'1'|'0'|'-'|'=' => {},
                _ => { return Err(ParseSNAFUError); }
            }
        }
        return Ok(SNAFU { num: String::from(s) });
    }
}

fn snafu_to_decimal(snafu_num: SNAFU) -> Result<i32, ParseSNAFUError> {
    let mut dec_num: i32 = 0;
    let mut mult: i32 = 1;

    for c in snafu_num.num.chars().rev() {
        dec_num += mult * match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => {
                return Err(ParseSNAFUError);
            }
        };

        mult *= 5;
    }

    return Ok(dec_num);
}

// TODO: implement this
fn to_snafu(dec_num: i32) -> SNAFU {
    let snafu_num = SNAFU::new();


    return snafu_num;
}


fn get_nums(file_name: &str) -> Vec<String> {
    let input = fs::read_to_string(file_name)
        .expect("Failed to read the input file");

    let mut nums: Vec<String> = Vec::new();

    for line in input.lines() {
        nums.push(String::from(line));
    }

    return nums;
}

// convert numbers in file to decimal
// add together
// convert back to SNAFU system
fn main() {
    let nums = get_nums("test_input.txt");

    let dec_sum = nums
        .iter()
        .map(|s| s.parse::<SNAFU>().unwrap())
        .fold(0, |accum, x| accum + snafu_to_decimal(x).unwrap());
    
    println!("Decimal sum: {dec_sum}");

    let snafu_num = to_snafu(dec_sum);

    println!("SNAFU sum: {}", snafu_num.num);
}
