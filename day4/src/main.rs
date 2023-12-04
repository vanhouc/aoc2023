use std::str::FromStr;

use color_eyre::eyre::{eyre, Error, Result};

struct Scratchcard {
    id: u8,
    numbers: Vec<u8>,
    winning_numbers: Vec<u8>,
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
    Ok(())
}

fn calculate_part_1(input: &str) -> Result<u32> {
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

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use crate::calculate_part_1;

    #[test]
    fn calculate_part_1_test() -> Result<()> {
        let input = include_str!("test.txt");
        assert_eq!(13, calculate_part_1(input)?);
        Ok(())
    }
}
