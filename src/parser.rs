use crate::data_structures::Grid2D;
use crate::AdventErr::InputParse;
use crate::AdventResult;
use std::fs::File;
use std::io::Read;

pub fn as_vec_by_line<T>(
    input: &mut File,
    line_parser: impl Fn(&str) -> AdventResult<T>,
) -> AdventResult<Vec<T>> {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str)?;
    as_vec_by_line_from_str(&input_str, line_parser)
}

fn as_vec_by_line_from_str<T>(
    input: &str,
    line_parser: impl Fn(&str) -> AdventResult<T>,
) -> AdventResult<Vec<T>> {
    input.lines().map(|line| line_parser(line.trim())).collect()
}

pub fn as_grid2d_by_char<T>(
    input: &mut File,
    char_parser: impl Fn(char) -> AdventResult<T>,
) -> AdventResult<Grid2D<T>> {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str)?;

    as_grid2d_by_char_from_str(&input_str, char_parser)
}

fn as_grid2d_by_char_from_str<T>(
    input: &str,
    char_parser: impl Fn(char) -> AdventResult<T>,
) -> AdventResult<Grid2D<T>> {
    let n_rows = input.lines().count();
    let n_cols = input
        .lines()
        .next()
        .ok_or_else(|| InputParse(String::from("Malformed input - empty first line")))?
        .chars()
        .count();

    let vec = input
        .chars()
        .filter(|&c| c != '\n' && c != '\r')
        .map(char_parser)
        .collect::<AdventResult<_>>()?;

    Ok(Grid2D::from(vec, n_rows, n_cols))
}
