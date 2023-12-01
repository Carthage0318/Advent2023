use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{self, Write};

pub mod days;
pub mod utils;

type AdventResult<T> = Result<T, AdventErr>;

pub enum AdventErr {
    Io(io::Error),
}

impl Display for AdventErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use AdventErr as AE;
        match self {
            AE::Io(e) => e.fmt(f),
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
            PDE::Io(e) => e.fmt(f),
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

pub fn get_day_fn(day: u8) -> Option<fn(File) -> AdventResult<()>> {
    match day {
        _ => None,
    }
}
