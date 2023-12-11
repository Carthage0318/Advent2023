use crate::data_structures::{Grid2D, GridPoint2D};
use crate::AdventErr::{Compute, InputParse};
use crate::{parser, utils, AdventErr, AdventResult};
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let mut galaxies = Vec::new();
    let image = parser::as_grid2d_by_char_with_pos(&mut input_file, |point, c| {
        let element = Element::try_from(c)?;
        if element == Element::Galaxy {
            galaxies.push(point);
        }
        Ok(element)
    })?;

    // Part 1
    utils::part_header(1);
    part_1(&galaxies, &image)?;

    Ok(())
}

fn part_1(galaxies: &[GridPoint2D], image: &Grid2D<Element>) -> AdventResult<()> {
    if galaxies.is_empty() {
        return Err(Compute(String::from("No galaxies")));
    }

    let mut row_empty = vec![true; image.n_rows()];
    let mut col_empty = vec![true; image.n_cols()];

    for &galaxy in galaxies {
        row_empty[galaxy.row] = false;
        col_empty[galaxy.col] = false;
    }

    let row_skips = count_true_before_or_at(&row_empty);
    let col_skips = count_true_before_or_at(&col_empty);

    let mut expanded_galaxies = galaxies.to_vec();
    for galaxy in expanded_galaxies.iter_mut() {
        galaxy.row += row_skips[galaxy.row];
        galaxy.col += col_skips[galaxy.col];
    }

    let pairwise_distance_sum: usize = expanded_galaxies
        .iter()
        .enumerate()
        .map(|(i, galaxy_1)| {
            expanded_galaxies
                .iter()
                .skip(i + 1)
                .map(|&galaxy_2| galaxy_1.manhattan_distance(galaxy_2))
                .sum::<usize>()
        })
        .sum();

    println!("Sum of pairwise galaxy distances: {pairwise_distance_sum}");

    Ok(())
}

fn count_true_before_or_at(values: &[bool]) -> Vec<usize> {
    let mut count = 0;
    values
        .iter()
        .map(|&x| {
            if x {
                count += 1;
            }
            count
        })
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Element {
    Empty,
    Galaxy,
}

impl TryFrom<char> for Element {
    type Error = AdventErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Element::Empty),
            '#' => Ok(Element::Galaxy),
            _ => Err(InputParse(format!("Unrecognized character '{value}'"))),
        }
    }
}
