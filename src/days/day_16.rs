use crate::data_structures::{Grid2D, GridPoint2D};
use crate::AdventErr::InputParse;
use crate::{parser, utils, AdventErr, AdventResult};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let grid = parser::as_grid2d_by_char(&mut input_file, |c| Tile::try_from(c))?;

    // Part 1
    utils::part_header(1);
    part_1(&grid);

    Ok(())
}

fn part_1(reference_grid: &Grid2D<Tile>) {
    let mut visited_grid = Grid2D::new(
        reference_grid.n_rows(),
        reference_grid.n_cols(),
        VisitedTile::new(),
    );
    let mut process_queue = VecDeque::new();
    process_queue.push_back((GridPoint2D::new(0, 0), Direction::Right));

    while let Some((point, direction)) = process_queue.pop_front() {
        let Some(visited_tile) = visited_grid.get_mut(point) else {
            continue;
        };

        if visited_tile.already_visited(direction) {
            continue;
        }

        visited_tile.visit(direction);

        match reference_grid.get_unchecked(point) {
            Tile::Empty => {
                if let Some(next) = point.move_direction(direction) {
                    process_queue.push_back((next, direction));
                }
            }

            Tile::MirrorForward => {
                if let (Some(next), direction) = point.reflect_forward(direction) {
                    process_queue.push_back((next, direction));
                }
            }

            Tile::MirrorBackward => {
                if let (Some(next), direction) = point.reflect_backward(direction) {
                    process_queue.push_back((next, direction));
                }
            }

            Tile::SplitToVertical => match direction {
                Direction::Up | Direction::Down => {
                    if let Some(next) = point.move_direction(direction) {
                        process_queue.push_back((next, direction));
                    }
                }

                Direction::Left | Direction::Right => {
                    if let Some(next) = point.move_direction(Direction::Up) {
                        process_queue.push_back((next, Direction::Up));
                    }

                    if let Some(next) = point.move_direction(Direction::Down) {
                        process_queue.push_back((next, Direction::Down));
                    }
                }
            },

            Tile::SplitToHorizontal => match direction {
                Direction::Left | Direction::Right => {
                    if let Some(next) = point.move_direction(direction) {
                        process_queue.push_back((next, direction));
                    }
                }

                Direction::Up | Direction::Down => {
                    if let Some(next) = point.move_direction(Direction::Left) {
                        process_queue.push_back((next, Direction::Left));
                    }

                    if let Some(next) = point.move_direction(Direction::Right) {
                        process_queue.push_back((next, Direction::Right));
                    }
                }
            },
        }
    }

    let total_energized = visited_grid.cells().filter(|tile| tile.energized()).count();

    println!("Energized tiles: {total_energized}");
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitToVertical,
    SplitToHorizontal,
}

impl TryFrom<char> for Tile {
    type Error = AdventErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Empty,
            '/' => Tile::MirrorForward,
            '\\' => Tile::MirrorBackward,
            '|' => Tile::SplitToVertical,
            '-' => Tile::SplitToHorizontal,
            _ => return Err(InputParse(format!("Unrecognized character '{value}'"))),
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn reflect_forward(self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Right,
        }
    }

    fn reflect_backward(self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct VisitedTile {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl VisitedTile {
    fn new() -> Self {
        Self {
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }

    fn already_visited(self, direction: Direction) -> bool {
        match direction {
            Direction::Up => self.up,
            Direction::Down => self.down,
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }

    fn visit(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.up = true,
            Direction::Down => self.down = true,
            Direction::Left => self.left = true,
            Direction::Right => self.right = true,
        }
    }

    fn energized(self) -> bool {
        self.left || self.right || self.up || self.down
    }
}

impl Display for VisitedTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.energized() {
            write!(f, "#")
        } else {
            write!(f, ".")
        }
    }
}

impl GridPoint2D {
    fn move_direction(self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::Up => self.previous_row(),
            Direction::Down => Some(self.next_row()),
            Direction::Left => self.previous_column(),
            Direction::Right => Some(self.next_column()),
        }
    }

    fn reflect_forward(self, direction: Direction) -> (Option<Self>, Direction) {
        let new_direction = direction.reflect_forward();
        (self.move_direction(new_direction), new_direction)
    }

    fn reflect_backward(self, direction: Direction) -> (Option<Self>, Direction) {
        let new_direction = direction.reflect_backward();
        (self.move_direction(new_direction), new_direction)
    }
}
