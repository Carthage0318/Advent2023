pub use days::get_day_fn;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{self, Write};

mod data_structures;
mod days;
mod math;
mod parser;
pub mod utils;

type AdventResult<T> = Result<T, AdventErr>;

pub enum AdventErr {
    Io(io::Error),
    InputParse(String),
    Compute(String),
}

impl Display for AdventErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use AdventErr as AE;
        match self {
            AE::Io(e) => Display::fmt(e, f),
            AE::InputParse(s) => write!(f, "Input Parse Error:\n{s}"),
            AE::Compute(s) => write!(f, "Compute Error:\n{s}"),
        }
    }
}

impl Debug for AdventErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use AdventErr as AE;
        match self {
            AE::Io(e) => Debug::fmt(e, f),
            e => Display::fmt(e, f),
        }
    }
}

impl From<io::Error> for AdventErr {
    fn from(value: io::Error) -> Self {
        AdventErr::Io(value)
    }
}

pub enum PromptDayErr {
    Io(io::Error),
    ParseDay(String),
}

impl Display for PromptDayErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PromptDayErr as PDE;
        match self {
            PDE::Io(e) => Display::fmt(e, f),
            PDE::ParseDay(s) => write!(f, "Could not parse '{s}' to day number"),
        }
    }
}

impl From<io::Error> for PromptDayErr {
    fn from(value: io::Error) -> Self {
        PromptDayErr::Io(value)
    }
}

pub fn prompt_day_number() -> Result<u8, PromptDayErr> {
    print!("Day: ");
    io::stdout().flush()?;
    let mut day = String::new();
    io::stdin().read_line(&mut day)?;

    let day = day.trim();

    match day.parse() {
        Ok(day) => Ok(day),
        Err(_) => Err(PromptDayErr::ParseDay(day.to_string())),
    }
}

pub fn is_valid_day(day: u8) -> bool {
    day >= 1 && day <= 25
}

pub fn get_input_file(day: u8) -> Result<File, io::Error> {
    let path = format!("input/input_{day}.txt");
    File::open(path)
}
