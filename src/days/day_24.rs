use crate::AdventErr::{Compute, InputParse};
use crate::{parser, utils, AdventErr, AdventResult};
use lazy_static::lazy_static;
use regex::Regex;
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

#[derive(Debug, Copy, Clone)]
struct Point3D {
    x: i64,
    y: i64,
    _z: i64,
}

#[derive(Debug, Copy, Clone)]
struct SlopeIntercept {
    slope: f64,
    intercept: f64,
}

const EPSILON: f64 = 1e-5;

impl SlopeIntercept {
    /// Gives (x, y) position of intersection
    fn intersection(self, other: Self) -> Option<(f64, f64)> {
        if (self.slope - other.slope).abs() < EPSILON {
            return if (self.intercept - other.intercept).abs() < EPSILON {
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

        Ok(Point3D { x, y, _z: z })
    }
}
