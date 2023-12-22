use crate::data_structures::{Grid2D, GridPoint2D};
use crate::AdventErr::{Compute, InputParse};
use crate::{parser, utils, AdventResult};
use std::fs::File;

mod implementation;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let (grid, starting_position) = parse_input(&mut input_file)?;

    // Part 1
    utils::part_header(1);
    part_1(&grid, starting_position);

    // Part 2
    utils::part_header(2);
    part_2(&grid, starting_position)?;

    Ok(())
}

fn part_1(grid: &Grid2D<Tile>, starting_position: GridPoint2D) {
    let reachable_plots = implementation::count_visitable_finite(grid, starting_position, 64);

    println!("Reachable garden plots in 64 steps: {reachable_plots}");
}

fn part_2(grid: &Grid2D<Tile>, starting_position: GridPoint2D) -> AdventResult<()> {
    // Our solution relies on a few assumptions about the input.
    // These are observed features which we do not believe to be coincidental,
    // as they would be unlikely in random input, and significantly simplify the problem space.
    // 1. The entire border contains no rocks.
    // 2. The entire row and column containing the starting position contain no rocks.
    // 3. The grid is a square.

    // Assert these assumptions.
    if grid.n_rows() == 0 || grid.n_cols() == 0 {
        return Err(Compute(String::from("Empty grid")));
    }

    for &cell in grid
        .row_unchecked(0)
        .iter()
        .chain(grid.row_unchecked(grid.n_rows() - 1))
        .chain(grid.column_unchecked(0))
        .chain(grid.column_unchecked(grid.n_cols() - 1))
    {
        if cell == Tile::Rock {
            return Err(Compute(String::from(
                "Assumption violated: Rock present in map boundary",
            )));
        }
    }

    if grid.get(starting_position).is_none() {
        return Err(Compute(String::from("Invalid starting position")));
    }

    for &cell in grid
        .row_unchecked(starting_position.row)
        .iter()
        .chain(grid.column_unchecked(starting_position.col))
    {
        if cell == Tile::Rock {
            return Err(Compute(String::from(
                "Assumption violated: Rock present in starting row/column",
            )));
        }
    }

    if !grid.is_square() {
        return Err(Compute(String::from(
            "Assumption violated: Non-square grid",
        )));
    }

    const PART_2_STEPS: u64 = 26501365;
    let reachable_plots =
        implementation::count_visitable_infinite(grid, starting_position, PART_2_STEPS);

    println!("Reachable garden plots in {PART_2_STEPS} steps: {reachable_plots}");

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Garden,
    Rock,
}

fn parse_input(input_file: &mut File) -> AdventResult<(Grid2D<Tile>, GridPoint2D)> {
    let mut starting_position = None;
    let grid = parser::as_grid2d_by_char_with_pos(input_file, |point, c| match c {
        '.' => Ok(Tile::Garden),
        '#' => Ok(Tile::Rock),
        'S' => {
            if starting_position.is_none() {
                starting_position = Some(point);
                Ok(Tile::Garden)
            } else {
                Err(InputParse(String::from(
                    "Multiple starting positions found",
                )))
            }
        }
        _ => Err(InputParse(format!("Unrecognized character '{c}'"))),
    })?;

    let Some(starting_position) = starting_position else {
        return Err(InputParse(String::from("No starting position found")));
    };

    Ok((grid, starting_position))
}
