struct ScratchCard {
    winning_nums: Vec<u64>,
    elf_nums: Vec<u64>,
}

impl ScratchCard {
    fn get_matches(&self) -> u32 {
        let mut n_matches = 0;
        for num in self.elf_nums.iter() {
            if self.winning_nums.contains(&num) {
                n_matches += 1;
            }
        }

        n_matches
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

fn update_counts(card_counts: &mut Vec<u32>, card_worth: &Vec<u32>) {
    for i in 0..card_counts.len() {
        if card_counts[i] > 0 {
            for j in 1..=card_worth[i] as usize {
                println!("Card {} wins {}", i + 1, card_worth[i]);
                card_counts[i + j] += 1 * card_counts[i];
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read input file");

    let cards = get_scratchcards(&input);
    let card_worth: Vec<u32> = cards.iter().map(|card| card.get_matches()).collect();

    let mut card_counts: Vec<u32> = vec![1; cards.len()];

    update_counts(&mut card_counts, &card_worth);

    let n_cards: u32 = card_counts.iter().sum();

    println!("Total cards: {}", n_cards);
}
