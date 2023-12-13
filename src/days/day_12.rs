use crate::AdventErr::InputParse;
use crate::{parser, utils, AdventErr, AdventResult};
use std::collections::HashMap;
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let spring_rows = parser::as_vec_by_line(&mut input_file, line_parser)?;

    // Part 1
    utils::part_header(1);
    part_1(&spring_rows);

    // Part 2
    utils::part_header(2);
    part_2(&spring_rows);

    Ok(())
}

fn part_1(spring_rows: &[SpringRow]) {
    let sum_total_arrangements: u64 = spring_rows.iter().map(number_solutions).sum();

    println!("Sum of total arrangements: {sum_total_arrangements}");
}

fn part_2(original_spring_rows: &[SpringRow]) {
    let spring_rows: Vec<_> = original_spring_rows
        .iter()
        .map(|spring_row| spring_row.expand(5))
        .collect();

    let sum_total_arrangements: u64 = spring_rows.iter().map(number_solutions).sum();

    println!("Sum of total arrangements: {sum_total_arrangements}");
}

fn number_solutions(spring_row: &SpringRow) -> u64 {
    let damaged_possible = spring_row
        .springs
        .iter()
        .filter(|&&s| s == Spring::Damaged || s == Spring::Unknown)
        .count();

    let damaged_required = spring_row.groups.iter().sum();

    let total_required = if spring_row.groups.is_empty() {
        0
    } else {
        damaged_required + (spring_row.groups.len() - 1)
    };

    let mut cache = HashMap::new();

    number_solutions_internal(
        &mut cache,
        &spring_row.springs,
        &spring_row.groups,
        None,
        damaged_possible,
        damaged_required,
        total_required,
    )
}

fn number_solutions_internal(
    cache: &mut HashMap<(usize, usize), u64>,
    springs: &[Spring],
    groups: &[usize],
    must_place: Option<usize>,
    damaged_possible_remaining: usize,
    damaged_required: usize,
    total_required: usize,
) -> u64 {
    if damaged_required > damaged_possible_remaining {
        return 0;
    }

    if total_required > springs.len() {
        return 0;
    }

    if springs.is_empty() {
        return 1;
    }

    let current = springs[0];

    match must_place {
        Some(0) => {
            // This spring must be Good.
            match current {
                Spring::Damaged => 0, // Contradiction
                Spring::Good => number_solutions_internal(
                    cache,
                    &springs[1..],
                    groups,
                    None,
                    damaged_possible_remaining,
                    damaged_required,
                    if groups.is_empty() {
                        total_required // should be 0 at this point
                    } else {
                        total_required - 1
                    },
                ),
                Spring::Unknown => number_solutions_internal(
                    cache,
                    &springs[1..],
                    groups,
                    None,
                    damaged_possible_remaining - 1,
                    damaged_required,
                    if groups.is_empty() {
                        total_required // should be 0 at this point
                    } else {
                        total_required - 1
                    },
                ),
            }
        }

        Some(must_place) => {
            // This spring must be Damaged.
            match current {
                Spring::Good => 0, // Contradiction
                Spring::Damaged | Spring::Unknown => number_solutions_internal(
                    cache,
                    &springs[1..],
                    groups,
                    Some(must_place - 1),
                    damaged_possible_remaining - 1,
                    damaged_required - 1,
                    total_required - 1,
                ),
            }
        }

        None => {
            let current_position = (springs.len(), groups.len());
            if let Some(&cached_result) = cache.get(&current_position) {
                return cached_result;
            }

            let result = match current {
                Spring::Good => number_solutions_internal(
                    cache,
                    &springs[1..],
                    groups,
                    None,
                    damaged_possible_remaining,
                    damaged_required,
                    total_required,
                ),
                Spring::Damaged => {
                    let Some(&current_group) = groups.first() else {
                        return 0;
                    };

                    number_solutions_internal(
                        cache,
                        &springs[1..],
                        &groups[1..],
                        Some(current_group - 1),
                        damaged_possible_remaining - 1,
                        damaged_required - 1,
                        total_required - 1,
                    )
                }
                Spring::Unknown => {
                    let number_good = number_solutions_internal(
                        cache,
                        &springs[1..],
                        groups,
                        None,
                        damaged_possible_remaining - 1,
                        damaged_required,
                        total_required,
                    );

                    let number_damaged = if let Some(&current_group) = groups.first() {
                        number_solutions_internal(
                            cache,
                            &springs[1..],
                            &groups[1..],
                            Some(current_group - 1),
                            damaged_possible_remaining - 1,
                            damaged_required - 1,
                            total_required - 1,
                        )
                    } else {
                        // Can't put damaged if there's no group
                        0
                    };

                    number_good + number_damaged
                }
            };

            cache.insert(current_position, result);
            result
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Spring {
    Good,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = AdventErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Spring::Good),
            '#' => Ok(Spring::Damaged),
            '?' => Ok(Spring::Unknown),
            _ => Err(InputParse(format!("Unrecognized character '{value}'"))),
        }
    }
}

#[derive(Debug)]
struct SpringRow {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl SpringRow {
    fn expand(&self, factor: usize) -> Self {
        assert!(factor > 1);
        let mut springs = Vec::with_capacity(self.springs.len() * 5 + 4);
        let groups = self.groups.repeat(factor);

        springs.extend(&self.springs);

        for _ in 1..factor {
            springs.push(Spring::Unknown);
            springs.extend(&self.springs);
        }

        Self { springs, groups }
    }
}

fn line_parser(line: &str) -> AdventResult<SpringRow> {
    let Some((springs, groups)) = line.split_once(' ') else {
        return Err(InputParse(format!("Failed to split line:\n{line}")));
    };

    let springs: Vec<_> = springs
        .chars()
        .map(Spring::try_from)
        .collect::<AdventResult<_>>()?;

    let groups: Vec<_> = groups
        .split(',')
        .map(|s| match s.parse::<usize>() {
            Ok(0) => Err(InputParse(String::from("Found group of 0"))),
            Ok(x) => Ok(x),
            Err(_) => Err(InputParse(format!("Failed to parse group '{s}'"))),
        })
        .collect::<AdventResult<_>>()?;

    Ok(SpringRow { springs, groups })
}
