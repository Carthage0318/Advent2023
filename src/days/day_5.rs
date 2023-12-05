use crate::days::day_5::types::{Category, CategoryMap, SeedData};
use crate::AdventErr::Compute;
use crate::{utils, AdventResult};
use std::fs::File;
use std::ops::Range;

mod parsing;
mod types;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let (seed_numbers, category_maps) = parsing::parse_input(&mut input_file)?;

    // Part 1
    utils::part_header(1);
    part_1(&seed_numbers, &category_maps)?;

    // Part 2
    utils::part_header(2);
    part_2(&seed_numbers, &category_maps)?;

    Ok(())
}

fn part_1(seed_numbers: &[u64], category_maps: &[CategoryMap]) -> AdventResult<()> {
    let Some(lowest_location) = seed_numbers
        .iter()
        .map(|&seed_number| SeedData::build_for_seed(seed_number, &category_maps).location)
        .min()
    else {
        return Err(Compute(String::from("Empty seed list")));
    };

    println!("Lowest location: {lowest_location}");

    Ok(())
}

fn part_2(seed_numbers: &[u64], category_maps: &[CategoryMap]) -> AdventResult<()> {
    if seed_numbers.len() % 2 != 0 {
        return Err(Compute(String::from("Odd number of values on seeds line")));
    }

    let mut current_ranges: Vec<_> = seed_numbers
        .chunks_exact(2)
        .map(|chunk| {
            let start = chunk[0];
            let length = chunk[1];
            start..(start + length)
        })
        .collect();

    let mut current_category = Category::Seed;

    let mut merged_ranges = vec![];

    while current_category != Category::Location {
        current_ranges.sort_unstable_by_key(|range| range.start);

        merge_ranges_into(&current_ranges, &mut merged_ranges);
        current_ranges.clear();

        let current_map = &category_maps[current_category as usize];
        for range in &merged_ranges {
            current_map.map_range_into(range, &mut current_ranges);
        }
        merged_ranges.clear();

        current_category = current_map.destination;
    }

    let Some(lowest_location) = current_ranges.iter().map(|range| range.start).min() else {
        return Err(Compute(String::from("Finished with no ranges")));
    };

    println!("Lowest location: {lowest_location}");

    Ok(())
}

/// Performs a merge operation across the given ranges, which are assumed to be sorted.
/// Adds merged ranges to the provided output vector.
/// Note: This does not consider overlapping ranges - only those which meet end-to-end.
fn merge_ranges_into(ranges: &[Range<u64>], output: &mut Vec<Range<u64>>) {
    let mut i = 0;
    while i < ranges.len() {
        let mut current = ranges[i].clone();
        i += 1;

        while i < ranges.len() {
            let next = &ranges[i];
            if current.end == next.start {
                current.end = next.end;
                i += 1;
            } else {
                break;
            }
        }

        output.push(current);
    }
}
