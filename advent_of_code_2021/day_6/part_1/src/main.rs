use std::fs;

#[derive(Debug)]
struct LanternFishPop {
    pop: [u32; 9]
}

impl LanternFishPop {
    fn new() -> Self {
        LanternFishPop {
            pop: [0; 9]
        }
    }

    fn total_pop(&self) -> u32 {
        return self.pop.iter().sum();
    }

    fn sim_day(&mut self) {
        let to_6 = self.pop[0]; 
        let to_8 = to_6;

        for i in 1..=8 {
            self.pop[i - 1] = self.pop[i];
        }

        self.pop[6] += to_6;
        self.pop[8] = to_8;
    }
}


fn get_fish(file_name: &str) -> LanternFishPop {
    let input = fs::read_to_string(file_name).expect("Failed to read the input file");

    let fishes: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    let mut pop = LanternFishPop::new();

    for &fish in fishes.iter() {
        pop.pop[fish] += 1;
    }
    
    return pop;
}

fn main() {
    let mut fish_pop = get_fish("input.txt");

    for _ in 0..80 {
        fish_pop.sim_day();
    }

    println!("Fish population after 80 days: {}", fish_pop.total_pop());
}
