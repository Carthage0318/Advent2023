use crate::data_structures::Direction;

#[derive(Debug, Copy, Clone)]
pub(super) struct ProtoBox {
    pub(super) start_column: i64,
    pub(super) start_row: i64,
    pub(super) end_row: i64,
}

impl ProtoBox {
    pub(super) fn commit(self, end_column_exclusive: i64) -> u64 {
        (end_column_exclusive - self.start_column) as u64
            * (self.end_row + 1 - self.start_row) as u64
    }

    pub(super) fn is_empty(self) -> bool {
        self.start_row > self.end_row
    }
}

impl From<VerticalEdge> for ProtoBox {
    fn from(value: VerticalEdge) -> Self {
        Self {
            start_column: value.column,
            start_row: value.row_start,
            end_row: value.row_end,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) struct VerticalEdge {
    pub(super) column: i64,
    pub(super) row_start: i64,
    pub(super) row_end: i64,
    pub(super) is_start: bool,
    pub(super) start_corner_convex: bool,
    pub(super) end_corner_convex: bool,
}

impl VerticalEdge {
    pub(super) fn new(
        column: i64,
        row_start: i64,
        row_end: i64,
        is_start: bool,
        start_corner_convex: bool,
        end_corner_convex: bool,
    ) -> Self {
        assert!(row_start <= row_end);
        Self {
            column,
            row_start,
            row_end,
            is_start,
            start_corner_convex,
            end_corner_convex,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instruction {
    pub(super) direction: Direction,
    pub(super) length: i64,
}
