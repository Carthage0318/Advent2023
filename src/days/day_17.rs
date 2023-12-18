use crate::data_structures::{Direction, Grid2D, GridPoint2D};
use crate::days::day_17::Direction::{Down, Left, Right, Up};
use crate::AdventErr::{Compute, InputParse};
use crate::{parser, utils, AdventResult};
use std::cmp;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let city = parser::as_grid2d_by_char(&mut input_file, |c| {
        c.to_digit(10)
            .map(|x| x as u8)
            .ok_or_else(|| InputParse(format!("Unrecognized character '{c}'")))
    })?;

    // Part 1
    utils::part_header(1);
    part_1(&city)?;

    // Part 2
    utils::part_header(2);
    part_2(&city)?;

    Ok(())
}

fn part_1(cost_grid: &Grid2D<u8>) -> AdventResult<()> {
    let min_heat_loss = least_cost(
        cost_grid,
        GridPoint2D::new(0, 0),
        GridPoint2D::new(cost_grid.n_rows() - 1, cost_grid.n_cols() - 1),
        0,
        3,
    )?;

    println!("Minimum heat loss: {min_heat_loss}");

    Ok(())
}

fn part_2(cost_grid: &Grid2D<u8>) -> AdventResult<()> {
    let min_heat_loss = least_cost(
        cost_grid,
        GridPoint2D::new(0, 0),
        GridPoint2D::new(cost_grid.n_rows() - 1, cost_grid.n_cols() - 1),
        4,
        10,
    )?;

    println!("Minimum heat loss: {min_heat_loss}");

    Ok(())
}

fn least_cost(
    entry_cost: &Grid2D<u8>,
    start: GridPoint2D,
    destination: GridPoint2D,
    min_steps_before_turn_or_stop: usize,
    max_steps_in_line: usize,
) -> AdventResult<u64> {
    assert!(entry_cost.n_rows() > 0 && entry_cost.n_cols() > 0);
    if start == destination {
        return Ok(0);
    }

    let min_steps_before_turn_or_stop = cmp::max(min_steps_before_turn_or_stop, 1);
    let max_steps_in_line = cmp::max(max_steps_in_line, 1);

    let mut search_grid = Grid2D::new(entry_cost.n_rows(), entry_cost.n_cols(), VisitedCell::new());
    *search_grid.get_mut_unchecked(start) = VisitedCell {
        to_up: CopyRange::new(0, usize::MAX),
        to_down: CopyRange::new(0, usize::MAX),
        to_left: CopyRange::new(0, usize::MAX),
        to_right: CopyRange::new(0, usize::MAX),
    };

    let mut queue = BinaryHeap::new();

    let mut init_direction = |direction: Direction| {
        if let Some(point) = start.move_direction(direction) {
            if let Some(&cost) = entry_cost.get(point) {
                queue.push(SearchElement::new(direction, 1, point, cost as u64))
            }
        }
    };

    for direction in [Up, Down, Left, Right] {
        init_direction(direction);
    }

    let worth_exploring = |seen_range: CopyRange<usize>, steps_so_far| {
        steps_so_far < seen_range.start
            || (steps_so_far > seen_range.end && seen_range.end < max_steps_in_line - 1)
    };

    while let Some(current_element) = queue.pop() {
        if current_element.point == destination
            && current_element.continuous_steps >= min_steps_before_turn_or_stop
        {
            return Ok(current_element.cost);
        }

        let mut explore_direction = |direction, steps_so_far_this_direction| {
            let visited = search_grid.get_mut_unchecked(current_element.point);
            if !worth_exploring(visited.get(direction), steps_so_far_this_direction) {
                return;
            }

            visited.mark_visited(direction, steps_so_far_this_direction);
            let Some(next_point) = current_element.point.move_direction(direction) else {
                return;
            };

            let Some(&next_cost) = entry_cost.get(next_point) else {
                return;
            };

            queue.push(SearchElement::new(
                direction,
                steps_so_far_this_direction + 1,
                next_point,
                current_element.cost + next_cost as u64,
            ));
        };

        if current_element.continuous_steps < max_steps_in_line {
            explore_direction(current_element.direction, current_element.continuous_steps);
        }
        if current_element.continuous_steps >= min_steps_before_turn_or_stop {
            explore_direction(current_element.direction.turn_left(), 0);
            explore_direction(current_element.direction.turn_right(), 0);
        }
    }

    Err(Compute(format!(
        "Failed to find any path from {start} to {destination}"
    )))
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct CopyRange<T: Copy> {
    start: T,
    end: T,
}

impl<T: Copy> CopyRange<T> {
    fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Copy, Clone)]
struct VisitedCell {
    to_up: CopyRange<usize>,
    to_down: CopyRange<usize>,
    to_left: CopyRange<usize>,
    to_right: CopyRange<usize>,
}

impl VisitedCell {
    fn new() -> Self {
        Self {
            to_up: CopyRange::new(usize::MAX, 0),
            to_down: CopyRange::new(usize::MAX, 0),
            to_left: CopyRange::new(usize::MAX, 0),
            to_right: CopyRange::new(usize::MAX, 0),
        }
    }

    fn get(self, direction: Direction) -> CopyRange<usize> {
        match direction {
            Up => self.to_up,
            Down => self.to_down,
            Left => self.to_left,
            Right => self.to_right,
        }
    }

    fn mark_visited(&mut self, direction: Direction, steps_so_far_this_direction: usize) {
        let range = match direction {
            Up => &mut self.to_up,
            Down => &mut self.to_down,
            Left => &mut self.to_left,
            Right => &mut self.to_right,
        };

        range.start = cmp::min(range.start, steps_so_far_this_direction);
        range.end = cmp::max(range.end, steps_so_far_this_direction);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct SearchElement {
    direction: Direction,
    continuous_steps: usize,
    point: GridPoint2D,
    cost: u64,
}

impl SearchElement {
    fn new(direction: Direction, continuous_steps: usize, point: GridPoint2D, cost: u64) -> Self {
        Self {
            direction,
            continuous_steps,
            point,
            cost,
        }
    }
}

impl PartialOrd for SearchElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for SearchElement {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
