use color_eyre::eyre::Result;
use once_cell::sync::Lazy;

static ENGLISH_DIGITS: Lazy<Vec<(&str, char)>> = Lazy::new(|| {
    vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]
});

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let input = include_str!("input.txt");

    let output = input
        .lines()
        .map(calculate_calibration)
        .sum::<Result<u32>>()?;

    println!("Part 1 Answer: {output}");

    let output: Option<u32> = input.lines().map(parse_calibration).sum();
    println!("Part 2 Answer: {}", output.unwrap());

    Ok(())
}

fn calculate_calibration(line: &str) -> color_eyre::Result<u32> {
    // Fetch each ascii digit from the input line
    let mut digits: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();
    // Per the example input, if the line only contains one digit, that digit is used twice
    if digits.len() < 2 {
        digits.push(*digits.first().unwrap());
    }
    // Drain inner elements of digits
    digits.drain(1..digits.len() - 1);
    // Create a string out of the first and last digits and parse it into a u32
    let calibration: u32 = digits.into_iter().collect::<String>().parse()?;
    Ok(calibration)
}

fn parse_calibration(line: &str) -> Option<u32> {
    let digits: String = [
        find_first_digit(line.chars())?,
        find_last_digit(line.chars().rev())?,
    ]
    .into_iter()
    .collect();
    digits.parse().ok()
}

fn find_first_digit(line: impl Iterator<Item = char>) -> Option<char> {
    let mut acc = String::new();
    for c in line {
        if c.is_ascii_digit() {
            return Some(c);
        }
        acc.push(c);
        for (name, digit) in &*ENGLISH_DIGITS {
            if acc.contains(name) {
                return Some(*digit);
            }
        }
    }
    None
}

fn find_last_digit(line: impl Iterator<Item = char>) -> Option<char> {
    let mut acc = String::new();
    for c in line {
        if c.is_ascii_digit() {
            return Some(c);
        }
        acc.insert(0, c);
        for (name, digit) in &*ENGLISH_DIGITS {
            if acc.contains(name) {
                return Some(*digit);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use crate::{calculate_calibration, parse_calibration};

    #[test]
    fn get_digits_test() -> Result<()> {
        assert_eq!(12, calculate_calibration("1abc2")?);
        assert_eq!(38, calculate_calibration("pqr3stu8vwx")?);
        assert_eq!(15, calculate_calibration("a1b2c3d4e5f")?);
        assert_eq!(77, calculate_calibration("treb7uchet")?);
        Ok(())
    }

    #[test]
    fn word_to_digit_test() -> Result<()> {
        assert_eq!(Some(29), parse_calibration("two1nine"));
        assert_eq!(Some(83), parse_calibration("eightwothree"));
        assert_eq!(Some(13), parse_calibration("abcone2threexyz"));
        assert_eq!(Some(24), parse_calibration("xtwone3four"));
        assert_eq!(Some(42), parse_calibration("4nineeightseven2"));
        assert_eq!(Some(14), parse_calibration("zoneight234"));
        assert_eq!(Some(76), parse_calibration("7pqrstsixteen"));
        Ok(())
    }
}
