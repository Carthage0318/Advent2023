use crate::data_structures::{Direction, Grid2D, GridPoint2D};
use crate::days::day_17::Direction::{Down, Left, Right, Up};
use crate::AdventErr::{Compute, InputParse};
use crate::{parser, utils, AdventResult};
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

    Ok(())
}

const MAX_CONTINUOUS_DIRECTION: usize = 3;

fn part_1(cost_grid: &Grid2D<u8>) -> AdventResult<()> {
    let min_heat_loss = least_cost(
        cost_grid,
        GridPoint2D::new(0, 0),
        GridPoint2D::new(cost_grid.n_rows() - 1, cost_grid.n_cols() - 1),
    )?;

    println!("Minimum heat loss: {min_heat_loss}");

    Ok(())
}

fn least_cost(
    entry_cost: &Grid2D<u8>,
    start: GridPoint2D,
    destination: GridPoint2D,
) -> AdventResult<u64> {
    assert!(entry_cost.n_rows() > 0 && entry_cost.n_cols() > 0);
    if start == destination {
        return Ok(0);
    }

    let mut search_grid = Grid2D::new(entry_cost.n_rows(), entry_cost.n_cols(), VisitedCell::new());
    *search_grid.get_mut_unchecked(start) = VisitedCell {
        to_up: 0,
        to_down: 0,
        to_left: 0,
        to_right: 0,
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

    while let Some(current_element) = queue.pop() {
        if current_element.point == destination {
            return Ok(current_element.cost);
        }

        let mut explore_direction = |direction: Direction, steps_so_far_this_direction: u8| {
            let visited = search_grid.get_mut_unchecked(current_element.point);
            if visited.get(direction) <= steps_so_far_this_direction as usize {
                return;
            }

            visited.mark_visited(direction, steps_so_far_this_direction as usize);
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

        if current_element.continuous_steps < MAX_CONTINUOUS_DIRECTION as u8 {
            explore_direction(current_element.direction, current_element.continuous_steps);
        }
        explore_direction(current_element.direction.turn_left(), 0);
        explore_direction(current_element.direction.turn_right(), 0);
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

#[derive(Debug, Copy, Clone)]
struct VisitedCell {
    to_up: usize,
    to_down: usize,
    to_left: usize,
    to_right: usize,
}

impl VisitedCell {
    fn new() -> Self {
        Self {
            to_up: usize::MAX,
            to_down: usize::MAX,
            to_left: usize::MAX,
            to_right: usize::MAX,
        }
    }

    fn get(self, direction: Direction) -> usize {
        match direction {
            Up => self.to_up,
            Down => self.to_down,
            Left => self.to_left,
            Right => self.to_right,
        }
    }

    fn mark_visited(&mut self, direction: Direction, steps_so_far_this_direction: usize) {
        match direction {
            Up => self.to_up = steps_so_far_this_direction,
            Down => self.to_down = steps_so_far_this_direction,
            Left => self.to_left = steps_so_far_this_direction,
            Right => self.to_right = steps_so_far_this_direction,
        };
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct SearchElement {
    direction: Direction,
    continuous_steps: u8,
    point: GridPoint2D,
    cost: u64,
}

impl SearchElement {
    fn new(direction: Direction, continuous_steps: u8, point: GridPoint2D, cost: u64) -> Self {
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
