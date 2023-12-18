use crate::data_structures::Direction;
use crate::days::day_18::types::Instruction;
use crate::AdventErr::InputParse;
use crate::AdventResult;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref LINE_REGEX: Regex =
        Regex::new(r"(?<direction>[RUDL]) (?<length>\d+) \(#(?<color>[0-9a-fA-F]{6})\)").unwrap();
}

pub(super) fn line_parser(line: &str) -> AdventResult<(Instruction, Instruction)> {
    let Some(caps) = LINE_REGEX.captures(line) else {
        return Err(InputParse(format!("Unable to parse line:\n{line}")));
    };

    let direction = &caps["direction"];
    let direction = char_to_direction(direction.chars().next().unwrap())?;
    let length = (&caps["length"])
        .parse()
        .map_err(|_| InputParse(format!("Failed to parse length")))?;

    let basic_instruction = Instruction { direction, length };

    let color = &caps["color"];
    let mut chars = color.chars();
    let mut length = 0;
    for _ in 0..5 {
        length *= 16;
        length += chars.next().unwrap().to_digit(16).unwrap() as i64;
    }
    let direction = match chars.next().unwrap() {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        c => {
            return Err(InputParse(format!(
                "Unknown direction character '{c}' in color code"
            )))
        }
    };

    let color_instruction = Instruction { direction, length };

    Ok((basic_instruction, color_instruction))
}

fn char_to_direction(c: char) -> AdventResult<Direction> {
    Ok(match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => return Err(InputParse(format!("Unrecognized character '{c}'"))),
    })
}
