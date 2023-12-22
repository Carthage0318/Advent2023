use crate::data_structures::{Grid2D, GridPoint2D};
use crate::days::day_21::Tile;
use std::collections::VecDeque;

pub fn count_visitable_finite(
    reference_grid: &Grid2D<Tile>,
    starting_position: GridPoint2D,
    total_steps: u64,
) -> u64 {
    let steps_grid = create_step_grid(reference_grid, starting_position);

    steps_grid
        .cells()
        .filter(|&&x| match x {
            Some(steps_required) => {
                steps_required <= total_steps && steps_required % 2 == total_steps % 2
            }
            None => false,
        })
        .count() as u64
}

pub fn count_visitable_infinite(
    reference_grid: &Grid2D<Tile>,
    starting_position: GridPoint2D,
    total_steps: u64,
) -> u64 {
    // Count how many cells are visitable within the starting instance
    let mut total_visitable =
        count_visitable_finite(reference_grid, starting_position, total_steps);

    // Now count how many are visitable in one quarter (lower right)
    total_visitable += count_visitable_quarter(reference_grid, starting_position, total_steps);

    // Now rotate 3 times, compute the remaining quarter for each
    let rotated_grid = reference_grid.rotate_counterclockwise();
    total_visitable += count_visitable_quarter(&rotated_grid, starting_position, total_steps);

    let rotated_grid = rotated_grid.rotate_counterclockwise();
    total_visitable += count_visitable_quarter(&rotated_grid, starting_position, total_steps);

    let rotated_grid = rotated_grid.rotate_counterclockwise();
    total_visitable += count_visitable_quarter(&rotated_grid, starting_position, total_steps);

    total_visitable
}

fn count_visitable_quarter(
    reference_grid: &Grid2D<Tile>,
    starting_position: GridPoint2D,
    total_steps: u64,
) -> u64 {
    count_visitable_horiz_axis_right(reference_grid, starting_position, total_steps)
        + count_visitable_fourth_quadrant(reference_grid, starting_position, total_steps)
}

fn count_visitable_fourth_quadrant(
    reference_grid: &Grid2D<Tile>,
    starting_position: GridPoint2D,
    total_steps: u64,
) -> u64 {
    // Below positive horizontal axis (not including tiles directly along vertical axis)

    let steps_to_first_instance = ((reference_grid.n_cols() - starting_position.col)
        + (reference_grid.n_rows() - starting_position.row))
        as u64;
    if steps_to_first_instance > total_steps {
        // You'll never make it off the starting instance. We're done here.
        return 0;
    }

    let steps_on_axis_right = total_steps - steps_to_first_instance;
    // Generate the step grid when entering from the top left corner.
    let step_grid = create_step_grid(reference_grid, GridPoint2D::new(0, 0));
    let visitable = count_visitable_with_skipping(&step_grid, steps_on_axis_right);

    // Compute how many even/odd parity spots there are.
    let (full_grid_even_count, full_grid_odd_count) = count_parity_cells(&step_grid);

    // Sum up multiple rows of the skipped instances.
    // This can be solved via triangular numbers.
    let total_visitable_skipped = match (visitable.parity_flips, visitable.first_skipped_is_even) {
        (false, true) => {
            // All are even
            let total_skipped_instances =
                (visitable.skipped_even_instances * (visitable.skipped_even_instances + 1)) / 2;
            total_skipped_instances * full_grid_even_count
        }
        (false, false) => {
            // All are odd
            let total_skipped_instances =
                (visitable.skipped_odd_instances * (visitable.skipped_odd_instances + 1)) / 2;
            total_skipped_instances * full_grid_odd_count
        }
        (true, true) => {
            // Alternating. Even is first.
            let total_even_skipped = visitable.skipped_even_instances.pow(2);
            let total_odd_skipped =
                visitable.skipped_odd_instances * (visitable.skipped_odd_instances + 1);
            total_even_skipped * full_grid_even_count + total_odd_skipped * full_grid_odd_count
        }
        (true, false) => {
            // Alternating. Odd is first.
            let total_even_skipped =
                visitable.skipped_even_instances * (visitable.skipped_even_instances + 1);
            let total_odd_skipped = visitable.skipped_odd_instances.pow(2);
            total_even_skipped * full_grid_even_count + total_odd_skipped * full_grid_odd_count
        }
    };

    let instances_skipped_first_row =
        visitable.skipped_even_instances + visitable.skipped_odd_instances;
    let total_visitable_unskipped = match visitable.unskipped_instances {
        None => 0,
        Some(ref unskipped_counts) => unskipped_counts
            .iter()
            .enumerate()
            .map(|(i, &visitable_per_instance)| {
                let instance_count = instances_skipped_first_row + i as u64 + 1;
                instance_count * visitable_per_instance
            })
            .sum(),
    };

    total_visitable_skipped + total_visitable_unskipped
}

fn count_visitable_horiz_axis_right(
    reference_grid: &Grid2D<Tile>,
    starting_position: GridPoint2D,
    total_steps: u64,
) -> u64 {
    // First, let's consume the number of steps required to get to the first instance to the right.
    let steps_to_first_instance = (reference_grid.n_cols() - starting_position.col) as u64;
    if steps_to_first_instance > total_steps {
        // You'll never make it off the starting instance. We're done here.
        return 0;
    }

    let steps_on_axis_right = total_steps - steps_to_first_instance;
    // Let's generate the step grid when entering from this point - we'll need it.
    let step_grid = create_step_grid(reference_grid, GridPoint2D::new(starting_position.row, 0));
    let visitable = count_visitable_with_skipping(&step_grid, steps_on_axis_right);

    // Compute how many even/odd parity spots there are.
    let (full_grid_even_count, full_grid_odd_count) = count_parity_cells(&step_grid);

    visitable.skipped_even_instances * full_grid_even_count
        + visitable.skipped_odd_instances * full_grid_odd_count
        + visitable
            .unskipped_instances
            .map_or(0, |counts| counts.iter().sum())
}

fn count_visitable_with_skipping(
    step_grid: &Grid2D<Option<u64>>,
    available_steps: u64,
) -> VisitableWithSkipping {
    // We can skip over all instances where we are able to reach everything.
    // So let's find out how many times we can do that.
    let steps_needed_for_full_grid = step_grid.cells().filter_map(|&cell| cell).max().unwrap();

    // Count the number of instances we're jumping over.
    let steps_to_cross = step_grid.n_cols() as u64;
    let skipped_instances =
        skippable_instances(available_steps, steps_needed_for_full_grid, steps_to_cross);
    // Check parity of these instances.
    let first_instance_is_even = available_steps % 2 == 0;
    let parity_flips = steps_to_cross % 2 == 1;
    let (skipped_even_instances, skipped_odd_instances) =
        partition_skipped_instances(skipped_instances, parity_flips, first_instance_is_even);

    // Now, let's handle the instances after we're done skipping
    // We started already inside a skipped instance, so we need to account for that 1 step not taken
    let steps_used_inside_skipped_instances = if skipped_instances == 0 {
        0
    } else {
        skipped_instances * (steps_to_cross - 1) + skipped_instances - 1
    };
    let steps_remaining_after_skipped_instances =
        available_steps - steps_used_inside_skipped_instances;

    if steps_remaining_after_skipped_instances == 0 {
        // Skipped instances covered the axis exactly. Wow!
        // (This is only possible if there's one row... so not exactly a likely scenario.)
        // We're done. Just return the values from the skipped grids.
        return VisitableWithSkipping {
            skipped_even_instances,
            skipped_odd_instances,
            parity_flips,
            first_skipped_is_even: first_instance_is_even,
            unskipped_instances: None,
        };
    }

    // Consume 1 step to move into the next instance
    let mut steps_remaining = steps_remaining_after_skipped_instances - 1;
    let mut unskipped_visitable_cells = vec![];
    loop {
        let current_parity = steps_remaining % 2;
        unskipped_visitable_cells.push(
            step_grid
                .cells()
                .filter(|&&cell| match cell {
                    Some(steps_required) => {
                        steps_required <= steps_remaining && steps_required % 2 == current_parity
                    }
                    None => false,
                })
                .count() as u64,
        );

        if steps_remaining < steps_to_cross {
            break;
        }

        steps_remaining -= steps_to_cross;
    }

    VisitableWithSkipping {
        skipped_even_instances,
        skipped_odd_instances,
        parity_flips,
        first_skipped_is_even: first_instance_is_even,
        unskipped_instances: Some(unskipped_visitable_cells),
    }
}

#[derive(Debug)]
struct VisitableWithSkipping {
    skipped_even_instances: u64,
    skipped_odd_instances: u64,
    parity_flips: bool,
    first_skipped_is_even: bool,
    unskipped_instances: Option<Vec<u64>>,
}

fn skippable_instances(
    steps_available: u64,
    steps_to_cover: u64,
    steps_to_next_instance: u64,
) -> u64 {
    // number of steps we can afford to spend walking straight before we don't have enough to cover,
    // divided by steps to enter next instance.
    // This leaves us at the start of or inside the last instance we can cover, so add 1.
    if steps_available < steps_to_next_instance {
        0
    } else {
        ((steps_available - steps_to_cover) / steps_to_next_instance) + 1
    }
}

/// Returns (evens, odds)
fn partition_skipped_instances(
    skipped_instances: u64,
    parity_flips: bool,
    first_instance_is_even: bool,
) -> (u64, u64) {
    let skipped_even_instances = match (parity_flips, first_instance_is_even) {
        (false, true) => skipped_instances,
        (false, false) => 0,
        (true, true) => skipped_instances - skipped_instances / 2, // evens get the extra one, if total is odd
        (true, false) => skipped_instances / 2, // odds get the extra one, if total is odd
    };
    let skipped_odd_instances = skipped_instances - skipped_even_instances;

    (skipped_even_instances, skipped_odd_instances)
}

/// Returns (evens, odds)
fn count_parity_cells(step_grid: &Grid2D<Option<u64>>) -> (u64, u64) {
    // Compute how many even/odd parity spots there are.
    let full_grid_even_count = step_grid
        .cells()
        .filter(|&&cell| match cell {
            Some(steps) => steps % 2 == 0,
            None => false,
        })
        .count() as u64;
    let full_grid_odd_count = step_grid
        .cells()
        .filter(|&&cell| match cell {
            Some(steps) => steps % 2 == 1,
            None => false,
        })
        .count() as u64;

    (full_grid_even_count, full_grid_odd_count)
}

fn create_step_grid(
    reference_grid: &Grid2D<Tile>,
    starting_position: GridPoint2D,
) -> Grid2D<Option<u64>> {
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

    steps_grid
}
