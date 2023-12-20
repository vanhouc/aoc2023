use std::str::FromStr;

use color_eyre::eyre::{bail, eyre, Error, Result};

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl FromStr for HandType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut card_counts: Vec<usize> = CARDS
            .iter()
            .map(|card| s.chars().filter(|c| c == card).count())
            .collect();
        card_counts.sort();
        card_counts.reverse();
        let highest = card_counts[0];
        if highest < 1 {
            bail!("hand contains no valid cards");
        }
        let second_highest = card_counts.get(1);
        let hand_type = match (highest, second_highest) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FourOfAKind,
            (3, Some(second)) if *second > 1 => HandType::FullHouse,
            (3, _) => HandType::ThreeOfAKind,
            (2, Some(second)) if *second > 1 => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            (_, _) => HandType::HighCard,
        };
        Ok(hand_type)
    }
}

fn calculate_part_1(input: &str) -> usize {
    let game: Vec<(&str, u32)> = input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(hand, count)| (hand, count.parse::<u32>()))
                .ok_or(eyre!(
                    "line did not contain both a hand and bet, value: {line}"
                ))
        })
        .collect();
}

fn main() -> Result<()> {
    color_eyre::install().unwrap();
    println!("Hello, world!");
    Ok(())
}
