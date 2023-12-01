use color_eyre::eyre::{bail, Result};

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let input = include_str!("input.txt");

    let output = input
        .lines()
        .map(calculate_calibration)
        .sum::<Result<u32>>()?;

    println!("Total Calibration Value: {output}");

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

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;

    use crate::calculate_calibration;

    #[test]
    fn get_digits_test() -> Result<()> {
        assert_eq!(12, calculate_calibration("1abc2")?);
        assert_eq!(38, calculate_calibration("pqr3stu8vwx")?);
        assert_eq!(15, calculate_calibration("a1b2c3d4e5f")?);
        assert_eq!(77, calculate_calibration("treb7uchet")?);
        Ok(())
    }
}
