use crate::{parser, utils, AdventErr, AdventResult};
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let calibration_doc = parser::as_vec_by_line(&mut input_file, |s| Ok(s.to_string()))?;

    // Part 1
    utils::part_header(1);
    part_1(&calibration_doc)?;

    Ok(())
}

fn part_1(calibration_lines: &[String]) -> AdventResult<()> {
    let calibration_sum = calibration_lines
        .iter()
        .map(|line| calibration_value(line))
        .sum::<AdventResult<u32>>()?;

    println!("Sum of calibration values: {calibration_sum}");
    Ok(())
}

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
