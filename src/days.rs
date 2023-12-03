use crate::AdventResult;
use std::fs::File;

pub mod day_1;
pub mod day_2;

pub fn get_day_fn(day: u8) -> Option<fn(File) -> AdventResult<()>> {
    match day {
        1 => Some(day_1::run),
        2 => Some(day_2::run),
        _ => None,
    }
}
