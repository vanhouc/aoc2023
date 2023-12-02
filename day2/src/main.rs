use std::str::FromStr;

use color_eyre::eyre::{bail, eyre, Error, Result};

#[derive(Default)]
struct CubeSet {
    red: u8,
    green: u8,
    blue: u8,
}

impl CubeSet {
    fn is_superset(&self, other: &CubeSet) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
}

impl FromStr for CubeSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cube_set: Result<CubeSet> =
            s.split(',')
                .try_fold(CubeSet::default(), |mut acc, color_count| {
                    let (count, color_name) =
                        color_count.trim().split_once(' ').ok_or_else(|| {
                            eyre!("cubeset string was not properly formatted, value: {color_count}")
                        })?;
                    let count: u8 = count.parse()?;
                    match color_name {
                        "red" => acc.red += count,
                        "green" => acc.green += count,
                        "blue" => acc.blue += count,
                        _ => bail!("color_name was not a valid value, name: {color_name}"),
                    }
                    Ok(acc)
                });
        cube_set
    }
}

struct Game {
    id: u8,
    sets: Vec<CubeSet>,
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, sets) = s
            .split_once(':')
            .ok_or(eyre!("game string did not contain a colon, value: {s}"))?;
        let id: u8 = id
            .split_once(' ')
            .ok_or(eyre!("game id is not valid, value: {id}"))?
            .1
            .parse()?;
        let sets = sets
            .split(';')
            .map(|set| set.parse())
            .collect::<Result<Vec<CubeSet>>>()?;
        Ok(Game { id, sets })
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
    let total = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games = input
        .lines()
        .map(|game| game.parse())
        .collect::<Result<Vec<Game>>>()?;
    let id_sum = games
        .into_iter()
        .filter(|g| g.sets.iter().all(|set| total.is_superset(set)))
        .fold(0u32, |acc, g| acc + g.id as u32);
    Ok(id_sum)
}

#[cfg(test)]
mod tests {
    use crate::calculate_part_1;
    use color_eyre::eyre::Result;

    #[test]
    fn calculate_part_1_test() -> Result<()> {
        let input = include_str!("test.txt");
        assert_eq!(8, calculate_part_1(input)?);
        Ok(())
    }
}
