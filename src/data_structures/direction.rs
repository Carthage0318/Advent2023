#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Direction {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}

impl Direction {
    pub fn is_left_turn(start: Self, end: Self) -> bool {
        ((end as u8 + 4) - (start as u8)) % 4 == 1
    }

    pub fn reverse(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    pub const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];
}

#[cfg(test)]
mod tests {
    use crate::data_structures::Direction;

    #[test]
    fn test_is_left_turn() {
        let cases = [
            (Direction::Up, Direction::Left, true),
            (Direction::Left, Direction::Down, true),
            (Direction::Down, Direction::Right, true),
            (Direction::Right, Direction::Up, true),
            (Direction::Right, Direction::Down, false),
            (Direction::Right, Direction::Left, false),
        ];

        for (start, end, expected) in cases {
            assert_eq!(expected, Direction::is_left_turn(start, end));
        }
    }
}
