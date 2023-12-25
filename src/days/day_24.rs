use crate::data_structures::Grid2D;
use crate::AdventErr::{Compute, InputParse};
use crate::{math, parser, utils, AdventErr, AdventResult};
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let hailstones = parser::as_vec_by_line(&mut input_file, |line| {
        let Some((position, velocity)) = line.split_once('@') else {
            return Err(InputParse(format!("Failed to split line:\n{line}")));
        };

        Ok(Parametric {
            position: Point3D::from_str(position)?,
            velocity: Point3D::from_str(velocity)?,
        })
    })?;

    // Part 1
    utils::part_header(1);
    part_1(&hailstones)?;

    // Part 2
    utils::part_header(2);
    part_2(&hailstones)?;

    Ok(())
}

fn part_1(hailstones: &[Parametric]) -> AdventResult<()> {
    let slope_intercepts: Vec<_> = hailstones
        .iter()
        .map(|parametric| parametric.to_x_y_slope_intercept())
        .collect::<AdventResult<_>>()?;

    const MIN_BOUND: f64 = 200000000000000_f64;
    const MAX_BOUND: f64 = 400000000000000_f64;
    const ACCEPTABLE_RANGE: RangeInclusive<f64> = MIN_BOUND..=MAX_BOUND;

    let path_intersections: usize = slope_intercepts
        .iter()
        .enumerate()
        .map(|(stone_a, slope_intercept_a)| {
            slope_intercepts
                .iter()
                .enumerate()
                .skip(stone_a + 1)
                .filter(|(stone_b, slope_intercept_b)| {
                    let Some((x, y)) = slope_intercept_a.intersection(**slope_intercept_b) else {
                        return false;
                    };

                    if !ACCEPTABLE_RANGE.contains(&x) || !ACCEPTABLE_RANGE.contains(&y) {
                        return false;
                    }

                    hailstones[stone_a].time_for_x(x) >= 0_f64
                        && hailstones[*stone_b].time_for_x(x) >= 0_f64
                })
                .count()
        })
        .sum();

    println!("Intersections within test area: {path_intersections}");

    Ok(())
}

/// This solution is based on a bunch of algebra done by-hand.
/// It wasn't fun, and I don't suggest you try following it,
/// if you're reading this in the future.
fn part_2(hailstones: &[Parametric]) -> AdventResult<()> {
    if hailstones.len() < 5 {
        return Err(Compute(String::from(
            "At least 5 pieces of data required to solve",
        )));
    }

    let coefficients: Vec<_> = hailstones[..5]
        .iter()
        .map(|data| ExpressionCoefficients {
            a: -data.position.y,
            b: data.position.x,
            c: data.velocity.y,
            d: -data.velocity.x,
            g: data.position.y * data.velocity.x - data.position.x * data.velocity.y,
        })
        .collect();

    let mut a_values = Grid2D::new(4, 4, 0_f64);
    let mut b_values = [0_f64; 4];
    for row_num in 0..4 {
        let e1 = coefficients[row_num];
        let e2 = coefficients[row_num + 1];

        let row = a_values.row_mut_unchecked(row_num);
        row[0] = (e1.a - e2.a) as f64;
        row[1] = (e1.b - e2.b) as f64;
        row[2] = (e1.c - e2.c) as f64;
        row[3] = (e1.d - e2.d) as f64;
        b_values[row_num] = (e2.g - e1.g) as f64;
    }

    math::gauss_jordan(&mut a_values, &mut b_values)?;

    let v_x = b_values[0];
    let x = b_values[2];
    let y = b_values[3];

    // Compute time for first hailstone to be hit
    let t0 = (hailstones[0].position.x as f64 - x) / (v_x - hailstones[0].velocity.x as f64);
    // And second
    let t1 = (hailstones[1].position.x as f64 - x) / (v_x - hailstones[1].velocity.x as f64);
    // From this, we can quickly compute z
    let z_intersect_0 = hailstones[0].position.z as f64 + t0 * hailstones[0].velocity.z as f64;
    let z_intersect_1 = hailstones[1].position.z as f64 + t1 * hailstones[1].velocity.z as f64;
    let v_z = (z_intersect_1 - z_intersect_0) / (t1 - t0);
    let z = (z_intersect_0) - v_z * t0;

    let x = x.round() as i64;
    let y = y.round() as i64;
    let z = z.round() as i64;

    println!("Sum of rock initial position components: {}", x + y + z);

    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct Parametric {
    position: Point3D,
    velocity: Point3D,
}

impl Parametric {
    fn to_x_y_slope_intercept(self) -> AdventResult<SlopeIntercept> {
        if self.velocity.x == 0 {
            return Err(Compute(String::from(
                "x velocity is 0 - cannot compute y-x slope-intercept",
            )));
        }

        let slope = self.velocity.y as f64 / self.velocity.x as f64;
        let intercept = self.position.y as f64 - (slope * self.position.x as f64);

        Ok(SlopeIntercept { slope, intercept })
    }

    fn time_for_x(self, x: f64) -> f64 {
        (x - self.position.x as f64) / self.velocity.x as f64
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Display for Point3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Copy, Clone)]
struct SlopeIntercept {
    slope: f64,
    intercept: f64,
}

impl SlopeIntercept {
    /// Gives (x, y) position of intersection
    fn intersection(self, other: Self) -> Option<(f64, f64)> {
        if math::approximately(self.slope, other.slope) {
            return if math::approximately(self.intercept, other.intercept) {
                // These are the same line. Any point is valid.
                Some((0_f64, self.compute_y(0_f64)))
            } else {
                // These are parallel lines. No intersection.
                None
            };
        }

        let x = (other.intercept - self.intercept) / (self.slope - other.slope);
        let y = self.compute_y(x);
        Some((x, y))
    }

    fn compute_y(self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }
}

/// Expression of the form a * vx + b * vy + c * x + d * y + g
/// This is equal to x * vy - y * vx
/// where our shot is fired from (x, y, z) @ (vx, vy, vz)
/// (Assume hit at an arbitrary t and manipulate algebra to get here)
#[derive(Debug, Copy, Clone)]
struct ExpressionCoefficients {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    g: i64,
}

lazy_static! {
    static ref POINT_REGEX: Regex =
        Regex::new(r"(?<x>-?\d+),\s+(?<y>-?\d+),\s+(?<z>-?\d+)").unwrap();
}

impl FromStr for Point3D {
    type Err = AdventErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let Some(caps) = POINT_REGEX.captures(s) else {
            return Err(InputParse(format!("Failed to parse point: {s}")));
        };
        let x = caps["x"]
            .parse()
            .map_err(|_| InputParse(format!("Failed to parse x for point: {s}")))?;
        let y = caps["y"]
            .parse()
            .map_err(|_| InputParse(format!("Failed to parse y for point: {s}")))?;
        let z = caps["z"]
            .parse()
            .map_err(|_| InputParse(format!("Failed to parse z for point: {s}")))?;

        Ok(Point3D { x, y, z: z })
    }
}
