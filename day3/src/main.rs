use std::str::FromStr;

use color_eyre::eyre::{Error, Result};

#[derive(Debug)]
struct Part {
    start: (usize, usize),
    length: usize,
    number: u32,
}

struct Schematic {
    grid: Vec<Vec<char>>,
    parts: Vec<Part>,
}

impl Schematic {
    fn get_valid_parts(&self) -> Vec<&Part> {
        self.parts
            .iter()
            .filter(|part| {
                // The number of rows to search is always 3 rows: above, actual, and below
                // however we need to do some clamping in case we are at the first or last row
                let (x, y) = part.start;
                let range_x = x.saturating_sub(1)..=(x + 1).clamp(0, self.grid.len() - 1);
                let rows = &self.grid[range_x];
                // then we iterate through the sub sections of each row around the number collecting all characters
                rows.iter()
                    .flat_map(|row| {
                        let range_y =
                            y.saturating_sub(1)..=(y + part.length).clamp(0, row.len() - 1);
                        &row[range_y]
                    })
                    // Finally we check if any of the characters are valid punctuation
                    .any(|char| char.is_ascii_punctuation() && *char != '.')
            })
            .collect()
    }
}

impl FromStr for Schematic {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Create a 2 dimensional grid of characters to represent the schematic grid
        let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let mut parts = Vec::new();
        // Loop twice to get x,y coordinates for each character in the grid
        for (x, row) in grid.iter().enumerate() {
            let mut start = (0, 0);
            let mut length = 0;
            let mut partial_number = Vec::new();
            for (y, char) in row.iter().enumerate() {
                // Using the rules given parse out part numbers regardless of validity
                match char {
                    d if d.is_ascii_digit() && partial_number.is_empty() => {
                        start = (x, y);
                        length = 1;
                        partial_number.push(*char);
                    }
                    d if d.is_ascii_digit() => {
                        length += 1;
                        partial_number.push(*char);
                    }
                    d if !d.is_ascii_digit() && !partial_number.is_empty() => {
                        parts.push(Part {
                            start,
                            length,
                            number: partial_number.iter().collect::<String>().parse()?,
                        });
                        partial_number.clear();
                    }
                    _ => (),
                }
            }
            // Handle numbers at the end of a line
            if !partial_number.is_empty() {
                parts.push(Part {
                    start,
                    length,
                    number: partial_number.iter().collect::<String>().parse()?,
                });
            }
        }
        Ok(Schematic { grid, parts })
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
    let schematic: Schematic = input.parse()?;
    let valid_parts = schematic.get_valid_parts();
    let valid_part_sum = valid_parts.iter().map(|part| part.number).sum();
    Ok(valid_part_sum)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use crate::calculate_part_1;
    #[test]
    fn calculate_part_1_test() -> Result<()> {
        let input = include_str!("test.txt");
        assert_eq!(4361, calculate_part_1(input)?);
        Ok(())
    }
}
