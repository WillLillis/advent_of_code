#[derive(Debug, Clone, Copy)]
struct RaceInfo {
    time: u32,
    record: u32,
}

impl RaceInfo {
    fn calc_dist(&self, t_hold: u32) -> u32 {
        if t_hold >= self.time {
            return 0;
        }

        t_hold * (self.time - t_hold)
    }

    fn opt_hold_time(&self) -> u32 {
        self.time / 2 // roundoff error hopefully not an issue?
    }

    fn n_record_breaks(&self) -> u32 {
        let mut n_breaks = 0;
        let optimal = self.opt_hold_time();

        for t_hold in (1..optimal).rev() {
            if self.calc_dist(t_hold) > self.record {
                n_breaks += 1;
            } else {
                break;
            }
        }

        for t_hold in optimal.. {
            if self.calc_dist(t_hold) > self.record {
                n_breaks += 1;
            } else {
                break;
            }
        }

        n_breaks
    }
}

fn get_race_info(input: &str) -> Vec<RaceInfo> {
    input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .into_iter()
        .zip(input.lines().last().unwrap().split_whitespace().into_iter())
        .filter_map(|(time_str, dist_str)| {
            match (time_str.parse::<u32>(), dist_str.parse::<u32>()) {
                (Ok(time), Ok(dist)) => Some((time, dist)),
                _ => None,
            }
        })
        .map(|(time, dist)| RaceInfo { time, record: dist })
        .collect()
}

// d = t_hold * (t - t_hold)
// d = t_hold * t - t_hold^2
// d' = t - 2 t_hold
// 0 = t - 2 t_hold
// Optimal: t_hold = t / 2

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read the input file");
    let races = get_race_info(&input);

    print!("{:?}", races);

    let val: u32 = races.iter().map(|race| race.n_record_breaks()).product();

    println!("Val: {:?}", val);
}
