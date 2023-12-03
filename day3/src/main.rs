use std::str::FromStr;

use color_eyre::eyre::{Error, Result};

#[derive(Debug)]
struct Part {
    start: (usize, usize),
    length: usize,
    number: u32,
}

impl Part {
    // Note that this grid will be outside of positive bounds of the grid
    fn get_coord_grid(&self) -> Vec<(usize, usize)> {
        let (x, y) = self.start;
        let mut coords = Vec::new();
        let range_x = x.saturating_sub(1)..=(x + 1);
        for x in range_x {
            let range_y = y.saturating_sub(1)..=(y + self.length);
            for y in range_y {
                coords.push((x, y));
            }
        }
        coords
    }
}

struct Schematic {
    grid: Vec<Vec<char>>,
    parts: Vec<Part>,
    gears: Vec<(usize, usize)>,
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
    fn get_gear_values(&self) -> Vec<u32> {
        self.gears
            .iter()
            .filter_map(|gear| {
                // Find all parts whose grid contains the gear
                let adjacent_parts: Vec<u32> = self
                    .parts
                    .iter()
                    .filter(|part| part.get_coord_grid().contains(gear))
                    .map(|part| part.number)
                    .collect();
                // Only include gears that have more than one adjacent part
                if adjacent_parts.len() > 1 {
                    Some(adjacent_parts.iter().product::<u32>())
                } else {
                    None
                }
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
        let mut gears = Vec::new();
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
                    d if d.is_ascii_punctuation() => {
                        // Save out number if partial is being tracked
                        if !partial_number.is_empty() {
                            parts.push(Part {
                                start,
                                length,
                                number: partial_number.iter().collect::<String>().parse()?,
                            });
                            partial_number.clear();
                        }
                        if *d == '*' {
                            gears.push((x, y));
                        }
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
        Ok(Schematic { grid, parts, gears })
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

fn calculate_part_1(input: &str) -> Result<u32> {
    let schematic: Schematic = input.parse()?;
    let valid_parts = schematic.get_valid_parts();
    let valid_part_sum = valid_parts.iter().map(|part| part.number).sum();
    Ok(valid_part_sum)
}

fn calculate_part_2(input: &str) -> Result<u32> {
    let schematic: Schematic = input.parse()?;
    let gear_score = schematic.get_gear_values();
    Ok(gear_score.iter().sum())
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use crate::{calculate_part_1, calculate_part_2};
    #[test]
    fn calculate_part_1_test() -> Result<()> {
        let input = include_str!("test.txt");
        assert_eq!(4361, calculate_part_1(input)?);
        Ok(())
    }
    #[test]
    fn calculate_part_2_test() -> Result<()> {
        let input = include_str!("test.txt");
        assert_eq!(467835, calculate_part_2(input)?);
        Ok(())
    }
}
