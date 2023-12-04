use std::{collections::HashMap, str::FromStr};

use color_eyre::eyre::{eyre, Error, Result};

#[derive(Clone)]
struct Scratchcard {
    id: u8,
    numbers: Vec<u8>,
    winning_numbers: Vec<u8>,
}

impl Scratchcard {
    fn num_matches(&self) -> usize {
        self.numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count()
    }
}

impl FromStr for Scratchcard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Skip the word "Card" at the front of input and subsequent spaces
        let (id, numbers) = s
            .split_once(':')
            .ok_or(eyre!("input string did not have colon, value: {s}"))?;
        let id: u8 = id.trim_matches(|c: char| !c.is_ascii_digit()).parse()?;
        // Split numbers on "|"" symbol
        let (numbers, winning_numbers) = numbers
            .split_once('|')
            .ok_or(eyre!("input string did not have pipe, value: {numbers}"))?;
        // Note that the "split_ascii_whitespace" method is doing implicit trimming of excess
        // white space inside the number strings
        let numbers: Vec<u8> = numbers
            .trim()
            .split_ascii_whitespace()
            .map(|digits| digits.parse().map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;
        let winning_numbers: Vec<u8> = winning_numbers
            .trim()
            .split_ascii_whitespace()
            .map(|digits| digits.parse().map_err(Error::from))
            .collect::<Result<Vec<u8>>>()?;
        Ok(Scratchcard {
            id,
            numbers,
            winning_numbers,
        })
    }
}

fn main() -> Result<()> {
    color_eyre::install().unwrap();
    let input = include_str!("input.txt");
    let output = calculate_part_1(input)?;
    println!("Part 1 Answer: {output}");
    let output = calculate_part_2(input)?;
    println!("Part 2 Answer: {output}");
    Ok(())
}

fn calculate_part_1(input: &str) -> Result<usize> {
    let scratchcards: Vec<Scratchcard> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>>>()?;
    let score = scratchcards
        .iter()
        .map(|card| {
            card.numbers
                .iter()
                .filter(|number| card.winning_numbers.contains(number))
                .fold(0, |score, _| if score == 0 { 1 } else { score + score })
        })
        .sum();
    Ok(score)
}

fn calculate_part_2(input: &str) -> Result<u32> {
    // Parse our scratchcards
    let scratchcards: Vec<Scratchcard> = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>>>()?;
    // Create a hashmap to track card counts
    let mut counts: HashMap<u8, usize> = scratchcards.iter().map(|card| (card.id, 1)).collect();
    for card in scratchcards.iter() {
        let current_count = *counts.get(&card.id).ok_or(eyre!(
            "attempted to access a non-existant card id, value: {}",
            card.id
        ))?;
        let add_range = (card.id + 1) as usize..=card.id as usize + card.num_matches();
        for id in add_range {
            *counts.get_mut(&id.try_into()?).ok_or(eyre!(
                "attempted to access a non-existant card id, value: {id}"
            ))? += current_count;
        }
    }
    Ok(counts.values().map(|count| *count as u32).sum())
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use crate::{calculate_part_1, calculate_part_2};

    #[test]
    fn calculate_part_1_test() -> Result<()> {
        let input = include_str!("test.txt");
        assert_eq!(13, calculate_part_1(input)?);
        Ok(())
    }

    #[test]
    fn calculate_part_2_test() -> Result<()> {
        let input = include_str!("test.txt");
        assert_eq!(30, calculate_part_2(input)?);
        Ok(())
    }
}
