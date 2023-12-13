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

    // Part 2
    utils::part_header(2);
    part_2(&terrains)?;

    Ok(())
}

fn part_1(terrains: &[TerrainPattern]) -> AdventResult<()> {
    let pattern_summary: usize = terrains
        .iter()
        .map(|terrain| {
            terrain
                .find_mirror_line(0)
                .map(|mirror| mirror.summary_value())
        })
        .sum::<AdventResult<_>>()?;

    println!("Summary of pattern notes: {pattern_summary}");

    Ok(())
}

fn part_2(terrains: &[TerrainPattern]) -> AdventResult<()> {
    let pattern_summary: usize = terrains
        .iter()
        .map(|terrain| {
            terrain
                .find_mirror_line(1)
                .map(|mirror| mirror.summary_value())
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

impl MirrorLine {
    fn summary_value(&self) -> usize {
        match self {
            Self::Row(row) => 100 * row,
            Self::Column(column) => *column,
        }
    }
}

impl<T> Grid2D<T>
where
    T: Eq,
{
    fn find_mirror_line(&self, smudge_count: usize) -> AdventResult<MirrorLine> {
        for row in 0..self.n_rows() {
            if self.row_is_mirror(row, smudge_count) {
                return Ok(MirrorLine::Row(row));
            }
        }

        for column in 0..self.n_cols() {
            if self.column_is_mirror(column, smudge_count) {
                return Ok(MirrorLine::Column(column));
            }
        }

        Err(Compute(format!(
            "Failed to find mirror line with {smudge_count} smudges"
        )))
    }

    fn row_is_mirror(&self, row_num: usize, smudge_count: usize) -> bool {
        if row_num == 0 || row_num >= self.n_rows() {
            return false;
        }

        let rows_to_bottom = self.n_rows() - row_num;
        let min_row = if row_num < rows_to_bottom {
            0
        } else {
            row_num - rows_to_bottom
        };

        let mut smudges_found = 0;
        for first_row in min_row..row_num {
            let second_row = row_num + (row_num - first_row - 1);

            if self
                .row_unchecked(first_row)
                .iter()
                .zip(self.row_unchecked(second_row))
                .any(|(value_a, value_b)| {
                    if *value_a != *value_b {
                        if smudges_found == smudge_count {
                            return true;
                        }
                        smudges_found += 1;
                    }
                    false
                })
            {
                return false;
            }
        }

        smudges_found == smudge_count
    }

    fn column_is_mirror(&self, column_num: usize, smudge_count: usize) -> bool {
        if column_num == 0 || column_num >= self.n_cols() {
            return false;
        }

        let cols_to_edge = self.n_cols() - column_num;
        let min_col = if column_num < cols_to_edge {
            0
        } else {
            column_num - cols_to_edge
        };

        let mut smudges_found = 0;
        for first_column in min_col..column_num {
            let second_column = column_num + (column_num - first_column - 1);

            if self
                .column_unchecked(first_column)
                .zip(self.column_unchecked(second_column))
                .any(|(value_a, value_b)| {
                    if *value_a != *value_b {
                        if smudges_found == smudge_count {
                            return true;
                        }
                        smudges_found += 1;
                    }
                    false
                })
            {
                return false;
            }
        }

        smudges_found == smudge_count
    }
}
