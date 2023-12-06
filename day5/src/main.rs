use std::str::FromStr;

use color_eyre::eyre::{bail, eyre, Error, Result};

use itertools::Itertools;

struct RangePair {
    source: (u64, u64),
    destination: (u64, u64),
}

#[derive(Default)]
struct RangeMap {
    maps: Vec<RangePair>,
}

impl RangeMap {
    fn from_parts<'a>(parts: impl Iterator<Item = &'a str>) -> Result<Self> {
        parts
            .skip(1)
            .try_fold(RangeMap::default(), |mut acc, part| {
                let values: Vec<u64> = part
                    .split_ascii_whitespace()
                    .map(|ranges| ranges.parse().map_err(Error::from))
                    .collect::<Result<Vec<_>>>()?;
                if let Some((destination, source, length)) = values.into_iter().collect_tuple() {
                    acc.maps.push(RangePair {
                        destination: (destination, length),
                        source: (source, length),
                    });
                    Ok(acc)
                } else {
                    bail!("destination, source, length values not present in data, value {part}")
                }
            })
    }

    fn map_source(&self, source: u64) -> u64 {
        if let Some(pair) = self
            .maps
            .iter()
            .find(|pair| (pair.source.0..pair.source.0 + pair.source.1).contains(&source))
        {
            let index: u64 = source - pair.source.0;
            pair.destination.0 + index
        } else {
            source
        }
    }
}

struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<RangeMap>,
}

impl Almanac {
    fn get_lowest_seed(&self) -> Option<u64> {
        self.seeds
            .iter()
            .copied()
            .map(|seed| self.maps.iter().fold(seed, |acc, map| map.map_source(acc)))
            .min()
    }
}

impl FromStr for Almanac {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let seeds = lines.next().ok_or(eyre!("input missing seeds line"))?;
        let seeds: Vec<u64> = seeds
            .trim_start_matches("seeds: ")
            .split_ascii_whitespace()
            .map(|seed| seed.parse::<u64>().map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;
        // Skip the blank line after seeds then proceed to split our iter into chunks
        // that contain each set of map ranges then map those into RangeMaps
        lines.next();
        let mut maps = Vec::new();
        while lines.peek().is_some() {
            maps.push(RangeMap::from_parts(
                lines.peeking_take_while(|line| !line.is_empty()),
            )?);
            lines.next();
        }
        Ok(Almanac { seeds, maps })
    }
}

fn main() -> Result<()> {
    color_eyre::install().unwrap();
    let input = include_str!("input.txt");
    let output = calculate_part_1(input)?;
    println!("Part 1 Answer: {output}");
    Ok(())
}

fn calculate_part_1(input: &str) -> Result<u64> {
    let almanac: Almanac = input.parse()?;
    let lowest = almanac
        .get_lowest_seed()
        .ok_or(eyre!("error while getting lowest seed"))?;
    Ok(lowest)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use crate::calculate_part_1;
    #[test]
    fn calculate_part_1_test() -> Result<()> {
        let input = include_str!("test.txt");
        assert_eq!(35, calculate_part_1(input)?);
        Ok(())
    }
}
