use crate::data_structures::{Grid2D, GridPoint2D};
use crate::{parser, utils, AdventResult};
use std::collections::HashMap;
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let schematic = parser::as_grid2d_by_char(&mut input_file, |c| Ok(c))?;

    let (numbers, symbols) = locate_numbers_and_symbols(&schematic);

    // Part 1
    utils::part_header(1);
    part_1(&numbers)?;

    // Part 2
    utils::part_header(2);
    part_2(&symbols)?;

    Ok(())
}

const EMPTY_SYMBOL: char = '.';
const GEAR_SYMBOL: char = '*';

fn part_1(numbers: &Vec<LocatedNumber>) -> AdventResult<()> {
    let part_number_sum: u32 = numbers
        .iter()
        .filter(|x| x.adjacent_symbol)
        .map(|number| number.value)
        .sum();

    println!("Sum of part numbers: {part_number_sum}");

    Ok(())
}

fn part_2(symbols: &Vec<LocatedSymbol>) -> AdventResult<()> {
    let gear_ratio_sum: u32 = symbols
        .iter()
        .filter_map(|symbol| symbol.as_gear())
        .map(|gear| gear.ratio())
        .sum();

    println!("Sum of gear ratios: {gear_ratio_sum}");

    Ok(())
}

/// Locates all numbers within the schematic, and symbols which are adjacent to numbers.
/// Note: symbols which are isolated are ignored.
fn locate_numbers_and_symbols(
    schematic: &Grid2D<char>,
) -> (Vec<LocatedNumber>, Vec<LocatedSymbol>) {
    let mut numbers = Vec::new();
    let mut symbols = HashMap::new();

    let mut adjacent_symbol_pts = Vec::new();

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

            adjacent_symbol_pts.clear();

            let mut value = 0;
            let mut adjacent_symbol = false;

            let mut check_adj_symbol = |point: Option<GridPoint2D>| {
                if let Some(point) = point {
                    if let Some(&c) = schematic.get(point) {
                        if is_symbol(c) {
                            adjacent_symbol = true;
                            adjacent_symbol_pts.push(point);
                            if !symbols.contains_key(&point) {
                                symbols.insert(
                                    point,
                                    LocatedSymbol {
                                        value: c,
                                        adjacent_numbers: Vec::new(),
                                    },
                                );
                            }
                        }
                    }
                }
            };

            for point in left_boundary(GridPoint2D::new(row_num, col_num)) {
                check_adj_symbol(point);
            }

            while col_num < schematic.n_cols() && digit.is_some() {
                value = value * 10 + digit.unwrap();

                for point in center_boundary(GridPoint2D::new(row_num, col_num)) {
                    check_adj_symbol(point)
                }

                col_num += 1;
                digit = if col_num == schematic.n_cols() {
                    None
                } else {
                    schematic
                        .get_unchecked(GridPoint2D::new(row_num, col_num))
                        .to_digit(10)
                }
            }

            for point in right_boundary(GridPoint2D::new(row_num, col_num - 1)) {
                check_adj_symbol(point)
            }

            numbers.push(LocatedNumber {
                value,
                adjacent_symbol,
            });

            for point in &adjacent_symbol_pts {
                symbols.get_mut(point).unwrap().adjacent_numbers.push(value);
            }
        }
    }

    (numbers, symbols.drain().map(|(_, symbol)| symbol).collect())
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
    adjacent_symbol: bool,
}

#[derive(Debug)]
struct LocatedSymbol {
    value: char,
    adjacent_numbers: Vec<u32>,
}

impl LocatedSymbol {
    fn is_gear(&self) -> bool {
        self.value == GEAR_SYMBOL && self.adjacent_numbers.len() == 2
    }

    fn as_gear(&self) -> Option<Gear> {
        if self.is_gear() {
            Some(Gear {
                value_1: self.adjacent_numbers[0],
                value_2: self.adjacent_numbers[1],
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Gear {
    value_1: u32,
    value_2: u32,
}

impl Gear {
    fn ratio(&self) -> u32 {
        self.value_1 * self.value_2
    }
}
