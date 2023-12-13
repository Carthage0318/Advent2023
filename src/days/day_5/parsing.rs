use crate::days::day_5::types::{Category, CategoryMap, MapRange};
use crate::AdventErr::InputParse;
use crate::{parser, AdventResult};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::Read;

pub fn parse_input(input_file: &mut File) -> AdventResult<(Vec<u64>, Vec<CategoryMap>)> {
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let Some((seeds_line, rest)) = input.split_once("\n\n") else {
        return Err(InputParse(String::from("Failed to extract seed line")));
    };

    let seeds = seeds_line
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|s| {
            s.parse()
                .map_err(|_| InputParse(format!("Failed to parse seed number '{s}'")))
        })
        .collect::<AdventResult<_>>()?;

    let mut maps = parser::as_vec_by_block_from_str(rest, "\n\n", block_parser)?;
    maps.sort_unstable_by_key(|map| map.source as u8);

    Ok((seeds, maps))
}

lazy_static! {
    static ref MAP_TYPE_REGEX: Regex =
        Regex::new(r"(?<source>\w+)-to-(?<destination>\w+)").unwrap();
}

fn block_parser(block: &str) -> AdventResult<CategoryMap> {
    let Some((header, ranges)) = block.split_once('\n') else {
        return Err(InputParse(format!(
            "Failed to separate header line - block:\n{block}"
        )));
    };

    let Some(caps) = MAP_TYPE_REGEX.captures(header) else {
        return Err(InputParse(format!(
            "Failed to find source/destination types in block header: {header}"
        )));
    };

    let source = &caps["source"];
    let Ok(source) = Category::try_from(source) else {
        return Err(InputParse(format!(
            "Failed to parse source '{source}' from block"
        )));
    };

    let destination = &caps["destination"];
    let Ok(destination) = Category::try_from(destination) else {
        return Err(InputParse(format!(
            "Failed to parse destination '{destination}' from block"
        )));
    };

    let mut ranges = parser::as_vec_by_line_from_str(ranges, line_range_parser)?;
    ranges.sort_unstable_by_key(|x| x.source_start);

    Ok(CategoryMap {
        source,
        destination,
        ranges,
    })
}

fn line_range_parser(line: &str) -> AdventResult<MapRange> {
    let mut iter = line.split_whitespace();

    let (Some(destination_start), Some(source_start), Some(length)) =
        (iter.next(), iter.next(), iter.next())
    else {
        return Err(InputParse(format!(
            "Failed to split range parts from line:\n{line}"
        )));
    };

    let (Ok(destination_start), Ok(source_start), Ok(length)) = (
        destination_start.parse(),
        source_start.parse(),
        length.parse(),
    ) else {
        return Err(InputParse(format!(
            "Failed to parse range values from line:\n{line}"
        )));
    };

    Ok(MapRange {
        source_start,
        destination_start,
        length,
    })
}
