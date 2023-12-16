use crate::AdventResult;
use std::fs::File;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

pub fn get_day_fn(day: u8) -> Option<fn(File) -> AdventResult<()>> {
    match day {
        1 => Some(day_1::run),
        2 => Some(day_2::run),
        3 => Some(day_3::run),
        4 => Some(day_4::run),
        5 => Some(day_5::run),
        6 => Some(day_6::run),
        7 => Some(day_7::run),
        8 => Some(day_8::run),
        9 => Some(day_9::run),
        10 => Some(day_10::run),
        11 => Some(day_11::run),
        12 => Some(day_12::run),
        13 => Some(day_13::run),
        14 => Some(day_14::run),
        15 => Some(day_15::run),
        16 => Some(day_16::run),
        _ => None,
    }
}
