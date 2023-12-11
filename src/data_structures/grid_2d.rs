use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct GridPoint2D {
    pub row: usize,
    pub col: usize,
}

impl GridPoint2D {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl Display for GridPoint2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Debug)]
pub struct Grid2D<T> {
    vec: Vec<T>,
    n_rows: usize,
    n_cols: usize,
}

impl<T> Grid2D<T>
where
    T: Clone,
{
    pub fn new(n_rows: usize, n_cols: usize, init: T) -> Self {
        Self {
            vec: vec![init; n_rows * n_cols],
            n_rows,
            n_cols,
        }
    }
}

impl<T> Grid2D<T> {
    pub fn from(vec: Vec<T>, n_rows: usize, n_cols: usize) -> Self {
        assert_eq!(vec.len(), n_rows * n_cols);

        Self {
            vec,
            n_rows,
            n_cols,
        }
    }

    pub fn n_rows(&self) -> usize {
        self.n_rows
    }

    pub fn n_cols(&self) -> usize {
        self.n_cols
    }

    fn index(&self, point: GridPoint2D) -> usize {
        point.row * self.n_cols + point.col
    }

    fn in_bounds(&self, point: GridPoint2D) -> bool {
        point.row < self.n_rows && point.col < self.n_cols
    }

    pub fn get_unchecked(&self, point: GridPoint2D) -> &T {
        &self.vec[self.index(point)]
    }

    pub fn get_mut_unchecked(&mut self, point: GridPoint2D) -> &mut T {
        let index = self.index(point);
        &mut self.vec[index]
    }

    pub fn get(&self, point: GridPoint2D) -> Option<&T> {
        if self.in_bounds(point) {
            Some(self.get_unchecked(point))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn get_mut(&mut self, point: GridPoint2D) -> Option<&mut T> {
        if self.in_bounds(point) {
            Some(self.get_mut_unchecked(point))
        } else {
            None
        }
    }

    pub fn row_unchecked(&self, row_num: usize) -> &[T] {
        let start = row_num * self.n_cols;
        let end = start + self.n_cols;
        &self.vec[start..end]
    }

    pub fn row_mut_unchecked(&mut self, row_num: usize) -> &mut [T] {
        let start = row_num * self.n_cols;
        let end = start + self.n_cols;
        &mut self.vec[start..end]
    }

    pub fn row(&self, row_num: usize) -> Option<&[T]> {
        if row_num < self.n_rows {
            Some(self.row_unchecked(row_num))
        } else {
            None
        }
    }

    pub fn row_mut(&mut self, row_num: usize) -> Option<&mut [T]> {
        if row_num < self.n_rows {
            Some(self.row_mut_unchecked(row_num))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn rows(&self) -> RowIterator<T> {
        RowIterator::new(self)
    }

    pub fn map_row_unchecked(&mut self, row_num: usize, map: impl Fn(&T) -> T) {
        for col_num in 0..self.n_cols() {
            let index = self.index(GridPoint2D::new(row_num, col_num));
            self.vec[index] = map(&self.vec[index]);
        }
    }
}

impl<T> Grid2D<T>
where
    T: Copy,
{
    pub fn swap_rows(&mut self, row_a: usize, row_b: usize) {
        if row_a > row_b {
            return self.swap_rows(row_b, row_a);
        }

        assert!(row_a < self.n_rows());
        assert!(row_b < self.n_rows());
        assert_ne!(row_a, row_b);

        let start_index_first = self.index(GridPoint2D::new(row_a, 0));
        let start_index_second = self.index(GridPoint2D::new(row_b, 0));
        let (_, first) = self.vec.split_at_mut(start_index_first);
        let (first, rest) = first.split_at_mut(self.n_cols);

        let already_split = start_index_first + self.n_cols;

        let (_, second) = rest.split_at_mut(start_index_second - already_split);
        let (second, _) = second.split_at_mut(self.n_cols);

        first.swap_with_slice(second);
    }
}

impl<T> Display for Grid2D<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.n_rows {
            for item in self.row_unchecked(row) {
                item.fmt(f)?;
            }

            if row < self.n_rows - 1 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

pub struct RowIterator<'a, T> {
    grid: &'a Grid2D<T>,
    current_row: usize,
}

impl<'a, T> RowIterator<'a, T> {
    #[allow(dead_code)]
    fn new(grid: &'a Grid2D<T>) -> Self {
        Self {
            grid,
            current_row: 0,
        }
    }
}

impl<'a, T> Iterator for RowIterator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.grid.row(self.current_row);
        if row.is_some() {
            self.current_row += 1;
        }

        row
    }
}
