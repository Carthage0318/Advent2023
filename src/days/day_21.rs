use crate::data_structures::{Grid2D, GridPoint2D};
use crate::AdventErr::InputParse;
use crate::{parser, utils, AdventResult};
use std::collections::VecDeque;
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let (grid, starting_position) = parse_input(&mut input_file)?;

    // Part 1
    utils::part_header(1);
    part_1(&grid, starting_position);

    Ok(())
}

fn part_1(grid: &Grid2D<Tile>, starting_position: GridPoint2D) {
    let reachable_plots = count_visitable(grid, starting_position, 64);

    println!("Reachable garden plots in 64 steps: {reachable_plots}");
}

fn count_visitable(
    reference_grid: &Grid2D<Tile>,
    starting_position: GridPoint2D,
    total_steps: u64,
) -> usize {
    let mut steps_grid = Grid2D::new(reference_grid.n_rows(), reference_grid.n_cols(), None);

    let mut queue = VecDeque::new();
    queue.push_back((starting_position, 0_u64));

    while let Some((point, steps_taken)) = queue.pop_front() {
        let Some(&tile) = reference_grid.get(point) else {
            continue;
        };

        match tile {
            Tile::Rock => continue,
            Tile::Garden => {
                let recorded_steps = steps_grid.get_mut_unchecked(point);
                if recorded_steps.is_some() {
                    continue;
                }

                *recorded_steps = Some(steps_taken);
                if steps_taken < total_steps {
                    if let Some(next) = point.previous_row() {
                        queue.push_back((next, steps_taken + 1));
                    }
                    if let Some(next) = point.previous_column() {
                        queue.push_back((next, steps_taken + 1))
                    }
                    queue.push_back((point.next_row(), steps_taken + 1));
                    queue.push_back((point.next_column(), steps_taken + 1));
                }
            }
        }
    }

    steps_grid
        .cells()
        .filter(|&&x| match x {
            Some(steps_required) => steps_required % 2 == total_steps % 2,
            None => false,
        })
        .count()
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
