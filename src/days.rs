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
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
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
        17 => Some(day_17::run),
        18 => Some(day_18::run),
        19 => Some(day_19::run),
        20 => Some(day_20::run),
        21 => Some(day_21::run),
        22 => Some(day_22::run),
        23 => Some(day_23::run),
        24 => Some(day_24::run),
        _ => None,
    }
}
