use crate::AdventErr::InputParse;
use crate::{utils, AdventResult};
use std::fs::File;
use std::io::Read;

pub fn run(mut input_file: File) -> AdventResult<()> {
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    // Part 1
    let races = parse_multiple_races(&input)?;
    utils::part_header(1);
    part_1(&races);

    // Part 2
    let race = parse_single_race(&input)?;
    utils::part_header(2);
    part_2(race);

    Ok(())
}

fn part_1(races: &[Race]) {
    let record_beating_product: u64 = races.iter().map(|race| race.winning_options()).product();

    println!("Product of ways to beat records: {record_beating_product}");
}

fn part_2(race: Race) {
    let ways_to_beat_record = race.winning_options();

    println!("Ways to beat record: {ways_to_beat_record}");
}

fn compute_distance(hold_time: u64, total_race_time: u64) -> u64 {
    debug_assert!(hold_time <= total_race_time);
    hold_time * (total_race_time - hold_time)
}

#[derive(Debug, Copy, Clone)]
struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn min_win_hold_time(&self) -> Option<u64> {
        let time = self.time as f64;
        let distance = self.record_distance as f64;

        let min_hold_time = ((time - (time * time - 4_f64 * distance).sqrt()) / 2f64).ceil();
        if min_hold_time.is_nan() {
            return None;
        }

        let mut min_hold_time = min_hold_time as u64;

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

    fn winning_options(&self) -> u64 {
        match self.min_win_hold_time() {
            None => 0,
            Some(min_time) => self.time - 2 * min_time + 1,
        }
    }
}

fn parse_multiple_races(input: &str) -> AdventResult<Vec<Race>> {
    let (time_line, distance_line) = extract_lines(input)?;

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

fn parse_single_race(input: &str) -> AdventResult<Race> {
    let (time_line, distance_line) = extract_lines(input)?;

    let Some((_, time)) = time_line.split_once(':') else {
        return Err(InputParse(String::from("Failed to remove Time header")));
    };
    let time = broken_str_to_u64(time);

    let Some((_, distance)) = distance_line.split_once(':') else {
        return Err(InputParse(String::from("Failed to remove Distance header")));
    };
    let record_distance = broken_str_to_u64(distance);

    Ok(Race {
        time,
        record_distance,
    })
}

fn extract_lines(input: &str) -> AdventResult<(&str, &str)> {
    let mut iter = input.lines();

    let (Some(time_line), Some(distance_line)) = (iter.next(), iter.next()) else {
        return Err(InputParse(String::from("Failed to find 2 lines in input")));
    };

    Ok((time_line, distance_line))
}

fn broken_str_to_u64(s: &str) -> u64 {
    s.chars()
        .filter(|c| c.is_digit(10))
        .fold(0, |acc, x| acc * 10 + x.to_digit(10).unwrap() as u64)
}
