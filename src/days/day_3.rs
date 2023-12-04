use crate::data_structures::{Grid2D, GridPoint2D};
use crate::{parser, utils, AdventResult};
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let schematic = parser::as_grid2d_by_char(&mut input_file, |c| Ok(c))?;

    // Part 1
    utils::part_header(1);
    part_1(&schematic)?;

    Ok(())
}

const EMPTY_SYMBOL: char = '.';

fn part_1(schematic: &Grid2D<char>) -> AdventResult<()> {
    let numbers = locate_numbers(schematic);

    let part_number_sum: u32 = numbers
        .iter()
        .filter(|x| x.adjacent_symbol.is_some())
        .map(|number| number.value)
        .sum();

    println!("Sum of part numbers: {part_number_sum}");

    Ok(())
}

fn locate_numbers(schematic: &Grid2D<char>) -> Vec<LocatedNumber> {
    let mut vec = Vec::new();

    for row_num in 0..schematic.n_rows() {
        let mut col_num = 0;
        while col_num < schematic.n_cols() {
            let mut digit = schematic
                .get_unchecked(GridPoint2D::new(row_num, col_num))
                .to_digit(10);
            if digit.is_none() {
                col_num += 1;
                continue;
            }

            let mut value = 0;
            let mut symbol = None;

            for point in left_boundary(GridPoint2D::new(row_num, col_num))
                .into_iter()
                .filter_map(std::convert::identity)
            {
                if let Some(&c) = schematic.get(point) {
                    if is_symbol(c) {
                        symbol = Some(c);
                    }
                }
            }

            while col_num < schematic.n_cols() && digit.is_some() {
                value = value * 10 + digit.unwrap();
                if symbol.is_none() {
                    for point in center_boundary(GridPoint2D::new(row_num, col_num))
                        .into_iter()
                        .filter_map(std::convert::identity)
                    {
                        if let Some(&c) = schematic.get(point) {
                            if is_symbol(c) {
                                symbol = Some(c);
                            }
                        }
                    }
                }

                col_num += 1;
                digit = if col_num == schematic.n_cols() {
                    None
                } else {
                    schematic.get_unchecked(GridPoint2D::new(row_num, col_num)).to_digit(10)
                }
            }

            if symbol.is_none() {
                for point in right_boundary(GridPoint2D::new(row_num, col_num - 1))
                    .into_iter()
                    .filter_map(std::convert::identity)
                {
                    if let Some(&c) = schematic.get(point) {
                        if is_symbol(c) {
                            symbol = Some(c);
                        }
                    }
                }
            }

            vec.push(LocatedNumber {
                value,
                adjacent_symbol: symbol
            })
        }
    }

    vec
}

fn is_symbol(c: char) -> bool {
    c != EMPTY_SYMBOL && !char::is_digit(c, 10)
}

fn left_boundary(point: GridPoint2D) -> [Option<GridPoint2D>; 3] {
    let row_safe = point.row > 0;
    let col_safe = point.col > 0;

    [
        if row_safe && col_safe {
            Some(GridPoint2D::new(point.row - 1, point.col - 1))
        } else {
            None
        },
        if col_safe {
            Some(GridPoint2D::new(point.row, point.col - 1))
        } else {
            None
        },
        if col_safe {
            Some(GridPoint2D::new(point.row + 1, point.col - 1))
        } else {
            None
        },
    ]
}

fn center_boundary(point: GridPoint2D) -> [Option<GridPoint2D>; 2] {
    let row_safe = point.row > 0;

    [
        if row_safe {
            Some(GridPoint2D::new(point.row - 1, point.col))
        } else {
            None
        },
        Some(GridPoint2D::new(point.row + 1, point.col)),
    ]
}

fn right_boundary(point: GridPoint2D) -> [Option<GridPoint2D>; 3] {
    let row_safe = point.row > 0;

    [
        if row_safe {
            Some(GridPoint2D::new(point.row - 1, point.col + 1))
        } else {
            None
        },
        Some(GridPoint2D::new(point.row, point.col + 1)),
        Some(GridPoint2D::new(point.row + 1, point.col + 1)),
    ]
}

#[derive(Debug, Copy, Clone)]
struct LocatedNumber {
    value: u32,
    adjacent_symbol: Option<char>,
}
