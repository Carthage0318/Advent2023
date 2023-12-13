use crate::data_structures::Grid2D;
use crate::AdventErr::{Compute, InputParse};
use crate::{parser, utils, AdventErr, AdventResult};
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let terrains = parser::as_vec_by_block(&mut input_file, "\n\n", |block| {
        parser::as_grid2d_by_char_from_str(block, |c| Terrain::try_from(c))
    })?;

    // Part 1
    utils::part_header(1);
    part_1(&terrains)?;

    Ok(())
}

fn part_1(terrains: &[TerrainPattern]) -> AdventResult<()> {
    let pattern_summary: usize = terrains
        .iter()
        .map(|terrain| {
            MirrorLine::try_from(terrain).map(|mirror| match mirror {
                MirrorLine::Column(column) => column,
                MirrorLine::Row(row) => 100 * row,
            })
        })
        .sum::<AdventResult<_>>()?;

    println!("Summary of pattern notes: {pattern_summary}");

    Ok(())
}

type TerrainPattern = Grid2D<Terrain>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Terrain {
    Ash,
    Rock,
}

impl TryFrom<char> for Terrain {
    type Error = AdventErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Terrain::Ash),
            '#' => Ok(Terrain::Rock),
            _ => Err(InputParse(format!("Unrecognized character '{value}'"))),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum MirrorLine {
    Row(usize),
    Column(usize),
}

impl<T> TryFrom<&Grid2D<T>> for MirrorLine
where
    T: Eq,
{
    type Error = AdventErr;

    fn try_from(grid: &Grid2D<T>) -> Result<Self, Self::Error> {
        for row in 0..grid.n_rows() {
            if grid.row_is_mirror(row) {
                return Ok(MirrorLine::Row(row));
            }
        }

        for column in 0..grid.n_cols() {
            if grid.column_is_mirror(column) {
                return Ok(MirrorLine::Column(column));
            }
        }

        Err(Compute(String::from("Failed to find mirror line")))
    }
}

impl<T> Grid2D<T>
where
    T: Eq,
{
    fn row_is_mirror(&self, row_num: usize) -> bool {
        if row_num == 0 || row_num >= self.n_rows() {
            return false;
        }

        let rows_to_bottom = self.n_rows() - row_num;
        let min_row = if row_num < rows_to_bottom {
            0
        } else {
            row_num - rows_to_bottom
        };

        for first_row in min_row..row_num {
            let second_row = row_num + (row_num - first_row - 1);

            if self
                .row_unchecked(first_row)
                .iter()
                .zip(self.row_unchecked(second_row))
                .any(|(value_a, value_b)| *value_a != *value_b)
            {
                return false;
            }
        }

        true
    }

    fn column_is_mirror(&self, column_num: usize) -> bool {
        if column_num == 0 || column_num >= self.n_cols() {
            return false;
        }

        let cols_to_edge = self.n_cols() - column_num;
        let min_col = if column_num < cols_to_edge {
            0
        } else {
            column_num - cols_to_edge
        };

        for first_column in min_col..column_num {
            let second_column = column_num + (column_num - first_column - 1);

            if self
                .column_unchecked(first_column)
                .zip(self.column_unchecked(second_column))
                .any(|(value_a, value_b)| *value_a != *value_b)
            {
                return false;
            }
        }

        true
    }
}
