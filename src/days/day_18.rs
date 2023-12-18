use crate::data_structures::{Direction, Grid2D, GridPoint2D};
use crate::AdventErr::{Compute, InputParse};
use crate::{parser, utils, AdventResult};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let instructions = parser::as_vec_by_line(&mut input_file, line_parser)?;

    // Part 1
    utils::part_header(1);
    part_1(&instructions)?;

    Ok(())
}

fn part_1(instructions: &[Instruction]) -> AdventResult<()> {
    let mut grid = create_grid(&instructions);
    flood_fill(&mut grid)?;

    let trench_size = grid
        .cells()
        .filter(|&&cell| cell == Terrain::Trench)
        .count();

    println!("Trench size: {trench_size}");

    Ok(())
}

fn create_grid(instructions: &[Instruction]) -> Grid2D<Terrain> {
    let mut min_row = 0;
    let mut min_col = 0;
    let mut max_row = 0;
    let mut max_col = 0;

    let mut current_row = 0;
    let mut current_col = 0;
    for instruction in instructions {
        match instruction.direction {
            Direction::Up => current_row -= instruction.length as i64,
            Direction::Down => current_row += instruction.length as i64,
            Direction::Left => current_col -= instruction.length as i64,
            Direction::Right => current_col += instruction.length as i64,
        }

        min_row = min(min_row, current_row);
        max_row = max(max_row, current_row);
        min_col = min(min_col, current_col);
        max_col = max(max_col, current_col);
    }

    let mut grid = Grid2D::new(
        (max_row - min_row + 1) as usize,
        (max_col - min_col + 1) as usize,
        Terrain::GroundLevel,
    );
    let mut current_row = (-1_i64 * min_row) as usize;
    let mut current_col = (-1_i64 * min_col) as usize;

    for instruction in instructions {
        match instruction.direction {
            Direction::Up => {
                for _ in 0..instruction.length {
                    current_row -= 1;
                    *grid.get_mut_unchecked(GridPoint2D::new(current_row, current_col)) =
                        Terrain::Trench;
                }
            }

            Direction::Down => {
                for _ in 0..instruction.length {
                    current_row += 1;
                    *grid.get_mut_unchecked(GridPoint2D::new(current_row, current_col)) =
                        Terrain::Trench;
                }
            }

            Direction::Left => {
                for _ in 0..instruction.length {
                    current_col -= 1;
                    *grid.get_mut_unchecked(GridPoint2D::new(current_row, current_col)) =
                        Terrain::Trench;
                }
            }

            Direction::Right => {
                for _ in 0..instruction.length {
                    current_col += 1;
                    *grid.get_mut_unchecked(GridPoint2D::new(current_row, current_col)) =
                        Terrain::Trench;
                }
            }
        }
    }

    grid
}

fn flood_fill(grid: &mut Grid2D<Terrain>) -> AdventResult<()> {
    let mut initial_point = None;
    for row in 1..grid.n_rows() - 1 {
        for col in 0..grid.n_cols() - 1 {
            if *grid.get_unchecked(GridPoint2D::new(row, col)) == Terrain::Trench
                && *grid.get_unchecked(GridPoint2D::new(row, col + 1)) == Terrain::GroundLevel
            {
                initial_point = Some(GridPoint2D::new(row, col + 1));
                break;
            }
        }
    }

    let Some(initial_point) = initial_point else {
        return Err(Compute(String::from("Failed to find interior point")));
    };

    let mut queue = VecDeque::new();
    queue.push_back(initial_point);

    while let Some(point) = queue.pop_front() {
        let Some(current) = grid.get_mut(point) else {
            continue;
        };

        if *current == Terrain::Trench {
            continue;
        }

        *current = Terrain::Trench;
        for direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(neighbor) = point.move_direction(direction) {
                queue.push_back(neighbor);
            }
        }
    }

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Terrain {
    GroundLevel,
    Trench,
}

impl Display for Terrain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Terrain::GroundLevel => write!(f, "."),
            Terrain::Trench => write!(f, "#"),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Instruction {
    direction: Direction,
    length: usize,
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"(?<direction>[RUDL]) (?<length>\d+)").unwrap();
}

fn line_parser(line: &str) -> AdventResult<Instruction> {
    let Some(caps) = LINE_REGEX.captures(line) else {
        return Err(InputParse(format!("Unable to parse line:\n{line}")));
    };

    let direction = &caps["direction"];
    let direction = char_to_direction(direction.chars().next().unwrap())?;
    let length: usize = (&caps["length"])
        .parse()
        .map_err(|_| InputParse(format!("Failed to parse length")))?;

    Ok(Instruction { direction, length })
}

fn char_to_direction(c: char) -> AdventResult<Direction> {
    Ok(match c {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => return Err(InputParse(format!("Unrecognized character '{c}'"))),
    })
}
