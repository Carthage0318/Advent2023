use crate::data_structures::{Direction, Grid2D, GridPoint2D};
use crate::AdventErr::{Compute, InputParse};
use crate::{parser, utils, AdventErr, AdventResult};
use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::fs::File;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let trail_map = parser::as_grid2d_by_char(&mut input_file, Tile::try_from)?;

    // Part 1
    utils::part_header(1);
    part_1(&trail_map)?;

    // Part 2
    utils::part_header(2);
    part_2(&trail_map)?;

    Ok(())
}

fn part_1(trail_map: &Grid2D<Tile>) -> AdventResult<()> {
    let outputs = build_graph(trail_map, true)?;

    let longest_hike = max_cost_acyclic(&outputs, 1)?;

    println!("Steps in longest hike: {longest_hike}");

    Ok(())
}

fn part_2(trail_map: &Grid2D<Tile>) -> AdventResult<()> {
    let cyclic_outputs = build_graph(trail_map, false)?;

    let longest_hike = max_cost_cyclic(&cyclic_outputs, 0, 1)?;

    println!("Steps in longest hike: {longest_hike}");

    Ok(())
}

fn max_cost_acyclic(outputs: &[Vec<Edge>], end_node: usize) -> AdventResult<u64> {
    if outputs.len() < 2 {
        return Err(Compute(String::from("Not enough nodes")));
    }

    let sorted_nodes = topological_sort(&outputs, 0)?;
    let mut cost_to_reach = vec![0; outputs.len()];

    for current_node_id in sorted_nodes {
        let Some(&cost_to_get_here) = cost_to_reach.get(current_node_id) else {
            return Err(Compute(String::from(
                "Tried to path through out-of-bounds node",
            )));
        };

        if current_node_id == end_node {
            return Ok(cost_to_get_here);
        }

        for output in &outputs[current_node_id] {
            let Some(cost_to_destination) = cost_to_reach.get_mut(output.destination) else {
                return Err(Compute(String::from(
                    "Tried to path through out-of-bounds node",
                )));
            };

            *cost_to_destination = max(*cost_to_destination, cost_to_get_here + output.length);
        }
    }

    Err(Compute(String::from("Failed to find path to end node")))
}

fn max_cost_cyclic(outputs: &[Vec<Edge>], start_node: usize, end_node: usize) -> AdventResult<u64> {
    fn visit(
        node_id: usize,
        cost_so_far: u64,
        outputs: &[Vec<Edge>],
        end_node: usize,
        visited: &mut [bool],
    ) -> AdventResult<u64> {
        if node_id == end_node {
            return Ok(cost_so_far);
        }

        *visited.get_mut(node_id).unwrap() = true;
        let mut longest_path = 0;
        for edge in outputs.get(node_id).unwrap() {
            if !*visited.get(edge.destination).ok_or_else(|| {
                Compute(String::from(
                    "Tried to check visited for out-of-bounds node",
                ))
            })? {
                longest_path = max(
                    longest_path,
                    visit(
                        edge.destination,
                        cost_so_far + edge.length,
                        outputs,
                        end_node,
                        visited,
                    )?,
                );
            }
        }
        *visited.get_mut(node_id).unwrap() = false;

        Ok(longest_path)
    }

    if outputs.get(start_node).is_none() {
        return Err(Compute(String::from("Invalid start node")));
    }

    let mut visited = vec![false; outputs.len()];
    let longest_path = visit(start_node, 0, outputs, end_node, &mut visited)?;
    if longest_path > 0 {
        Ok(longest_path)
    } else {
        Err(Compute(String::from("Failed to find path to end node")))
    }
}

fn topological_sort(outputs: &[Vec<Edge>], start_node: usize) -> AdventResult<Vec<usize>> {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum Mark {
        None,
        Temporary,
        Permanent,
    }

    let mut result = Vec::new();
    let mut markings = vec![Mark::None; outputs.len()];

    fn visit(
        node_id: usize,
        outputs: &[Vec<Edge>],
        markings: &mut Vec<Mark>,
        result: &mut Vec<usize>,
    ) -> AdventResult<()> {
        let Some(mark) = markings.get(node_id) else {
            return Err(Compute(String::from("Tried to visit out-of-bounds node")));
        };

        match mark {
            Mark::Permanent => Ok(()),
            Mark::Temporary => Err(Compute(String::from("Graph has a cycle"))),
            Mark::None => {
                markings[node_id] = Mark::Temporary;
                for output in &outputs[node_id] {
                    visit(output.destination, outputs, markings, result)?;
                }
                markings[node_id] = Mark::Permanent;
                result.push(node_id);
                Ok(())
            }
        }
    }

    visit(start_node, outputs, &mut markings, &mut result)?;

    result.reverse();
    Ok(result)
}

fn build_graph(trail_map: &Grid2D<Tile>, one_way_slopes: bool) -> AdventResult<Vec<Vec<Edge>>> {
    if trail_map.n_rows() < 2 || trail_map.n_cols() < 1 {
        return Err(Compute(String::from("Trail map is too small")));
    }

    let mut outputs = Vec::new();
    let mut point_to_node = HashMap::new();
    let mut queue = VecDeque::new();

    // Enter and exit nodes get indices 0 and 1
    let Some(enter_col) = trail_map
        .row_unchecked(0)
        .iter()
        .position(|&c| c == Tile::Path)
    else {
        return Err(Compute(String::from("Failed to find start tile")));
    };
    point_to_node.insert(GridPoint2D::new(0, enter_col), outputs.len());
    outputs.push(vec![]);
    queue.push_back(GridPoint2D::new(0, enter_col));

    let Some(exit_col) = trail_map
        .row_unchecked(trail_map.n_rows() - 1)
        .iter()
        .position(|&c| c == Tile::Path)
    else {
        return Err(Compute(String::from("Failed to find end tile")));
    };
    point_to_node.insert(
        GridPoint2D::new(trail_map.n_rows() - 1, exit_col),
        outputs.len(),
    );
    outputs.push(vec![]);

    while let Some(source_node_point) = queue.pop_front() {
        let Some(&source_node_id) = point_to_node.get(&source_node_point) else {
            return Err(Compute(String::from(
                "Tried to explore from unrecorded node",
            )));
        };

        // Walk the path in each direction from this node to find its outputs
        for exit_direction in Direction::ALL {
            let Some(next_point) = source_node_point.move_direction(exit_direction) else {
                continue;
            };

            let Some(next_tile) = trail_map.get(next_point) else {
                continue;
            };

            if !next_tile.is_valid_step_onto(exit_direction, one_way_slopes) {
                continue;
            }

            if let Some((destination_node_point, steps_taken)) =
                walk_path(trail_map, one_way_slopes, next_point, 1, exit_direction)
            {
                // We found a connection to another node!
                let destination_node_id = *point_to_node
                    .entry(destination_node_point)
                    .or_insert_with(|| {
                        outputs.push(vec![]);
                        queue.push_back(destination_node_point);
                        outputs.len() - 1
                    });

                outputs[source_node_id].push(Edge::new(destination_node_id, steps_taken));
            }
        }
    }

    Ok(outputs)
}

/// If the walk reaches another node (point with more than one valid exit), returns Some
/// with that point and the number of steps it took to get there.
fn walk_path(
    trail_map: &Grid2D<Tile>,
    one_way_slopes: bool,
    current_point: GridPoint2D,
    steps_so_far: u64,
    last_step_direction: Direction,
) -> Option<(GridPoint2D, u64)> {
    match path_exits(
        trail_map,
        one_way_slopes,
        current_point,
        last_step_direction,
    ) {
        ValidExits::Zero => {
            // Dead end, but might be the end point
            if current_point.row == trail_map.n_rows() - 1 {
                Some((current_point, steps_so_far))
            } else {
                None
            }
        }

        ValidExits::One(next_direction, next_point) => {
            // The path continues
            walk_path(
                trail_map,
                one_way_slopes,
                next_point,
                steps_so_far + 1,
                next_direction,
            )
        }

        ValidExits::Many => {
            // This point is a node. Path complete.
            Some((current_point, steps_so_far))
        }
    }
}

fn path_exits(
    trail_map: &Grid2D<Tile>,
    one_way_slopes: bool,
    point: GridPoint2D,
    entry_direction: Direction,
) -> ValidExits {
    let mut last_exit_direction = Direction::Up;
    let mut last_exit_point = GridPoint2D::new(0, 0);
    let mut exits_found: u8 = 0;
    for exit_direction in Direction::ALL {
        if exit_direction == entry_direction.reverse() {
            continue;
        }

        let Some(next_point) = point.move_direction(exit_direction) else {
            continue;
        };

        let Some(&next_tile) = trail_map.get(next_point) else {
            continue;
        };

        if next_tile.is_valid_step_onto(exit_direction, one_way_slopes) {
            exits_found += 1;
            if exits_found > 1 {
                return ValidExits::Many;
            }

            last_exit_direction = exit_direction;
            last_exit_point = next_point;
        }
    }

    if exits_found == 0 {
        ValidExits::Zero
    } else {
        ValidExits::One(last_exit_direction, last_exit_point)
    }
}

#[derive(Debug, Copy, Clone)]
enum ValidExits {
    Zero,
    One(Direction, GridPoint2D), // That's what makes you beautiful
    Many,
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    destination: usize,
    length: u64,
}

impl Edge {
    fn new(destination: usize, length: u64) -> Self {
        Self {
            destination,
            length,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl Tile {
    fn is_valid_step_onto(self, direction: Direction, one_way_slopes: bool) -> bool {
        match self {
            Self::Path => true,
            Self::Forest => false,
            Self::Slope(slope) => !one_way_slopes || slope == direction,
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = AdventErr;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            '^' => Tile::Slope(Direction::Up),
            '>' => Tile::Slope(Direction::Right),
            'v' => Tile::Slope(Direction::Down),
            '<' => Tile::Slope(Direction::Left),
            c => return Err(InputParse(format!("Unrecognized character '{c}'"))),
        })
    }
}
