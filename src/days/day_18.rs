use crate::data_structures::Direction;
use crate::AdventErr::Compute;
use crate::{parser, utils, AdventResult};
use std::cmp::{max, min, Ordering};
use std::fs::File;
use std::mem;
use types::{Instruction, ProtoBox, VerticalEdge};

mod parsing;
mod types;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let instructions = parser::as_vec_by_line(&mut input_file, parsing::line_parser)?;
    let (basic_instructions, color_instructions): (Vec<_>, Vec<_>) =
        instructions.into_iter().unzip();

    // Part 1
    utils::part_header(1);
    run_part(&basic_instructions)?;

    // Part 2
    utils::part_header(2);
    run_part(&color_instructions)?;

    Ok(())
}

fn run_part(instructions: &[Instruction]) -> AdventResult<()> {
    let trench_size = trench_size(instructions)?;

    println!("Trench size: {trench_size}");

    Ok(())
}

fn is_counterclockwise(instructions: &[Instruction]) -> bool {
    if instructions.is_empty() {
        return false;
    }

    let mut count = 0;
    let mut last_direction = instructions[0].direction;
    for instruction in &instructions[1..] {
        count += if Direction::is_left_turn(last_direction, instruction.direction) {
            1
        } else {
            -1
        };
        last_direction = instruction.direction;
    }

    count > 0
}

fn trench_size(instructions: &[Instruction]) -> AdventResult<u64> {
    if instructions.is_empty() {
        return Err(Compute(String::from("No instructions")));
    }

    let mut edges = create_edges(instructions);
    edges.sort_unstable_by(|edge_a, edge_b| match edge_a.column.cmp(&edge_b.column) {
        Ordering::Equal => edge_a.row_start.cmp(&edge_b.row_start),
        x => x,
    });

    let mut total_size = 0;
    let mut current_boxes: Vec<ProtoBox> = Vec::new();
    let mut next_boxes = Vec::new();

    for &edge in &edges {
        for &existing_box in &current_boxes {
            if existing_box.start_row > edge.row_end || existing_box.end_row < edge.row_start {
                next_boxes.push(existing_box);
                continue;
            }

            total_size += existing_box.commit(edge.column);

            let (new_boxes, additional_commit) = intersect(existing_box, edge);
            total_size += additional_commit;
            for new_box in new_boxes {
                if let Some(new_box) = new_box {
                    next_boxes.push(new_box);
                }
            }
        }

        // Now, if this is a start edge, create a new box for it.
        if edge.is_start {
            next_boxes.push(ProtoBox {
                start_column: edge.column,
                start_row: edge.row_start,
                end_row: edge.row_end,
            })
        }

        mem::swap(&mut current_boxes, &mut next_boxes);
        next_boxes.clear();
    }

    if !current_boxes.is_empty() {
        return Err(Compute(String::from("Failed to drain all boxes")));
    }

    Ok(total_size)
}

fn intersect(existing_box: ProtoBox, edge: VerticalEdge) -> ([Option<ProtoBox>; 2], u64) {
    let mut result = [None; 2];
    let mut total_committed = 0;

    if edge.is_start {
        // Only way this can happen is to have a single shared row.
        // The new edge will get the shared row, so cut one row off of our current box.

        if existing_box.start_row == edge.row_end {
            let new_box = ProtoBox {
                start_column: edge.column,
                start_row: existing_box.start_row + 1,
                end_row: existing_box.end_row,
            };
            if !new_box.is_empty() {
                result[0] = Some(new_box);
            }
        } else {
            let new_box = ProtoBox {
                start_column: edge.column,
                start_row: existing_box.start_row,
                end_row: existing_box.end_row - 1,
            };
            if !new_box.is_empty() {
                result[0] = Some(new_box);
            }
        }
    } else {
        let row_start_blocking = edge.row_start + if edge.start_corner_convex { 0 } else { 1 };
        let row_end_blocking = edge.row_end - if edge.end_corner_convex { 0 } else { 1 };

        let new_box_0 = ProtoBox {
            start_column: edge.column,
            start_row: existing_box.start_row,
            end_row: row_start_blocking - 1,
        };
        if !new_box_0.is_empty() {
            result[0] = Some(new_box_0);
        }

        let new_box_1 = ProtoBox {
            start_column: edge.column,
            start_row: row_end_blocking + 1,
            end_row: existing_box.end_row,
        };
        if !new_box_1.is_empty() {
            result[1] = Some(new_box_1);
        }

        // Add cells of the blocking portion of the edge which we hit
        let start_block_hit = max(row_start_blocking, existing_box.start_row);
        let end_block_hit = min(row_end_blocking, existing_box.end_row);
        total_committed += (end_block_hit + 1 - start_block_hit) as u64;
    }

    (result, total_committed)
}

fn create_edges(instructions: &[Instruction]) -> Vec<VerticalEdge> {
    if instructions.is_empty() {
        return vec![];
    }

    let is_counterclockwise = is_counterclockwise(instructions);

    let mut current_row = 0;
    let mut current_column = 0;
    let mut last_direction = instructions.last().unwrap().direction;

    let mut result = vec![];

    for (i, instruction) in instructions.iter().enumerate() {
        let was_left_turn = Direction::is_left_turn(last_direction, instruction.direction);
        let was_convex_corner = was_left_turn == is_counterclockwise;
        let next_instruction = instructions[(i + 1) % instructions.len()];
        let will_turn_left =
            Direction::is_left_turn(instruction.direction, next_instruction.direction);
        let will_convex_corner = will_turn_left == is_counterclockwise;
        last_direction = instruction.direction;

        match instruction.direction {
            Direction::Left => {
                current_column -= instruction.length;
            }
            Direction::Right => {
                current_column += instruction.length;
            }
            Direction::Down => {
                let end_row = current_row + instruction.length;
                let edge = VerticalEdge::new(
                    current_column,
                    current_row,
                    end_row,
                    is_counterclockwise,
                    was_convex_corner,
                    will_convex_corner,
                );
                current_row = end_row;
                result.push(edge);
            }
            Direction::Up => {
                let end_row = current_row - instruction.length;
                let edge = VerticalEdge::new(
                    current_column,
                    end_row,
                    current_row,
                    !is_counterclockwise,
                    will_convex_corner,
                    was_convex_corner,
                );
                current_row = end_row;
                result.push(edge);
            }
        }
    }

    result
}
