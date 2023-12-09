use std::{collections::HashMap, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_char = match self {
            Card::Two => '2',
            Card::Three => '3',
            Card::Four => '4',
            Card::Five => '5',
            Card::Six => '6',
            Card::Seven => '7',
            Card::Eight => '8',
            Card::Nine => '9',
            Card::Ten => 'T',
            Card::Jack => 'J',
            Card::Queen => 'Q',
            Card::King => 'K',
            Card::Ace => 'A',
        };
        write!(f, "{}", display_char)?;
        Ok(())
    }
}

impl FromStr for Card {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
            "J" => Ok(Card::Jack),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            _ => Err(String::from("Failed to deserialize")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CardHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, Clone, Eq)]
struct CardHand {
    cards: Vec<Card>,
    bid: usize,
}

impl CardHand {
    fn get_hand_type(&self) -> CardHandType {
        let mut counts = HashMap::new();

        for card in self.cards.iter() {
            *counts.entry(card).or_insert(0) += 1;
        }

        let max_count = counts.values().max().unwrap();
        let min_count = counts.values().min().unwrap();

        match (max_count, min_count) {
            (5, 5) => CardHandType::FiveKind,
            (4, 1) => CardHandType::FourKind,
            (3, 2) => CardHandType::FullHouse,
            (3, 1) => CardHandType::ThreeKind,
            (2, 1) => {
                let n_twos: usize = counts.values().map(|c| if *c == 2 { 1 } else { 0 }).sum();
                if n_twos == 2 {
                    CardHandType::TwoPair
                } else {
                    CardHandType::OnePair
                }
            }
            _ => CardHandType::HighCard,
        }
    }
}

impl PartialEq for CardHand {
    fn eq(&self, other: &Self) -> bool {
        if self.get_hand_type() != other.get_hand_type() {
            return false;
        }

        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if self_card != other_card {
                return false;
            }
        }

        true
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for CardHand {
    fn lt(&self, other: &Self) -> bool {
        if self.cmp(other) == std::cmp::Ordering::Less {
            return true;
        }

        false
    }

    fn le(&self, other: &Self) -> bool {
        let cmp = self.cmp(other);
        if cmp == std::cmp::Ordering::Less || cmp == std::cmp::Ordering::Equal {
            return true;
        }

        false
    }

    fn gt(&self, other: &Self) -> bool {
        if self.cmp(other) == std::cmp::Ordering::Greater {
            return true;
        }

        false
    }

    fn ge(&self, other: &Self) -> bool {
        let cmp = self.cmp(other);
        if cmp == std::cmp::Ordering::Greater || cmp == std::cmp::Ordering::Equal {
            return true;
        }

        false
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.lt(other), self.gt(other)) {
            (true, false) => Some(std::cmp::Ordering::Less),
            (false, true) => Some(std::cmp::Ordering::Greater),
            _ => Some(std::cmp::Ordering::Equal),
        }
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_hand = self.get_hand_type();
        let other_hand = other.get_hand_type();

        if self_hand != other_hand {
            return self_hand.cmp(&other_hand);
        }

        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            if self_card != other_card {
                return self_card.cmp(other_card);
            }
        }

        return std::cmp::Ordering::Equal;
    }
}

fn get_cards(input: &str) -> Vec<CardHand> {
    let mut hands = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        assert!(parts.len() == 2);
        let cards: Vec<Card> = parts[0]
            .chars()
            .filter_map(|c| String::from(c).parse::<Card>().ok())
            .collect();

        let bid = parts[1].parse::<usize>().unwrap();

        hands.push(CardHand { cards, bid })
    }

    hands
}

fn main() {
    let input = std::fs::read_to_string("../input").expect("Failed to read the input file");

    let mut hands = get_cards(&input);

    hands.sort();

    let winnings: usize = hands
        .iter()
        .enumerate()
        .rev()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum();

    println!("Total winnings: {}", winnings);
}
