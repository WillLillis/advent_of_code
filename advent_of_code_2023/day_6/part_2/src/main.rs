#[derive(Debug, Clone, Copy)]
struct RaceInfo {
    time: usize,
    record: usize,
}

impl RaceInfo {
    fn calc_dist(&self, t_hold: usize) -> usize {
        if t_hold >= self.time {
            return 0;
        }

        t_hold * (self.time - t_hold)
    }

    fn opt_hold_time(&self) -> usize {
        self.time / 2 // roundoff error hopefully not an issue?
    }

    fn n_record_breaks(&self) -> usize {
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

fn get_race_info(input: &str) -> RaceInfo {
    let time_str = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .map(|time_str| time_str.replace(" ", ""))
        .collect::<String>();

    let time = time_str.parse::<usize>().unwrap();

    let record_str = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .map(|record_str| record_str.replace(" ", ""))
        .collect::<String>();

    let record = record_str.parse::<usize>().unwrap();

    RaceInfo { time, record }
}

// d = t_hold * (t - t_hold)
// d = t_hold * t - t_hold^2
// d' = t - 2 t_hold
// 0 = t - 2 t_hold
// Optimal: t_hold = t / 2

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read the input file");
    let race = get_race_info(&input);

    print!("{:?}", race);

    let val: usize = race.n_record_breaks();

    println!("Val: {:?}", val);
}
