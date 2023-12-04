struct ScratchCard {
    winning_nums: Vec<u64>,
    elf_nums: Vec<u64>,
}

impl ScratchCard {
    fn get_worth(&self) -> u64 {
        let mut n_matches = 0;
        for num in self.elf_nums.iter() {
            if self.winning_nums.contains(&num) {
                n_matches += 1;
            }
        }

        if n_matches == 0 {
            0
        } else {
            2u64.pow(n_matches - 1)
        }
    }
}

fn get_scratchcards(input: &str) -> Vec<ScratchCard> {
    let mut cards = Vec::new();

    for line in input.lines() {
        let num_info = line.trim().split(':').last().unwrap();
        let winning_nums = num_info.split('|').next().unwrap();
        let elf_nums = num_info.split('|').last().unwrap();
        let winning_nums = winning_nums
            .split_whitespace()
            .into_iter()
            .filter_map(|slice| slice.trim().parse::<u64>().ok())
            .collect();
        let elf_nums = elf_nums
            .split_whitespace()
            .into_iter()
            .filter_map(|slice| slice.trim().parse::<u64>().ok())
            .collect();
        cards.push(ScratchCard {
            winning_nums,
            elf_nums,
        });
    }

    cards
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read input file");

    let cards = get_scratchcards(&input);

    let points: u64 = cards.iter().map(|card| card.get_worth()).sum();

    println!("Total points: {}", points);
}
