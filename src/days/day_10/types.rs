use crate::data_structures::GridPoint2D;
use crate::AdventErr;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub(super) fn flip(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum Tile {
    Pipe(Direction, Direction),
    Ground,
    Start,
}

impl TryFrom<char> for Tile {
    type Error = AdventErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use crate::AdventErr::InputParse;
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum Boundary {
    Line(usize),
    Corner(usize, Direction),
}

impl Ord for Boundary {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_val = match self {
            Boundary::Line(x) => *x,
            Boundary::Corner(x, _) => *x,
        };

        let other_val = match other {
            Boundary::Line(x) => *x,
            Boundary::Corner(x, _) => *x,
        };

        self_val.cmp(&other_val)
    }
}

impl PartialOrd for Boundary {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_val = match self {
            Boundary::Line(x) => *x,
            Boundary::Corner(x, _) => *x,
        };

        let other_val = match other {
            Boundary::Line(x) => *x,
            Boundary::Corner(x, _) => *x,
        };

        self_val.partial_cmp(&other_val)
    }
}

impl GridPoint2D {
    pub(super) fn north(&self) -> Option<Self> {
        if self.row == 0 {
            None
        } else {
            Some(Self::new(self.row - 1, self.col))
        }
    }

    pub(super) fn west(&self) -> Option<Self> {
        if self.col == 0 {
            None
        } else {
            Some(Self::new(self.row, self.col - 1))
        }
    }

    pub(super) fn east(&self) -> Self {
        Self::new(self.row, self.col + 1)
    }

    pub(super) fn south(&self) -> Self {
        Self::new(self.row + 1, self.col)
    }

    pub(super) fn go(&self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::North => self.north(),
            Direction::East => Some(self.east()),
            Direction::South => Some(self.south()),
            Direction::West => self.west(),
        }
    }
}
