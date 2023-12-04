use crate::AdventResult;
use std::fs::File;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

pub fn get_day_fn(day: u8) -> Option<fn(File) -> AdventResult<()>> {
    match day {
        1 => Some(day_1::run),
        2 => Some(day_2::run),
        3 => Some(day_3::run),
        4 => Some(day_4::run),
        _ => None,
    }
}
