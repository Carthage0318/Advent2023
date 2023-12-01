use crate::{parser, utils, AdventErr, AdventResult};
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let calibration_doc = parser::as_vec_by_line(&mut input_file, |s| Ok(s.to_string()))?;

    // Part 1
    utils::part_header(1);
    part_1(&calibration_doc)?;

    // Part 2
    utils::part_header(2);
    part_2(&calibration_doc)?;

    Ok(())
}

fn part_1(calibration_lines: &[String]) -> AdventResult<()> {
    let calibration_sum = calibration_lines
        .iter()
        .map(|line| calibration_value(line))
        .sum::<AdventResult<u32>>()?;

    println!("Sum of calibration values: {calibration_sum}");
    return Ok(());

    fn calibration_value(line: &str) -> AdventResult<u32> {
        let first_digit = line
            .chars()
            .filter_map(|c| char::to_digit(c, 10))
            .next()
            .ok_or_else(|| {
                AdventErr::InputParse(format!("Failed to find first digit in line:\n{line}"))
            })?;

        let last_digit = line
            .chars()
            .rev()
            .filter_map(|c| char::to_digit(c, 10))
            .next()
            .ok_or_else(|| {
                AdventErr::InputParse(format!("Failed to find last digit in line:\n{line}"))
            })?;

        Ok(10 * first_digit + last_digit)
    }
}

fn part_2(calibration_lines: &[String]) -> AdventResult<()> {
    fn calibration_value(line: &str) -> u32 {
        const DIGIT_STRINGS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        // First digit

        // Find actual digit
        let (mut earliest_index, mut first_digit) = line
            .chars()
            .enumerate()
            .filter_map(|(index, c)| char::to_digit(c, 10).map(|value| (index, value)))
            .next()
            .unwrap_or((usize::MAX, 0));

        // Check words
        for (value, digit_name) in DIGIT_STRINGS
            .iter()
            .enumerate()
            .map(|(index, name)| (index + 1, name))
        {
            if let Some(index) = line.find(digit_name) {
                if index < earliest_index {
                    earliest_index = index;
                    first_digit = value as u32;
                }
            }
        }

        // Last digit
        let char_count = line.chars().count();

        // Find actual digit
        let (mut latest_index, mut last_digit) = line
            .chars()
            .rev()
            .enumerate()
            .filter_map(|(index, c)| char::to_digit(c, 10).map(|value| (char_count - index - 1, value)))
            .next()
            .unwrap_or((0usize, 0));

        // Check words
        for (value, digit_name) in DIGIT_STRINGS
            .iter()
            .enumerate()
            .map(|(index, name)| (index + 1, name))
        {
            if let Some(index) = line.rfind(digit_name) {
                if index > latest_index {
                    latest_index = index;
                    last_digit = value as u32;
                }
            }
        }

        10 * first_digit + last_digit
    }

    let calibration_sum: u32 = calibration_lines
        .iter()
        .map(|line| calibration_value(line))
        .sum();

    println!("Sum of calibration values: {calibration_sum}");
    Ok(())
}
