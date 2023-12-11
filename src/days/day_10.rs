use crate::data_structures::{Grid2D, GridPoint2D};
use crate::AdventErr::{Compute, InputParse};
use crate::{parser, utils, AdventErr, AdventResult};
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let (grid, start_point) = parse_input(&mut input_file)?;

    // Part 1
    utils::part_header(1);
    part_1(&grid, start_point)?;

    Ok(())
}

fn part_1(grid: &Grid2D<Tile>, start_point: GridPoint2D) -> AdventResult<()> {
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

    println!("Steps to farthest point: {}", step_count / 2);

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn flip(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Pipe(Direction, Direction),
    Ground,
    Start,
}

impl TryFrom<char> for Tile {
    type Error = AdventErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Direction as D;
        match value {
            '|' => Ok(Tile::Pipe(D::North, D::South)),
            '-' => Ok(Tile::Pipe(D::East, D::West)),
            'L' => Ok(Tile::Pipe(D::North, D::East)),
            'J' => Ok(Tile::Pipe(D::North, D::West)),
            '7' => Ok(Tile::Pipe(D::South, D::West)),
            'F' => Ok(Tile::Pipe(D::South, D::East)),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            _ => Err(InputParse(format!("Unknown character '{value}'"))),
        }
    }
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

impl GridPoint2D {
    fn north(&self) -> Option<Self> {
        if self.row == 0 {
            None
        } else {
            Some(Self::new(self.row - 1, self.col))
        }
    }

    fn west(&self) -> Option<Self> {
        if self.col == 0 {
            None
        } else {
            Some(Self::new(self.row, self.col - 1))
        }
    }

    fn east(&self) -> Self {
        Self::new(self.row, self.col + 1)
    }

    fn south(&self) -> Self {
        Self::new(self.row + 1, self.col)
    }

    fn go(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::North => self.north(),
            Direction::East => Some(self.east()),
            Direction::South => Some(self.south()),
            Direction::West => self.west(),
        }
    }
}
