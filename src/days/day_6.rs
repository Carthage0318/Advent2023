use crate::AdventErr::InputParse;
use crate::{utils, AdventResult};
use std::fs::File;
use std::io::Read;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let races = parse_input(&mut input_file)?;

    // Part 1
    utils::part_header(1);
    part_1(&races);

    Ok(())
}

fn part_1(races: &[Race]) {
    let record_beating_product: u32 = races.iter().map(|race| race.winning_options()).product();

    println!("Product of ways to win: {record_beating_product}");
}

fn compute_distance(hold_time: u32, total_race_time: u32) -> u32 {
    debug_assert!(hold_time <= total_race_time);
    hold_time * (total_race_time - hold_time)
}

#[derive(Debug, Copy, Clone)]
struct Race {
    time: u32,
    record_distance: u32,
}

impl Race {
    fn min_win_hold_time(&self) -> Option<u32> {
        let time = self.time as f64;
        let distance = self.record_distance as f64;

        let min_hold_time = ((time - (time * time - 4_f64 * distance).sqrt()) / 2f64).ceil();
        if min_hold_time.is_nan() {
            return None;
        }

        let mut min_hold_time = min_hold_time as u32;

        // In the case where the root is actually an integer, we want to exclude the root itself.
        // Check just this one case.
        if compute_distance(min_hold_time, self.time) <= self.record_distance {
            min_hold_time += 1;
            if min_hold_time > self.time / 2 {
                return None;
            }
        }

        Some(min_hold_time)
    }

    fn winning_options(&self) -> u32 {
        match self.min_win_hold_time() {
            None => 0,
            Some(min_time) => self.time - 2 * min_time + 1,
        }
    }
}

fn parse_input(input_file: &mut File) -> AdventResult<Vec<Race>> {
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let mut iter = input.lines();

    let (Some(time_line), Some(distance_line)) = (iter.next(), iter.next()) else {
        return Err(InputParse(String::from("Failed to find 2 lines in input")));
    };

    let races = time_line
        .split_whitespace()
        .skip(1)
        .zip(distance_line.split_whitespace().skip(1))
        .map(|(time, distance)| {
            Ok(Race {
                time: time
                    .parse()
                    .map_err(|_| InputParse(format!("Failed to parse time '{time}")))?,
                record_distance: distance
                    .parse()
                    .map_err(|_| InputParse(format!("Failed to parse distance '{distance}'")))?,
            })
        })
        .collect::<AdventResult<_>>()?;

    Ok(races)
}
