use crate::data_structures::{Grid2D, GridPoint2D};
use crate::AdventErr::InputParse;
use crate::{parser, utils, AdventErr, AdventResult};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs::File;
use std::str::FromStr;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let mut bricks = parser::as_vec_by_line(&mut input_file, line_parser)?;

    // Part 1
    utils::part_header(1);
    part_1(&mut bricks);

    Ok(())
}

fn part_1(bricks: &mut [Brick]) {
    let support_structures = drop_bricks(bricks);

    let removable_bricks = (0..bricks.len())
        .filter(|&brick_id| {
            support_structures[brick_id]
                .above
                .iter()
                .all(|&above_brick_id| support_structures[above_brick_id].below.len() > 1)
        })
        .count();

    println!("Bricks safe to disintegrate: {removable_bricks}");
}

fn drop_bricks(bricks: &mut [Brick]) -> Vec<SupportStructure> {
    let mut support_structures = vec![SupportStructure::default(); bricks.len()];

    let max_extents = bricks.iter().fold(GridPoint2D::new(0, 0), |acc, brick| {
        GridPoint2D::new(max(acc.row, brick.max_x()), max(acc.col, brick.max_y()))
    });

    let mut highest_brick = Grid2D::new(max_extents.row + 1, max_extents.col + 1, None);

    // Sort ids of bricks based on their current minimum z
    let mut sorted_ids: Vec<_> = (0..bricks.len()).collect();
    sorted_ids.sort_unstable_by_key(|&brick_id| bricks[brick_id].min_z());

    let mut bricks_below: HashSet<usize> = HashSet::new();
    for current_brick_id in sorted_ids {
        bricks_below.extend(
            bricks[current_brick_id]
                .footprint()
                .filter_map(|point| *highest_brick.get_unchecked(point)),
        );

        // Get the highest thing below this brick,
        // then move it down to 1 above that.
        let max_height_below = bricks_below
            .iter()
            .map(|&below_brick_id| bricks[below_brick_id].max_z())
            .max()
            .unwrap_or(0);
        bricks.get_mut(current_brick_id).unwrap().start_point.z = max_height_below + 1;

        // For each of the bricks which were are now directly below us,
        // add them to the support structure
        for &below_brick_id in &bricks_below {
            if bricks[below_brick_id].max_z() == max_height_below {
                support_structures
                    .get_mut(below_brick_id)
                    .unwrap()
                    .above
                    .push(current_brick_id);
                support_structures
                    .get_mut(current_brick_id)
                    .unwrap()
                    .below
                    .push(below_brick_id);
            }
        }

        // Update the highest brick
        for point in bricks[current_brick_id].footprint() {
            *highest_brick.get_mut_unchecked(point) = Some(current_brick_id);
        }

        bricks_below.clear()
    }

    support_structures
}

#[derive(Debug, Clone)]
struct SupportStructure {
    below: Vec<usize>,
    above: Vec<usize>,
}

impl Default for SupportStructure {
    fn default() -> Self {
        Self {
            below: vec![],
            above: vec![],
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Brick {
    start_point: Point3D,
    axis: Axis,
    cubes: usize,
}

impl Brick {
    fn max_x(self) -> usize {
        match self.axis {
            Axis::X => self.start_point.x + self.cubes - 1,
            Axis::Y | Axis::Z => self.start_point.x,
        }
    }

    fn max_y(self) -> usize {
        match self.axis {
            Axis::Y => self.start_point.y + self.cubes - 1,
            Axis::X | Axis::Z => self.start_point.y,
        }
    }

    fn max_z(self) -> usize {
        match self.axis {
            Axis::Z => self.start_point.z + self.cubes - 1,
            Axis::X | Axis::Y => self.start_point.z,
        }
    }

    fn min_z(self) -> usize {
        self.start_point.z
    }

    fn footprint(self) -> BrickFootprintIterator {
        BrickFootprintIterator {
            current_point: self.start_point.discard_z(),
            cubes: if self.axis == Axis::Z { 1 } else { self.cubes },
            step: GridPoint2D::new(
                if self.axis == Axis::X { 1 } else { 0 },
                if self.axis == Axis::Y { 1 } else { 0 },
            ),
        }
    }
}

#[derive(Debug)]
struct BrickFootprintIterator {
    current_point: GridPoint2D,
    step: GridPoint2D,
    cubes: usize,
}

impl Iterator for BrickFootprintIterator {
    type Item = GridPoint2D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cubes == 0 {
            None
        } else {
            self.cubes -= 1;
            let result = self.current_point;
            self.current_point += self.step;
            Some(result)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

impl Point3D {
    fn discard_z(self) -> GridPoint2D {
        GridPoint2D {
            row: self.x,
            col: self.y,
        }
    }
}

lazy_static! {
    static ref POINT_REGEX: Regex = Regex::new(r"(?<x>\d+),(?<y>\d+),(?<z>\d+)").unwrap();
}

impl FromStr for Point3D {
    type Err = AdventErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(caps) = POINT_REGEX.captures(s) else {
            return Err(InputParse(format!("Failed to parse point: '{s}'")));
        };

        let x = caps["x"]
            .parse()
            .map_err(|_| InputParse(format!("Failed to parse x: '{s}'")))?;
        let y = caps["y"]
            .parse()
            .map_err(|_| InputParse(format!("Failed to parse y: '{s}'")))?;
        let z = caps["z"]
            .parse()
            .map_err(|_| InputParse(format!("Failed to parse z: '{s}'")))?;

        Ok(Point3D { x, y, z })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Axis {
    X,
    Y,
    Z,
}

fn line_parser(line: &str) -> AdventResult<Brick> {
    let Some((first, second)) = line.split_once('~') else {
        return Err(InputParse(format!("Failed to split line: '{line}'")));
    };

    let first = Point3D::from_str(first)?;
    let second = Point3D::from_str(second)?;

    // Make sure at most one axis is different.
    let mut axis = None;
    let mut cubes = 0;
    if first.x != second.x {
        axis = Some(Axis::X);
        cubes = first.x.abs_diff(second.x) + 1;
    }
    if first.y != second.y {
        if axis.is_some() {
            return Err(InputParse(format!(
                "Malformed input - brick spans multiple axes: '{line}'"
            )));
        }
        axis = Some(Axis::Y);
        cubes = first.y.abs_diff(second.y) + 1;
    }
    if first.z != second.z {
        if axis.is_some() {
            return Err(InputParse(format!(
                "Malformed input - brick spans multiple axes: '{line}'"
            )));
        }
        axis = Some(Axis::Z);
        cubes = first.z.abs_diff(second.z) + 1;
    }

    let axis = axis.unwrap_or(Axis::Z);
    cubes = max(cubes, 1);

    Ok(Brick {
        start_point: min(first, second),
        axis,
        cubes,
    })
}
