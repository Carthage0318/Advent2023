use crate::data_structures::{Grid2D, GridPoint2D};
use crate::AdventErr::{Compute, InputParse};
use crate::{parser, utils, AdventResult};
use std::fs::File;
use types::{Boundary, Direction, Tile};

mod types;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let (grid, start_point) = parse_input(&mut input_file)?;

    // Part 1
    utils::part_header(1);
    part_1(&grid, start_point)?;

    // Part 2
    utils::part_header(2);
    part_2(&grid, start_point)?;

    Ok(())
}

fn part_1(grid: &Grid2D<Tile>, start_point: GridPoint2D) -> AdventResult<()> {
    let step_count = traverse_cycle(
        grid,
        start_point,
        None::<fn(GridPoint2D, Direction, Direction)>,
    )?;

    println!("Steps to farthest point: {}", step_count / 2);

    Ok(())
}

fn part_2(grid: &Grid2D<Tile>, start_point: GridPoint2D) -> AdventResult<()> {
    let mut rows = vec![vec![]; grid.n_rows()];

    let pipe_mark = |point: GridPoint2D, direction_1: Direction, direction_2: Direction| {
        let ns_1 = direction_1 == Direction::North || direction_1 == Direction::South;
        let ns_2 = direction_2 == Direction::North || direction_2 == Direction::South;
        if ns_1 || ns_2 {
            let entry = if ns_1 && ns_2 {
                Boundary::Line(point.col)
            } else {
                let ns_direction = if ns_1 { direction_1 } else { direction_2 };
                Boundary::Corner(point.col, ns_direction)
            };
            rows.get_mut(point.row).unwrap().push(entry);
        }
    };

    traverse_cycle(grid, start_point, Some(pipe_mark))?;

    let enclosed_tiles: u64 = rows
        .iter_mut()
        .map(|row| {
            row.sort();
            let mut total_included: u64 = 0;
            let mut include_start = None;
            let mut last_corner_data = None;

            for boundary in row.iter() {
                match boundary {
                    Boundary::Line(col) => match include_start {
                        None => include_start = Some(col),
                        Some(&start_col) => {
                            total_included += (col - start_col - 1) as u64;
                            include_start = None;
                        }
                    },

                    Boundary::Corner(col, direction) => match last_corner_data {
                        None => {
                            let was_including = include_start.is_some();
                            if was_including {
                                let start_col = include_start.unwrap();
                                total_included += (col - start_col - 1) as u64;
                                include_start = None;
                            }

                            last_corner_data = Some((was_including, *direction));
                        }

                        Some((was_including, last_direction)) => {
                            let start_including = if last_direction == *direction {
                                was_including
                            } else {
                                !was_including
                            };

                            if start_including {
                                include_start = Some(col);
                            }

                            last_corner_data = None;
                        }
                    },
                }
            }

            Ok(total_included)
        })
        .sum::<AdventResult<_>>()?;

    println!("Enclosed tiles: {enclosed_tiles}");

    Ok(())
}

fn traverse_cycle(
    grid: &Grid2D<Tile>,
    start_point: GridPoint2D,
    mut pipe_fn: Option<impl FnMut(GridPoint2D, Direction, Direction)>,
) -> AdventResult<u64> {
    let mut step_count: u64 = 0;

    let mut current_position = start_point;
    let Some(Tile::Pipe(_, mut entry_direction)) = grid.get(start_point) else {
        return Err(Compute(format!("Start Point {start_point} is not a pipe")));
    };

    while current_position != start_point || step_count == 0 {
        let Some(current_tile) = grid.get(current_position) else {
            return Err(Compute(format!("Invalid point {current_position}")));
        };

        let Tile::Pipe(direction_1, direction_2) = current_tile else {
            return Err(Compute(format!("Ended up on a non-pipe tile")));
        };

        if pipe_fn.is_some() {
            pipe_fn.as_mut().unwrap()(current_position, *direction_1, *direction_2);
        }

        let next_direction = if entry_direction == *direction_1 {
            *direction_2
        } else {
            *direction_1
        };

        current_position = current_position
            .go(next_direction)
            .ok_or_else(|| Compute(String::from("Ran off grid edge")))?;

        entry_direction = next_direction.flip();

        step_count += 1
    }

    Ok(step_count)
}

fn parse_input(input_file: &mut File) -> AdventResult<(Grid2D<Tile>, GridPoint2D)> {
    let mut start_pos = None;

    let mut grid = parser::as_grid2d_by_char_with_pos(input_file, |point, c| {
        let tile = Tile::try_from(c)?;
        if tile == Tile::Start {
            start_pos = Some(point);
        }
        Ok(tile)
    })?;

    let Some(start_pos) = start_pos else {
        return Err(InputParse(String::from("Didn't find a start point")));
    };

    // For convenience, let's replace the Start tile with its real tile, so we can easily move from it
    let mut connects_neighbors = [false, false, false, false]; // N, E, S, W
    if let Some(north) = start_pos.north() {
        match grid.get(north) {
            Some(Tile::Pipe(Direction::South, _)) | Some(Tile::Pipe(_, Direction::South)) => {
                connects_neighbors[0] = true;
            }
            _ => {}
        }
    }

    if let Some(west) = start_pos.west() {
        match grid.get(west) {
            Some(Tile::Pipe(Direction::East, _)) | Some(Tile::Pipe(_, Direction::East)) => {
                connects_neighbors[3] = true;
            }
            _ => {}
        }
    }

    match grid.get(start_pos.east()) {
        Some(Tile::Pipe(Direction::West, _)) | Some(Tile::Pipe(_, Direction::West)) => {
            connects_neighbors[1] = true;
        }
        _ => {}
    }

    match grid.get(start_pos.south()) {
        Some(Tile::Pipe(Direction::North, _)) | Some(Tile::Pipe(_, Direction::North)) => {
            connects_neighbors[2] = true;
        }
        _ => {}
    }

    if connects_neighbors.iter().filter(|&&x| x).count() != 2 {
        return Err(InputParse(String::from(
            "Unexpected number of connections to start",
        )));
    }

    fn to_direction(x: usize) -> Option<Direction> {
        match x {
            0 => Some(Direction::North),
            1 => Some(Direction::East),
            2 => Some(Direction::South),
            3 => Some(Direction::West),
            _ => None,
        }
    }

    let start_tile = Tile::Pipe(
        to_direction(
            connects_neighbors
                .iter()
                .enumerate()
                .filter_map(|(i, x)| if *x { Some(i) } else { None })
                .next()
                .unwrap(),
        )
        .unwrap(),
        to_direction(
            connects_neighbors
                .iter()
                .enumerate()
                .rev()
                .filter_map(|(i, x)| if *x { Some(i) } else { None })
                .next()
                .unwrap(),
        )
        .unwrap(),
    );

    *grid.get_mut_unchecked(start_pos) = start_tile;

    Ok((grid, start_pos))
}
