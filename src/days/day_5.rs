use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;
use regex::Regex;
use crate::AdventErr::{Compute, InputParse};
use crate::{AdventResult, parser, utils};

pub fn run(mut input_file: File) -> AdventResult<()> {
    let (seed_numbers, category_maps) = parse_input(&mut input_file)?;

    let seeds: Vec<_> = seed_numbers
        .iter()
        .map(|&seed_number| SeedData::build_for_seed(seed_number, &category_maps))
        .collect();

    // Part 1
    utils::part_header(1);
    part_1(&seeds)?;

    Ok(())
}

fn part_1(seeds: &[SeedData]) -> AdventResult<()> {
    let Some(lowest_location) = seeds
        .iter()
        .map(|seed| seed.location)
        .min()
        else {
            return Err(Compute(String::from("Empty seed list")));
        };

    println!("Lowest location: {lowest_location}");

    Ok(())
}

struct SeedData {
    seed: u64,
    soil: u64,
    fertilizer: u64,
    water: u64,
    light: u64,
    temperature: u64,
    humidity: u64,
    location: u64,
}

impl SeedData {
    fn empty() -> Self {
        Self {
            seed: 0,
            soil: 0,
            fertilizer: 0,
            water: 0,
            light: 0,
            temperature: 0,
            humidity: 0,
            location: 0
        }
    }

    fn get_prop_mut(&mut self, category: Category) -> &mut u64 {
        match category {
            Category::Seed => &mut self.seed,
            Category::Soil => &mut self.soil,
            Category::Fertilizer => &mut self.fertilizer,
            Category::Water => &mut self.water,
            Category::Light => &mut self.light,
            Category::Temperature => &mut self.temperature,
            Category::Humidity => &mut self.humidity,
            Category::Location => &mut self.location,
        }
    }

    fn build(initial_data: u64, initial_category: Category, category_maps: &[CategoryMap]) -> Self {
        fn fill_props(seed_data: &mut SeedData, category: Category, value: u64, category_maps: &[CategoryMap]) {
            *seed_data.get_prop_mut(category) = value;

            let Some(category_map) = category_maps.get(category as usize) else {
                return;
            };

            let destination_category = category_map.destination;
            let destination_value = category_map.map_value(value);

            fill_props(seed_data, destination_category, destination_value, category_maps)
        }

        let mut result = Self::empty();
        fill_props(&mut result, initial_category, initial_data, category_maps);

        result
    }

    fn build_for_seed(seed: u64, category_maps: &[CategoryMap]) -> Self {
        Self::build(seed, Category::Seed, category_maps)
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl TryFrom<&str> for Category {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "seed" => Ok(Category::Seed),
            "soil" => Ok(Category::Soil),
            "fertilizer" => Ok(Category::Fertilizer),
            "water" => Ok(Category::Water),
            "light" => Ok(Category::Light),
            "temperature" => Ok(Category::Temperature),
            "humidity" => Ok(Category::Humidity),
            "location" => Ok(Category::Location),
            _ => Err(())
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct MapRange {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

impl MapRange {
    /// Exclusive end of source range
    fn source_end(&self) -> u64 {
        self.source_start + self.length
    }

    fn map_value_unchecked(&self, source_value: u64) -> u64 {
        let offset = source_value - self.source_start;
        self.destination_start + offset
    }
}

#[derive(Debug)]
struct CategoryMap {
    source: Category,
    destination: Category,
    ranges: Vec<MapRange>,
}

impl CategoryMap {
    fn map_value(&self, source_val: u64) -> u64 {
        if let Ok(index) = self.ranges
            .binary_search_by(|test_range| {
                match test_range.source_start.cmp(&source_val) {
                    Ordering::Greater => Ordering::Greater,
                    Ordering::Equal => Ordering::Equal,
                    Ordering::Less => if source_val < test_range.source_end() {
                        Ordering::Equal
                    } else {
                        Ordering::Less
                    },
                }
            }) {
            self.ranges[index].map_value_unchecked(source_val)
        } else {
            source_val
        }
    }
}

fn parse_input(input_file: &mut File) -> AdventResult<(Vec<u64>, Vec<CategoryMap>)> {
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let Some((seeds_line, rest)) = input.split_once("\n\n") else {
        return Err(InputParse(String::from("Failed to extract seed line")));
    };

    let seeds = seeds_line
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().map_err(|_| InputParse(format!("Failed to parse seed number '{s}'"))))
        .collect::<AdventResult<_>>()?;

    let mut maps = parser::as_vec_by_block(rest, "\n\n", block_parser)?;
    maps.sort_unstable_by_key(|map| map.source as u8);

    Ok((seeds, maps))
}

lazy_static! {
    static ref MAP_TYPE_REGEX: Regex = Regex::new(r"(?<source>\w+)-to-(?<destination>\w+)").unwrap();
}

fn block_parser(block: &str) -> AdventResult<CategoryMap> {
    let Some((header, ranges)) = block.split_once('\n') else {
        return Err(InputParse(format!("Failed to separate header line - block:\n{block}")));
    };

    let Some(caps) = MAP_TYPE_REGEX.captures(header) else {
        return Err(InputParse(format!("Failed to find source/destination types in block header: {header}")));
    };

    let source = &caps["source"];
    let Ok(source) = Category::try_from(source) else {
        return Err(InputParse(format!("Failed to parse source '{source}' from block")));
    };

    let destination = &caps["destination"];
    let Ok(destination) = Category::try_from(destination) else {
        return Err(InputParse(format!("Failed to parse destination '{destination}' from block")));
    };

    let mut ranges = parser::as_vec_by_line_from_str(ranges, line_range_parser)?;
    ranges.sort_unstable_by_key(|x| x.source_start);

    Ok(CategoryMap {
        source,
        destination,
        ranges
    })
}

fn line_range_parser(line: &str) -> AdventResult<MapRange> {
    let mut iter = line.split_whitespace();

    let (Some(destination_start), Some(source_start), Some(length)) = (iter.next(), iter.next(), iter.next()) else {
        return Err(InputParse(format!("Failed to split range parts from line:\n{line}")));
    };

    let (Ok(destination_start), Ok(source_start), Ok(length)) = (destination_start.parse(), source_start.parse(), length.parse()) else {
        return Err(InputParse(format!("Failed to parse range values from line:\n{line}")));
    };

    Ok(MapRange {
        source_start,
        destination_start,
        length
    })
}
