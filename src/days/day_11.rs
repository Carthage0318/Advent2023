use crate::data_structures::{Grid2D, GridPoint2D};
use crate::AdventErr::InputParse;
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
    part_1(&galaxies, &image);

    // Part 2
    utils::part_header(2);
    part_2(&galaxies, &image);

    Ok(())
}

fn part_1(galaxies: &[GridPoint2D], image: &Grid2D<Element>) {
    let expanded_galaxies = expand_universe(galaxies, image, 2);

    let pairwise_distance_sum: usize = sum_pairwise_manhattan_distances(&expanded_galaxies);

    println!("Sum of pairwise galaxy distances: {pairwise_distance_sum}");
}

fn part_2(galaxies: &[GridPoint2D], image: &Grid2D<Element>) {
    let expanded_galaxies = expand_universe(galaxies, image, 1_000_000);

    let pairwise_distance_sum: usize = sum_pairwise_manhattan_distances(&expanded_galaxies);

    println!("Sum of pairwise galaxy distances: {pairwise_distance_sum}");
}

fn expand_universe(
    galaxies: &[GridPoint2D],
    image: &Grid2D<Element>,
    expansion_factor: usize,
) -> Vec<GridPoint2D> {
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
        galaxy.row += row_skips[galaxy.row] * (expansion_factor - 1);
        galaxy.col += col_skips[galaxy.col] * (expansion_factor - 1);
    }

    expanded_galaxies
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

fn sum_pairwise_manhattan_distances(points: &[GridPoint2D]) -> usize {
    points
        .iter()
        .enumerate()
        .map(|(i, point_1)| {
            points
                .iter()
                .skip(i + 1)
                .map(|&point_2| point_1.manhattan_distance(point_2))
                .sum::<usize>()
        })
        .sum()
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
