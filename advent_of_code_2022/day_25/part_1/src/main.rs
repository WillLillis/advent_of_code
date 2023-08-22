use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

const LOG_10_5: f32 = 0.69897000433; //5.0_f32.log10();

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

    fn build(s: String) -> Self {
        SNAFU {
            num: s
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

fn snafu_to_decimal(snafu_num: SNAFU) -> Result<i64, ParseSNAFUError> {
    let mut dec_num: i64 = 0;
    let mut mult: i64 = 1;

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

fn to_snafu_recur(num: i64, place: u32, cache: &mut HashMap<(i64, u32), (bool, String)>) -> (bool, String) {
    if let Some((success, tmp_str)) = cache.get(&(num, place)) {
        return (*success, tmp_str.clone());
    }
    if place == 0 {
        match num {
            2 => {
                cache.insert((num, place), (true, String::from("2")));
                return (true, String::from("2")); 
            },
            1 => { 
                cache.insert((num, place), (true, String::from("1")));
                return (true, String::from("1"));
            },
            0 => { 
                cache.insert((num, place), (true, String::from("0")));
                return (true, String::from("0")); 
            },
            -1 => { 
                cache.insert((num, place), (true, String::from("-")));
                return (true, String::from("-"));
            },
            -2 => { 
                cache.insert((num, place), (true, String::from("=")));
                return (true, String::from("="));
            },
            _ => { 
                cache.insert((num, place), (true, String::from("")));
                return (false, String::from("")); 
            }
        }
    }
    if let (true, p2_str) = to_snafu_recur(num - 2 * 5_u64.pow(place) as i64, place - 1, cache) {
        let ret_str = String::from("2") + &p2_str;
        cache.insert((num, place), (true, ret_str.clone()));
        return (true, ret_str);
    } else if let (true, p1_str) = to_snafu_recur(num - 5_u64.pow(place) as i64, place - 1, cache) {
        let ret_str = String::from("1") + &p1_str;
        cache.insert((num, place), (true, ret_str.clone()));
        return (true, ret_str);
    } else if let (true, p0_str)= to_snafu_recur(num, place - 1, cache) {
        let ret_str = String::from("0") + &p0_str;
        cache.insert((num, place), (true, ret_str.clone()));
        return (true, ret_str);
    } else if let (true, m1_str)= to_snafu_recur(num + 5_u64.pow(place) as i64, place - 1, cache) {
        let ret_str = String::from("-") + &m1_str;
        cache.insert((num, place), (true, ret_str.clone()));
        return (true, ret_str);
    } else if let (true, m2_str)= to_snafu_recur(num + 2 * 5_u64.pow(place) as i64, place - 1, cache) {
        let ret_str = String::from("=") + &m2_str;
        cache.insert((num, place), (true, ret_str.clone()));
        return (true, ret_str);
    } else {
        cache.insert((num, place), (true, String::from("")));
        return (false, String::from(""));
    }
}

fn to_snafu(dec_num: u64) -> Option<SNAFU> {
    let mut snafu_num = SNAFU::new();
    // - First find the highest place we need to represent the number at hand
    //      - Change of base formula: Log_5(x) = Log_10(x) / Log_10(5) 
    let highest_place = f32::floor((dec_num as f32).log10() / LOG_10_5) as u32;

    let mut cache: HashMap<(i64, u32), (bool, String)> = HashMap::new();
    let (success, snafu_str) = to_snafu_recur(dec_num as i64, highest_place + 1, &mut cache);

    if success {
        snafu_num.num = String::from(snafu_str.strip_prefix('0').unwrap());
        return Some(snafu_num);
    } else {
        return None;
    }
}

// - Originally wrote the above recursive solution without thinking
// - It was correct for the example input but much too slow for the full input
// - Found this helpful video explaining the conversion algorithm, which I wasn't able to figure
// out on my own (https://www.youtube.com/watch?v=MdLubTzLjIw&ab_channel=hyper-neutrino)
fn to_snafu2(mut dec_num: u64) -> SNAFU {
    let mut snafu_num = String::new();

    while dec_num > 0 {
        let rem = dec_num % 5;
        dec_num /= 5;
        
        if rem <= 2 {
            snafu_num = rem.to_string() + &snafu_num; // can proceed as normal to convert to base 5
        } else { // otherwise we have to compensate on the next digit
            dec_num += 1; // add one more to the "/= 5" result to represent the carry over
            if rem == 3 { // off by 2 from a "full 5" -> minus 2 character
                snafu_num = String::from("=") + &snafu_num;
            } else if rem == 4 { // off by 1 from a "full 5" -> minus 1 character
                snafu_num = String::from("-") + &snafu_num;
            }
        }
    }

    return SNAFU::build(snafu_num);
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
    let nums = get_nums("input.txt");

    let dec_sum = nums
        .iter()
        .map(|s| s.parse::<SNAFU>().unwrap())
        .fold(0, |accum, x| accum + snafu_to_decimal(x).unwrap());
    
    println!("Decimal sum: {dec_sum}");

    // To slow! Caching takes up too much memory...
    //let snafu_num = to_snafu(dec_sum as u64).unwrap();
    //println!("SNAFU sum: {}", snafu_num.num);
    
    let snafu_num = to_snafu2(dec_sum as u64);
    println!("SNAFU sum: {}", snafu_num.num);
}
