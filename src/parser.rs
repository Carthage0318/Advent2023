use crate::AdventResult;
use std::fs::File;
use std::io::Read;

pub fn as_vec_by_line<T>(
    input: &mut File,
    line_parser: impl Fn(&str) -> AdventResult<T>,
) -> AdventResult<Vec<T>> {
    let mut input_str = String::new();
    input.read_to_string(&mut input_str)?;
    as_vec_by_line_from_str(&mut input_str, line_parser)
}

pub fn as_vec_by_line_from_str<T>(
    input: &str,
    line_parser: impl Fn(&str) -> AdventResult<T>,
) -> AdventResult<Vec<T>> {
    input.lines().map(|line| line_parser(line.trim())).collect()
}
