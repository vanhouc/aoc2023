use std::str::FromStr;

use color_eyre::eyre::{eyre, Error, Result};

struct Race {
    time: u64,
    best_distance: u64,
}

impl Race {
    fn get_winning_speeds(&self) -> Vec<u64> {
        (0..self.time)
            .map(|speed| (self.time - speed) * speed)
            .filter(|distance| *distance > self.best_distance)
            .collect()
    }
}

// This impl is intended to parse a leaderboard as one mega race for part 2
impl FromStr for Race {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (time_input, distance_input) = s
            .split_once('\n')
            .ok_or(eyre!("input did not contain two lines, value: {s}"))?;
        let time = time_input
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()?;
        let best_distance = distance_input
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()?;
        Ok(Race {
            time,
            best_distance,
        })
    }
}

struct Leaderboard {
    races: Vec<Race>,
}

impl FromStr for Leaderboard {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (time_input, distance_input) = s
            .split_once('\n')
            .ok_or(eyre!("input did not contain two lines, value: {s}"))?;
        let times: Vec<u64> = time_input
            .trim_start_matches("Time:")
            .split_ascii_whitespace()
            .map(|digits| digits.parse().map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;
        let distances: Vec<u64> = distance_input
            .trim_start_matches("Distance:")
            .split_ascii_whitespace()
            .map(|digits| digits.parse().map_err(Error::from))
            .collect::<Result<Vec<_>>>()?;
        let races: Vec<Race> = times
            .into_iter()
            .zip(distances)
            .map(|(time, best_distance)| Race {
                time,
                best_distance,
            })
            .collect();
        Ok(Leaderboard { races })
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
    let leaderboard: Leaderboard = input.parse()?;
    let answer = leaderboard
        .races
        .iter()
        .map(|race| race.get_winning_speeds().len())
        .product();
    Ok(answer)
}

fn calculate_part_2(input: &str) -> Result<usize> {
    let race: Race = input.parse()?;
    let answer = race.get_winning_speeds().len();
    Ok(answer)
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use crate::{calculate_part_1, calculate_part_2};

    #[test]
    fn calculate_part_1_test() -> Result<()> {
        color_eyre::install().unwrap();
        let input = include_str!("test.txt");
        assert_eq!(288, calculate_part_1(input)?);
        Ok(())
    }
    #[test]
    fn calculate_part_2_test() -> Result<()> {
        color_eyre::install().unwrap();
        let input = include_str!("test.txt");
        assert_eq!(71503, calculate_part_2(input)?);
        Ok(())
    }
}
